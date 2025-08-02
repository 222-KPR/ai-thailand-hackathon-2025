pub mod config;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

use config::AppConfig;
use redis::Client as RedisClient;
use services::ServiceRegistry;

/// Application state shared across all handlers
#[derive(Clone, Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub redis_client: RedisClient,
    pub service_registry: ServiceRegistry,
}

impl AppState {
    /// Create a new application state instance
    pub fn new(
        config: AppConfig,
        redis_client: RedisClient,
        service_registry: ServiceRegistry,
    ) -> Self {
        Self {
            config,
            redis_client,
            service_registry,
        }
    }

    /// Get Redis connection
    pub async fn get_redis(&self) -> Result<redis::aio::Connection, redis::RedisError> {
        self.redis_client.get_async_connection().await
    }
}

/// Application errors
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Configuration error: {0}")]
    Config(#[from] ::config::ConfigError),

    #[error("Service error: {0}")]
    Service(#[from] services::ServiceError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::{http::StatusCode, Json};
        use serde_json::json;

        let (status, error_message) = match self {
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Service(_) => (StatusCode::BAD_GATEWAY, "Service error"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::ExternalApi(_) => (StatusCode::BAD_GATEWAY, "External API error"),
            AppError::RateLimit => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
            AppError::ServiceUnavailable(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable")
            }
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "details": self.to_string(),
            "status_code": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        (status, body).into_response()
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Response wrapper for API endpoints
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message.into()),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Extract user ID from JWT token (used in authenticated routes)
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn new(user_id: uuid::Uuid, email: String, roles: Vec<String>) -> Self {
        Self {
            user_id,
            email,
            roles,
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}
