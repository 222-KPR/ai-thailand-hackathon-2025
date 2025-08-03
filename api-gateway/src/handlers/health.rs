use crate::{ApiResponse, AppResult, AppState};
use axum::{extract::State, response::Json};
use serde_json::json;

/// Basic health check endpoint
pub async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(json!({
        "status": "healthy",
        "service": "AI4Thai API Gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds()
    })))
}

/// Readiness check endpoint
pub async fn readiness_check(State(state): State<AppState>) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let mut health_status = json!({
        "status": "healthy",
        "service": "AI4Thai API Gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds(),
        "services": {}
    });

    let services = health_status["services"].as_object_mut().unwrap();

    // Check Redis
    match state.get_redis().await {
        Ok(_) => {
            services.insert("redis".to_string(), json!({
                "status": "healthy",
                "message": "Connected successfully"
            }));
        }
        Err(e) => {
            services.insert("redis".to_string(), json!({
                "status": "unhealthy",
                "error": e.to_string()
            }));
        }
    }

    // Check RabbitMQ
    let rabbitmq_healthy = state.rabbitmq_service.health_check().await;
    services.insert("rabbitmq".to_string(), json!({
        "status": if rabbitmq_healthy { "healthy" } else { "unhealthy" },
        "connection_state": format!("{:?}", state.rabbitmq_service.connection_status()),
        "message": if rabbitmq_healthy { "Connected successfully" } else { "Connection failed" }
    }));

    // Check file storage
    match state.file_storage_service.get_stats().await {
        Ok(stats) => {
            services.insert("file_storage".to_string(), json!({
                "status": "healthy",
                "total_files": stats.total_files,
                "total_size_bytes": stats.total_size_bytes,
                "temp_dir": stats.temp_dir
            }));
        }
        Err(e) => {
            services.insert("file_storage".to_string(), json!({
                "status": "unhealthy",
                "error": e.to_string()
            }));
        }
    }

    // Check vision service
    let vision_healthy = state.service_registry.check_vision_health().await;
    services.insert("vision_service".to_string(), json!({
        "status": if vision_healthy { "healthy" } else { "unhealthy" },
        "url": state.config.services.vision_service_url()
    }));

    // Check LLM service
    let llm_healthy = state.service_registry.check_llm_health().await;
    services.insert("llm_service".to_string(), json!({
        "status": if llm_healthy { "healthy" } else { "unhealthy" },
        "url": state.config.services.llm_service_url()
    }));

    // Determine overall status
    let all_healthy = services.values().all(|service| {
        service["status"].as_str().unwrap_or("unknown") == "healthy"
    });

    if !all_healthy {
        health_status["status"] = json!("degraded");
    }

    Ok(Json(ApiResponse::success(health_status)))
}

/// Metrics endpoint
pub async fn metrics(State(state): State<AppState>) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let mut metrics = json!({
        "service": "AI4Thai API Gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds(),
        "metrics": {}
    });

    let metrics_obj = metrics["metrics"].as_object_mut().unwrap();

    // File storage metrics
    if let Ok(stats) = state.file_storage_service.get_stats().await {
        metrics_obj.insert("file_storage".to_string(), json!({
            "total_files": stats.total_files,
            "total_size_bytes": stats.total_size_bytes,
            "total_size_mb": (stats.total_size_bytes as f64 / (1024.0 * 1024.0)).round() as u64
        }));
    }

    // RabbitMQ metrics
    let rabbitmq_healthy = state.rabbitmq_service.health_check().await;
    metrics_obj.insert("rabbitmq".to_string(), json!({
        "connection_healthy": rabbitmq_healthy,
        "connection_state": format!("{:?}", state.rabbitmq_service.connection_status())
    }));

    // System metrics (basic)
    metrics_obj.insert("system".to_string(), json!({
        "uptime_seconds": get_uptime_seconds(),
        "memory_usage_mb": get_memory_usage_mb()
    }));

    Ok(Json(ApiResponse::success(metrics)))
}

/// Get uptime in seconds since service start
fn get_uptime_seconds() -> u64 {
    use std::sync::OnceLock;
    use std::time::Instant;

    static START_TIME: OnceLock<Instant> = OnceLock::new();
    let start = START_TIME.get_or_init(Instant::now);
    start.elapsed().as_secs()
}

/// Get memory usage in MB (basic implementation)
fn get_memory_usage_mb() -> u64 {
    // This is a basic implementation - in production you might want to use a proper metrics library
    // For now, we'll return a placeholder value
    0
}
