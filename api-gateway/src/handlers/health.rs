use crate::{AppState, AppResult, ApiResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
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

/// Readiness check endpoint - verifies all dependencies are ready
pub async fn readiness_check(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let start_time = Instant::now();
    let mut checks = Vec::new();
    let mut all_healthy = true;

    // Check database connection
    let db_start = Instant::now();
    let db_healthy = match sqlx::query("SELECT 1").fetch_one(&state.db_pool).await {
        Ok(_) => true,
        Err(_) => {
            all_healthy = false;
            false
        }
    };
    checks.push(json!({
        "name": "database",
        "status": if db_healthy { "healthy" } else { "unhealthy" },
        "response_time_ms": db_start.elapsed().as_millis()
    }));

    // Check Redis connection
    let redis_start = Instant::now();
    let redis_healthy = match state.get_redis().await {
        Ok(mut conn) => {
            match redis::cmd("PING").query_async::<_, String>(&mut conn).await {
                Ok(_) => true,
                Err(_) => {
                    all_healthy = false;
                    false
                }
            }
        }
        Err(_) => {
            all_healthy = false;
            false
        }
    };
    checks.push(json!({
        "name": "redis",
        "status": if redis_healthy { "healthy" } else { "unhealthy" },
        "response_time_ms": redis_start.elapsed().as_millis()
    }));

    // Check vision service health
    let vision_start = Instant::now();
    let vision_healthy = state.service_registry.check_vision_health().await;
    checks.push(json!({
        "name": "vision_service",
        "status": if vision_healthy { "healthy" } else { "unhealthy" },
        "response_time_ms": vision_start.elapsed().as_millis()
    }));

    // Check LLM service health
    let llm_start = Instant::now();
    let llm_healthy = state.service_registry.check_llm_health().await;
    checks.push(json!({
        "name": "llm_service",
        "status": if llm_healthy { "healthy" } else { "unhealthy" },
        "response_time_ms": llm_start.elapsed().as_millis()
    }));

    let response_data = json!({
        "status": if all_healthy { "ready" } else { "not_ready" },
        "service": "ai4thai-api-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_check_time_ms": start_time.elapsed().as_millis(),
        "checks": checks
    });

    if all_healthy {
        Ok(Json(ApiResponse::success(response_data)))
    } else {
        Err(crate::AppError::ServiceUnavailable("One or more dependencies are unhealthy".to_string()))
    }
}

/// Metrics endpoint - returns service metrics
pub async fn metrics(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    // Get basic metrics from Redis (if available)
    let mut metrics = json!({
        "service": "ai4thai-api-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime_seconds": get_uptime_seconds()
    });

    // Try to get cached metrics from Redis
    if let Ok(mut redis_conn) = state.get_redis().await {
        // Get request counts
        if let Ok(request_count) = redis::cmd("GET")
            .arg("metrics:requests:total")
            .query_async::<_, Option<u64>>(&mut redis_conn)
            .await
        {
            metrics["requests_total"] = json!(request_count.unwrap_or(0));
        }

        // Get error counts
        if let Ok(error_count) = redis::cmd("GET")
            .arg("metrics:errors:total")
            .query_async::<_, Option<u64>>(&mut redis_conn)
            .await
        {
            metrics["errors_total"] = json!(error_count.unwrap_or(0));
        }

        // Get service health status
        let vision_health = state.service_registry.get_cached_health("vision").await;
        let llm_health = state.service_registry.get_cached_health("llm").await;
        
        metrics["services"] = json!({
            "vision_service": vision_health,
            "llm_service": llm_health
        });
    }

    Ok(Json(ApiResponse::success(metrics)))
}

/// Vision service health check (proxy endpoint)
pub async fn vision_service_health(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let health_status = state.service_registry.check_vision_health().await;
    let detailed_health = state.service_registry.get_vision_health_details().await?;
    
    Ok(Json(ApiResponse::success(json!({
        "service": "vision-service",
        "healthy": health_status,
        "details": detailed_health,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))))
}

/// LLM service health check (proxy endpoint)
pub async fn llm_service_health(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let health_status = state.service_registry.check_llm_health().await;
    let detailed_health = state.service_registry.get_llm_health_details().await?;
    
    Ok(Json(ApiResponse::success(json!({
        "service": "llm-service",
        "healthy": health_status,
        "details": detailed_health,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))))
}

/// Get uptime in seconds since service start
fn get_uptime_seconds() -> u64 {
    use std::sync::OnceLock;
    static START_TIME: OnceLock<Instant> = OnceLock::new();
    
    let start = START_TIME.get_or_init(|| Instant::now());
    start.elapsed().as_secs()
}