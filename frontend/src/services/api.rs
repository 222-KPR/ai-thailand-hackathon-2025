use uuid::Uuid;
use chrono::Utc;
use shared::{
    VisionRequest, VisionResponse, LLMRequest, LLMResponse, 
    CropType, Language, ImageMetadata, ChatContext,
    ServiceResponse, ServiceError
};
use gloo_net::http::Request;
use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::console;

const API_BASE_URL: &str = "http://localhost:3000/api";

/// API service error types
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    SerializationError(String),
    HttpError(u16, String),
    ServiceError(String),
    ValidationError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ApiError::HttpError(code, msg) => write!(f, "HTTP {} error: {}", code, msg),
            ApiError::ServiceError(msg) => write!(f, "Service error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl From<ApiError> for String {
    fn from(error: ApiError) -> Self {
        error.to_string()
    }
}

/// Main API service struct
pub struct ApiService;

impl ApiService {
    /// Analyze crop image for disease detection
    pub async fn analyze_image(
        image_data: &str,
        crop_type: CropType,
        metadata: Option<ImageMetadata>,
    ) -> Result<VisionResponse, ApiError> {
        // Validate input
        if image_data.is_empty() {
            return Err(ApiError::ValidationError("Image data cannot be empty".to_string()));
        }

        let request = VisionRequest {
            request_id: Uuid::new_v4(),
            image_data: image_data.to_string(),
            crop_type,
            metadata: metadata.unwrap_or_else(|| ImageMetadata {
                size_bytes: 0,
                width: 0,
                height: 0,
                format: "jpeg".to_string(),
            }),
            timestamp: Utc::now(),
        };
        
        console::log_1(&JsValue::from_str(&format!("Sending vision request: {:?}", request.request_id)));
        
        let response = Request::post(&format!("{}/vision/analyze", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| ApiError::SerializationError(format!("Failed to serialize request: {}", e)))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Network request failed: {}", e)))?;
        
        let status = response.status();
        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::HttpError(status, error_text));
        }
        
        let service_response: ServiceResponse<VisionResponse> = response
            .json()
            .await
            .map_err(|e| ApiError::SerializationError(format!("Failed to parse response: {}", e)))?;
        
        if service_response.success {
            service_response.data
                .ok_or_else(|| ApiError::ServiceError("No data in successful response".to_string()))
        } else {
            let error_msg = service_response.error
                .map(|e| e.error_message)
                .unwrap_or_else(|| "Unknown service error".to_string());
            Err(ApiError::ServiceError(error_msg))
        }
    }
    
    /// Get treatment advice based on vision analysis results
    pub async fn get_treatment_advice(
        vision_result: &VisionResponse,
        crop_type: CropType,
        language: Language,
        location: Option<String>,
    ) -> Result<LLMResponse, ApiError> {
        let request = LLMRequest {
            request_id: Uuid::new_v4(),
            disease: vision_result.disease.clone(),
            crop_type,
            severity: vision_result.severity.clone(),
            location,
            language,
            context: ChatContext {
                conversation_id: Uuid::new_v4(),
                previous_messages: Vec::new(),
                user_profile: None,
                weather_context: None,
            },
            timestamp: Utc::now(),
        };
        
        console::log_1(&JsValue::from_str(&format!("Sending LLM request: {:?}", request.request_id)));
        
        let response = Request::post(&format!("{}/llm/advice", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| ApiError::SerializationError(format!("Failed to serialize request: {}", e)))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Network request failed: {}", e)))?;
        
        let status = response.status();
        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::HttpError(status, error_text));
        }
        
        let service_response: ServiceResponse<LLMResponse> = response
            .json()
            .await
            .map_err(|e| ApiError::SerializationError(format!("Failed to parse response: {}", e)))?;
        
        if service_response.success {
            service_response.data
                .ok_or_else(|| ApiError::ServiceError("No data in successful response".to_string()))
        } else {
            let error_msg = service_response.error
                .map(|e| e.error_message)
                .unwrap_or_else(|| "Unknown service error".to_string());
            Err(ApiError::ServiceError(error_msg))
        }
    }
    
    /// Send chat message and get AI response
    pub async fn send_chat_message(
        message: &str,
        conversation_id: &Uuid,
        language: Language,
    ) -> Result<String, ApiError> {
        // Validate input
        if message.trim().is_empty() {
            return Err(ApiError::ValidationError("Message cannot be empty".to_string()));
        }

        if message.len() > 1000 {
            return Err(ApiError::ValidationError("Message too long (max 1000 characters)".to_string()));
        }

        let payload = json!({
            "message": message.trim(),
            "conversation_id": conversation_id,
            "language": language,
            "timestamp": Utc::now()
        });
        
        console::log_1(&JsValue::from_str(&format!("Sending chat message to conversation: {}", conversation_id)));
        
        let response = Request::post(&format!("{}/chat/message", API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&payload)
            .map_err(|e| ApiError::SerializationError(format!("Failed to serialize request: {}", e)))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Network request failed: {}", e)))?;
        
        let status = response.status();
        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::HttpError(status, error_text));
        }
        
        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::SerializationError(format!("Failed to parse response: {}", e)))?;
        
        // Extract response message
        response_data
            .get("data")
            .and_then(|data| data.get("response"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                // Fallback to direct response field
                response_data
                    .get("response")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .ok_or_else(|| ApiError::ServiceError("No response message found in API response".to_string()))
    }
    
    /// Check API service health
    pub async fn check_health() -> Result<(), ApiError> {
        console::log_1(&JsValue::from_str("Checking API health"));
        
        let response = Request::get(&format!("{}/health", API_BASE_URL))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Health check network error: {}", e)))?;
        
        if response.ok() {
            console::log_1(&JsValue::from_str("API health check passed"));
            Ok(())
        } else {
            let error_msg = format!("Health check failed with status: {}", response.status());
            console::error_1(&JsValue::from_str(&error_msg));
            Err(ApiError::HttpError(response.status(), error_msg))
        }
    }

    /// Get conversation history
    pub async fn get_conversation_history(
        conversation_id: &Uuid,
        limit: Option<u32>,
    ) -> Result<Vec<shared::ChatMessage>, ApiError> {
        let mut url = format!("{}/chat/conversations/{}/messages", API_BASE_URL, conversation_id);
        
        if let Some(limit) = limit {
            url.push_str(&format!("?limit={}", limit));
        }
        
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Network request failed: {}", e)))?;
        
        let status = response.status();
        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::HttpError(status, error_text));
        }
        
        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::SerializationError(format!("Failed to parse response: {}", e)))?;
        
        // Extract messages from response
        response_data
            .get("data")
            .and_then(|data| data.get("messages"))
            .and_then(|messages| serde_json::from_value(messages.clone()).ok())
            .ok_or_else(|| ApiError::ServiceError("Failed to parse conversation history".to_string()))
    }

    /// Upload image and get base64 data
    pub async fn process_image_file(file: web_sys::File) -> Result<(String, ImageMetadata), ApiError> {
        use wasm_bindgen_futures::JsFuture;
        use js_sys::Uint8Array;
        
        // Validate file type
        let file_type = file.type_();
        if !file_type.starts_with("image/") {
            return Err(ApiError::ValidationError("File must be an image".to_string()));
        }
        
        // Validate file size (10MB limit)
        const MAX_SIZE: f64 = 10.0 * 1024.0 * 1024.0; // 10MB
        if file.size() > MAX_SIZE {
            return Err(ApiError::ValidationError("Image file too large (max 10MB)".to_string()));
        }
        
        // Read file as array buffer
        let file_reader = web_sys::FileReader::new()
            .map_err(|_| ApiError::ServiceError("Failed to create FileReader".to_string()))?;
        
        file_reader.read_as_array_buffer(&file)
            .map_err(|_| ApiError::ServiceError("Failed to read file".to_string()))?;
        
        let load_promise = js_sys::Promise::new(&mut |resolve, reject| {
            let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                resolve.call0(&JsValue::NULL).unwrap();
            }) as Box<dyn FnMut(_)>);
            
            let onerror = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                reject.call1(&JsValue::NULL, &JsValue::from_str("Failed to read file")).unwrap();
            }) as Box<dyn FnMut(_)>);
            
            file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            
            onload.forget();
            onerror.forget();
        });
        
        JsFuture::from(load_promise).await
            .map_err(|_| ApiError::ServiceError("Failed to read file".to_string()))?;
        
        // Get the result and convert to base64
        let array_buffer = file_reader.result()
            .map_err(|_| ApiError::ServiceError("Failed to get file result".to_string()))?;
        
        let uint8_array = Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();
        let base64_data = base64::encode(&bytes);
        
        // Create metadata
        let metadata = ImageMetadata {
            size_bytes: file.size() as u64,
            width: 0, // Will be determined by backend
            height: 0, // Will be determined by backend
            format: file_type.split('/').nth(1).unwrap_or("jpeg").to_string(),
        };
        
        Ok((base64_data, metadata))
    }
}

// Helper function for base64 encoding
mod base64 {
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;
    
    pub fn encode(bytes: &[u8]) -> String {
        let uint8_array = Uint8Array::from(bytes);
        let js_string = js_sys::JSON::stringify(&uint8_array.into())
            .unwrap_or_else(|| JsValue::from_str(""));
        js_string.as_string().unwrap_or_default()
    }
}
