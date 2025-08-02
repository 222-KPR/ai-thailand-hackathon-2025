use crate::AppResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct VisionClient {
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    base_url: String,
}

#[derive(Debug, Serialize)]
pub struct VisionRequest {
    pub image_data: String,
    pub crop_type: String,
}

#[derive(Debug, Deserialize)]
pub struct VisionResponse {
    pub disease: String,
    pub confidence: f32,
    pub severity: String,
}

impl VisionClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn analyze_image(&self, _request: VisionRequest) -> AppResult<VisionResponse> {
        // TODO: Implement actual vision service call
        Ok(VisionResponse {
            disease: "Mock Disease".to_string(),
            confidence: 0.95,
            severity: "moderate".to_string(),
        })
    }
}
