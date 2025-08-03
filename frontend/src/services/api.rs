use crate::types::{
    ApiError, ChatMessage, CropType, HealthResponse, JobResponse, JobStatus, Language,
    PestDetectionResult, TreatmentStep, VisionRequest, VisionResponse,
};
use chrono::Utc;
use gloo_net::http::Request;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use web_sys::{FormData, File};

pub struct ApiService {
    pub vision_service_url: String,
    pub queue_service_url: String,
}

impl Default for ApiService {
    fn default() -> Self {
        Self {
            vision_service_url: "http://localhost:2001".to_string(),
            queue_service_url: "http://localhost:8001".to_string(),
        }
    }
}

impl ApiService {
    pub fn new() -> Self {
        // Get URLs from environment or use defaults
        let vision_service_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .map(|origin| format!("{}:2001", origin.replace(":8080", "")))
            .unwrap_or_else(|| "http://localhost:2001".to_string());

        let queue_service_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .map(|origin| format!("{}:8001", origin.replace(":8080", "")))
            .unwrap_or_else(|| "http://localhost:8001".to_string());

        Self {
            vision_service_url,
            queue_service_url,
        }
    }

    // Vision Service - Disease Detection
    pub async fn analyze_disease(
        &self,
        image_data: String,
        crop_type: CropType,
        confidence_threshold: f64,
    ) -> Result<VisionResponse, ApiError> {
        let request_data = VisionRequest {
            image_data,
            crop_type: crop_type.to_string(),
            confidence_threshold,
        };

        let response = Request::post(&format!("{}/analyze", self.vision_service_url))
            .header("Content-Type", "application/json")
            .json(&request_data)
            .map_err(|e| ApiError::NetworkError(format!("Request creation failed: {:?}", e)))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<VisionResponse>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )))
        }
    }

    // Queue Worker - Pest Detection
    pub async fn submit_pest_detection(
        &self,
        image_file: File,
        crop_type: CropType,
        description: Option<String>,
    ) -> Result<JobResponse, ApiError> {
        let form_data = FormData::new().map_err(|_| ApiError::NetworkError("Failed to create FormData".to_string()))?;

        // Add form fields
        form_data
            .append_with_blob("image", &image_file)
            .map_err(|_| ApiError::NetworkError("Failed to append image".to_string()))?;

        form_data
            .append_with_str("crop_type", &crop_type.to_string())
            .map_err(|_| ApiError::NetworkError("Failed to append crop_type".to_string()))?;

        if let Some(desc) = description {
            form_data
                .append_with_str("description", &desc)
                .map_err(|_| ApiError::NetworkError("Failed to append description".to_string()))?;
        }

        let response = Request::post(&format!("{}/api/v1/queue/pest-detection", self.queue_service_url))
            .body(JsValue::from(form_data))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<JobResponse>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )))
        }
    }

    // Queue Worker - Disease Detection
    pub async fn submit_disease_detection(
        &self,
        image_file: File,
        crop_type: CropType,
        description: Option<String>,
    ) -> Result<JobResponse, ApiError> {
        let form_data = FormData::new().map_err(|_| ApiError::NetworkError("Failed to create FormData".to_string()))?;

        // Add form fields
        form_data
            .append_with_blob("image", &image_file)
            .map_err(|_| ApiError::NetworkError("Failed to append image".to_string()))?;

        form_data
            .append_with_str("crop_type", &crop_type.to_string())
            .map_err(|_| ApiError::NetworkError("Failed to append crop_type".to_string()))?;

        if let Some(desc) = description {
            form_data
                .append_with_str("description", &desc)
                .map_err(|_| ApiError::NetworkError("Failed to append description".to_string()))?;
        }

        let response = Request::post(&format!("{}/api/v1/queue/disease-detection", self.queue_service_url))
            .body(JsValue::from(form_data))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<JobResponse>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )))
        }
    }

    // Get job status
    pub async fn get_job_status(&self, job_id: &str) -> Result<JobStatus, ApiError> {
        let response = Request::get(&format!("{}/api/v1/jobs/{}", self.queue_service_url, job_id))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<JobStatus>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::NetworkError(format!(
                "HTTP error: {}",
                response.status()
            )))
        }
    }

    // Health checks
    pub async fn check_vision_health(&self) -> Result<HealthResponse, ApiError> {
        let response = Request::get(&format!("{}/health", self.vision_service_url))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<HealthResponse>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::ServiceUnavailable)
        }
    }

    pub async fn check_queue_health(&self) -> Result<HealthResponse, ApiError> {
        let response = Request::get(&format!("{}/health", self.queue_service_url))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Request failed: {:?}", e)))?;

        if response.ok() {
            response
                .json::<HealthResponse>()
                .await
                .map_err(|e| ApiError::ParseError(format!("JSON parse error: {:?}", e)))
        } else {
            Err(ApiError::ServiceUnavailable)
        }
    }

    // Mock chat functionality (can be extended to connect to LLM service)
    pub async fn send_chat_message(
        &self,
        message: String,
        _conversation_id: Uuid,
        language: Language,
    ) -> Result<ChatMessage, ApiError> {
        // Mock response - in real implementation, this would call an LLM service
        let response_content = match language {
            Language::Thai => "นี่คือการตอบกลับจากผู้ช่วย AI ในการพัฒนาจริงจะเชื่อมต่อกับบริการ LLM".to_string(),
            Language::English => "This is a mock response from the AI assistant. In a real implementation, this would connect to the LLM service.".to_string(),
        };

        Ok(ChatMessage {
            id: Uuid::new_v4(),
            content: response_content,
            is_user: false,
            timestamp: Utc::now(),
            language,
        })
    }
}
