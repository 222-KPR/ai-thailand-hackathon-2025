use uuid::Uuid;
use chrono::Utc;
use shared::{
    VisionRequest, VisionResponse, LLMRequest, LLMResponse, 
    CropType, Language, ImageMetadata, ChatContext, ChatMessage,
    ServiceResponse, DiseaseSeverity
};
use gloo_net::http::Request;
use serde_json::json;

const API_BASE_URL: &str = "http://localhost:3000/api";

pub struct ApiService;

impl ApiService {
    pub async fn analyze_image(
        image_data: &str,
        crop_type: CropType,
    ) -> Result<VisionResponse, String> {
        let request = VisionRequest {
            request_id: Uuid::new_v4(),
            image_data: image_data.to_string(),
            crop_type,
            metadata: ImageMetadata {
                size_bytes: 0, // Will be calculated by backend
                width: 0,
                height: 0,
                format: "jpeg".to_string(),
            },
            timestamp: Utc::now(),
        };
        
        let response = Request::post(&format!("{}/vision/analyze", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
        
        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }
        
        let service_response: ServiceResponse<VisionResponse> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        if service_response.success {
            service_response.data.ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(service_response.error
                .map(|e| e.error_message)
                .unwrap_or_else(|| "Unknown error".to_string()))
        }
    }
    
    pub async fn get_treatment_advice(
        vision_result: &VisionResponse,
    ) -> Result<LLMResponse, String> {
        let request = LLMRequest {
            request_id: Uuid::new_v4(),
            disease: vision_result.disease.clone(),
            crop_type: CropType::Rice, // This should come from the original request
            severity: vision_result.severity.clone(),
            location: None, // Could be added from user profile
            language: Language::Thai,
            context: ChatContext {
                conversation_id: Uuid::new_v4(),
                previous_messages: Vec::new(),
                user_profile: None,
                weather_context: None,
            },
            timestamp: Utc::now(),
        };
        
        let response = Request::post(&format!("{}/llm/advice", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
        
        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }
        
        let service_response: ServiceResponse<LLMResponse> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        if service_response.success {
            service_response.data.ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(service_response.error
                .map(|e| e.error_message)
                .unwrap_or_else(|| "Unknown error".to_string()))
        }
    }
    
    pub async fn send_chat_message(
        message: &str,
        conversation_id: &Uuid,
    ) -> Result<String, String> {
        let payload = json!({
            "message": message,
            "conversation_id": conversation_id,
            "language": "Thai"
        });
        
        let response = Request::post(&format!("{}/chat/message", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&payload)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
        
        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }
        
        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        response_data
            .get("response")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "No response message found".to_string())
    }
    
    pub async fn check_health() -> Result<(), String> {
        let response = Request::get(&format!("{}/health", API_BASE_URL))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
        
        if response.ok() {
            Ok(())
        } else {
            Err(format!("Health check failed: {}", response.status()))
        }
    }
}