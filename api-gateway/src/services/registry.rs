use crate::{AppError, AppResult};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tracing::{error, info, warn};

/// Service registry for managing external service connections
#[derive(Clone, Debug)]
pub struct ServiceRegistry {
    client: Client,
    vision_service_url: String,
    llm_service_url: String,
}

impl ServiceRegistry {
    pub fn new(vision_service_url: String, llm_service_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            vision_service_url,
            llm_service_url,
        }
    }

    /// Check if vision service is healthy
    pub async fn check_vision_health(&self) -> bool {
        let health_url = format!("{}/health", self.vision_service_url);

        match self.client.get(&health_url).send().await {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    info!("Vision service health check passed");
                } else {
                    warn!("Vision service health check failed: {}", response.status());
                }
                is_healthy
            }
            Err(e) => {
                error!("Vision service health check error: {}", e);
                false
            }
        }
    }

    /// Check if LLM service is healthy
    pub async fn check_llm_health(&self) -> bool {
        let health_url = format!("{}/health", self.llm_service_url);

        match self.client.get(&health_url).send().await {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    info!("LLM service health check passed");
                } else {
                    warn!("LLM service health check failed: {}", response.status());
                }
                is_healthy
            }
            Err(e) => {
                error!("LLM service health check error: {}", e);
                false
            }
        }
    }

    /// Get detailed vision service health information
    pub async fn get_vision_health_details(&self) -> AppResult<Value> {
        let health_url = format!("{}/health/detailed", self.vision_service_url);

        let response = self.client.get(&health_url).send().await.map_err(|e| {
            AppError::Service(ServiceError::Error(format!(
                "Vision service request failed: {e}"
            )))
        })?;

        if !response.status().is_success() {
            return Err(AppError::Service(ServiceError::Error(format!(
                "Vision service returned error: {}",
                response.status()
            ))));
        }

        let health_data = response.json::<Value>().await.map_err(|e| {
            AppError::Service(ServiceError::Error(format!(
                "Failed to parse vision service response: {e}"
            )))
        })?;

        Ok(health_data)
    }

    /// Get detailed LLM service health information
    pub async fn get_llm_health_details(&self) -> AppResult<Value> {
        let health_url = format!("{}/health/detailed", self.llm_service_url);

        let response = self.client.get(&health_url).send().await.map_err(|e| {
            AppError::Service(ServiceError::Error(format!(
                "LLM service request failed: {e}"
            )))
        })?;

        if !response.status().is_success() {
            return Err(AppError::Service(ServiceError::Error(format!(
                "LLM service returned error: {}",
                response.status()
            ))));
        }

        let health_data = response.json::<Value>().await.map_err(|e| {
            AppError::Service(ServiceError::Error(format!(
                "Failed to parse LLM service response: {e}"
            )))
        })?;

        Ok(health_data)
    }

    /// Get cached health status for a service
    pub async fn get_cached_health(&self, service: &str) -> Value {
        // For now, return a simple status. In a real implementation,
        // this would check a cache (Redis) for the last known health status
        match service {
            "vision" => serde_json::json!({
                "status": if self.check_vision_health().await { "healthy" } else { "unhealthy" },
                "last_check": chrono::Utc::now().to_rfc3339()
            }),
            "llm" => serde_json::json!({
                "status": if self.check_llm_health().await { "healthy" } else { "unhealthy" },
                "last_check": chrono::Utc::now().to_rfc3339()
            }),
            _ => serde_json::json!({
                "status": "unknown",
                "error": "Unknown service"
            }),
        }
    }
}

/// Service error type
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Service timeout: {0}")]
    Timeout(String),

    #[error("Service error: {0}")]
    Error(String),
}
