use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

/// Convert AppError to HTTP response
pub fn error_response(status: StatusCode, message: &str) -> impl IntoResponse {
    let body = json!({
        "success": false,
        "error": {
            "message": message,
            "code": status.as_u16()
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    (status, Json(body))
}
