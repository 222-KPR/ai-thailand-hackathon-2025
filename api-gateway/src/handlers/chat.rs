use crate::{ApiResponse, AppResult, AppState};
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

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
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> AppResult<Json<ApiResponse<ChatResponse>>> {
    info!("Received chat message: {}", request.message);
    
    // For hackathon demo, provide mock responses instead of calling external services
    let response_text = match request.message.to_lowercase().as_str() {
        msg if msg.contains("hello") || msg.contains("hi") => {
            "Hello! I'm the AI4Thai Crop Guardian. I can help you with crop disease detection and pest identification. Please upload an image of your crop for analysis."
        },
        msg if msg.contains("disease") || msg.contains("sick") => {
            "I can help you identify crop diseases. Please upload a clear image of the affected plant, and I'll analyze it for common diseases and provide treatment recommendations."
        },
        msg if msg.contains("pest") || msg.contains("insect") => {
            "I can identify pests and insects that might be harming your crops. Upload an image of the pest or the damage it's causing, and I'll help you identify it and suggest control methods."
        },
        msg if msg.contains("upload") || msg.contains("image") => {
            "Great! You can upload an image using the vision analysis endpoint at /api/v1/vision/analyze. I'll analyze it for both diseases and pests."
        },
        msg if msg.contains("help") => {
            "I'm here to help with crop health! I can:\n- Detect plant diseases\n- Identify pests and insects\n- Provide treatment recommendations\n- Answer questions about crop care\n\nJust upload an image or ask me anything about your crops!"
        },
        _ => {
            "I'm the AI4Thai Crop Guardian, your AI assistant for crop health. I can help you identify diseases, pests, and provide treatment recommendations. Please upload an image of your crop or ask me specific questions about plant health."
        }
    };

    let conversation_id = request
        .conversation_id
        .unwrap_or_else(|| format!("conv_{}", chrono::Utc::now().timestamp()));

    // Store conversation in Redis for demo purposes
    if let Ok(mut conn) = state.get_redis().await {
        let _: Result<(), _> = redis::cmd("HSET")
            .arg("chat_history")
            .arg(&conversation_id)
            .arg(serde_json::json!({
                "message": request.message,
                "response": response_text,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
            .query_async(&mut conn)
            .await;
    }

    let response = ChatResponse {
        response: response_text.to_string(),
        conversation_id,
    };

    info!("Sending chat response for conversation: {}", response.conversation_id);
    Ok(Json(ApiResponse::success(response)))
}

/// Get conversation history endpoint
pub async fn get_conversation(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    info!("Retrieving conversation history");
    
    let mut messages = Vec::new();
    
    // Try to get conversation history from Redis
    if let Ok(mut conn) = state.get_redis().await {
        if let Ok(conversations) = redis::cmd("HGETALL")
            .arg("chat_history")
            .query_async::<_, Vec<(String, String)>>(&mut conn)
            .await
        {
            for (conv_id, conv_data) in conversations {
                if let Ok(conv_json) = serde_json::from_str::<serde_json::Value>(&conv_data) {
                    messages.push(serde_json::json!({
                        "conversation_id": conv_id,
                        "data": conv_json
                    }));
                }
            }
        }
    }

    // If no history found, return demo data
    if messages.is_empty() {
        messages.push(serde_json::json!({
            "conversation_id": "demo_conv_1",
            "data": {
                "message": "Hello, can you help me with my crops?",
                "response": "Hello! I'm the AI4Thai Crop Guardian. I can help you with crop disease detection and pest identification. Please upload an image of your crop for analysis.",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));
    }

    let response = serde_json::json!({
        "messages": messages,
        "total_conversations": messages.len(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    info!("Retrieved {} conversations", messages.len());
    Ok(Json(ApiResponse::success(response)))
}
