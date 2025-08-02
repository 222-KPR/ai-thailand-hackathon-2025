use crate::{ApiResponse, AppResult, AppState};
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub conversation_id: String,
}

/// Send chat message endpoint
pub async fn send_message(
    State(_state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> AppResult<Json<ApiResponse<ChatResponse>>> {
    // TODO: Implement actual chat logic
    let response = ChatResponse {
        response: format!("Mock response to: {}", request.message),
        conversation_id: request
            .conversation_id
            .unwrap_or_else(|| "mock_conv_id".to_string()),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Get conversation history endpoint
pub async fn get_conversation(
    State(_state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    // TODO: Implement conversation retrieval logic
    Ok(Json(ApiResponse::success(serde_json::json!({
        "messages": [],
        "conversation_id": "mock_conv_id"
    }))))
}
