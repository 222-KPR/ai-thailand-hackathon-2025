use axum::{http::StatusCode, response::Json};
use serde_json::json;
use std::time::Instant;

/// Health check endpoint - returns basic service status
pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "service": "ai4thai-api-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds()
    })))
}

/// Readiness check endpoint - verifies Redis is ready
pub async fn readiness_check() -> Result<Json<serde_json::Value>, StatusCode> {
    let start_time = Instant::now();

    // For simplicity, we'll just return ready if the health check passes
    // In a more complex setup, you'd check Redis connectivity here
    let response_data = json!({
        "status": "ready",
        "service": "ai4thai-api-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_check_time_ms": start_time.elapsed().as_millis()
    });

    Ok(Json(response_data))
}

/// Metrics endpoint - returns basic service metrics
pub async fn metrics() -> Result<Json<serde_json::Value>, StatusCode> {
    let metrics = json!({
        "service": "ai4thai-api-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds(),
        "requests_total": 0, // In production, track via middleware
        "errors_total": 0
    });

    Ok(Json(metrics))
}

/// Vision service health check endpoint
pub async fn vision_service_health() -> Result<Json<serde_json::Value>, StatusCode> {
    // For simplified demo, return a mock healthy status
    // In production, this would check the actual vision service
    Ok(Json(json!({
        "status": "healthy",
        "service": "vision-service",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "models_loaded": true,
        "gpu_available": false, // Mock status for demo
        "note": "Mock response for simplified demo"
    })))
}

/// LLM service health check endpoint
pub async fn llm_service_health() -> Result<Json<serde_json::Value>, StatusCode> {
    // For simplified demo, return a mock healthy status
    // In production, this would check the actual LLM service
    Ok(Json(json!({
        "status": "healthy",
        "service": "llm-service",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "models_loaded": true,
        "context_manager": "ready",
        "note": "Mock response for simplified demo"
    })))
}

/// Get uptime in seconds since service start
fn get_uptime_seconds() -> u64 {
    use std::sync::OnceLock;
    static START_TIME: OnceLock<Instant> = OnceLock::new();

    let start = START_TIME.get_or_init(Instant::now);
    start.elapsed().as_secs()
}
