use api_gateway::{config::AppConfig, services::ServiceRegistry, AppState};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Initialize Redis client
    let redis_client = redis::Client::open(config.redis.url.as_str())?;
    info!("Redis client initialized");

    // Initialize service registry
    let service_registry = ServiceRegistry::new(
        config.services.vision_service_url(),
        config.services.llm_service_url(),
    );
    info!("Service registry initialized");

    // Create application state
    let app_state = AppState::new(config.clone(), redis_client, service_registry);

    // Build the application router
    let app = create_router(app_state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check endpoints
        .route("/health", get(api_gateway::handlers::health_check))
        .route("/health/ready", get(api_gateway::handlers::readiness_check))
        .route("/health/metrics", get(api_gateway::handlers::metrics))
        // Chat endpoints
        .route("/api/v1/chat", post(api_gateway::handlers::send_message))
        .route(
            "/api/v1/chat/history",
            get(api_gateway::handlers::get_conversation),
        )
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}

/// Get uptime in seconds since service start
#[allow(dead_code)]
fn get_uptime_seconds() -> u64 {
    use std::sync::OnceLock;
    use std::time::Instant;

    static START_TIME: OnceLock<Instant> = OnceLock::new();
    let start = START_TIME.get_or_init(Instant::now);
    start.elapsed().as_secs()
}
