use crate::AppResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LlmClient {
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    base_url: String,
}

#[derive(Debug, Serialize)]
pub struct LlmRequest {
    pub message: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub response: String,
    pub confidence: f32,
}

impl LlmClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate_response(&self, request: LlmRequest) -> AppResult<LlmResponse> {
        // TODO: Implement actual LLM service call
        Ok(LlmResponse {
            response: format!("Mock response to: {}", request.message),
            confidence: 0.9,
        })
    }
}
