use api_gateway::config::AppConfig;
use api_gateway::handlers;
use api_gateway::middleware;
use api_gateway::services::ServiceRegistry;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let config = AppConfig::load()?;
    info!("Starting AI4Thai API Gateway v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration loaded: {:?}", config);

    // Initialize database connection pool
    let db_pool = config.database.create_pool().await?;
    info!("Database connection pool initialized");

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    info!("Database migrations completed");

    // Initialize Redis connection
    let redis_client = config.redis.create_client().await?;
    info!("Redis connection initialized");

    // Initialize service registry
    let service_registry = ServiceRegistry::new(
        config.services.clone(),
        config.external_apis.clone(),
        redis_client.clone(),
    ).await?;
    info!("Service registry initialized");

    // Create application state
    let app_state = api_gateway::AppState {
        config: config.clone(),
        db_pool,
        redis_client,
        service_registry,
    };

    // Build application with middleware stack
    let app = create_app(app_state).await;

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
async fn create_app(state: api_gateway::AppState) -> Router {
    // Build middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // Configure CORS properly for production
        .layer(middleware::auth::AuthLayer::new(state.config.auth.jwt_secret.clone()))
        .layer(middleware::rate_limit::RateLimitLayer::new(state.redis_client.clone()))
        .layer(middleware::circuit_breaker::CircuitBreakerLayer::new());

    Router::new()
        // Health check endpoints
        .route("/health", get(handlers::health::health_check))
        .route("/ready", get(handlers::health::readiness_check))
        .route("/metrics", get(handlers::health::metrics))
        
        // Authentication endpoints
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        
        // User management
        .route("/api/v1/user/profile", get(handlers::auth::get_profile))
        .route("/api/v1/user/profile", post(handlers::auth::update_profile))
        
        // Chat endpoints
        .route("/api/v1/chat", post(handlers::chat::create_chat))
        .route("/api/v1/chat/:chat_id", get(handlers::chat::get_chat))
        .route("/api/v1/chat/:chat_id/messages", get(handlers::chat::get_messages))
        .route("/api/v1/chat/:chat_id/messages", post(handlers::chat::send_message))
        
        // Diagnosis endpoints
        .route("/api/v1/diagnose", post(handlers::diagnosis::diagnose_image))
        .route("/api/v1/diagnose/:diagnosis_id", get(handlers::diagnosis::get_diagnosis))
        .route("/api/v1/diagnose/history", get(handlers::diagnosis::get_diagnosis_history))
        
        // File upload/download
        .route("/api/v1/files/upload", post(handlers::files::upload_file))
        .route("/api/v1/files/:file_id", get(handlers::files::download_file))
        
        // WebSocket for real-time chat
        .route("/ws", get(handlers::chat::websocket_handler))
        
        // Service proxy endpoints (for development/debugging)
        .route("/api/v1/services/vision/health", get(handlers::health::vision_service_health))
        .route("/api/v1/services/llm/health", get(handlers::health::llm_service_health))
        
        .layer(middleware_stack)
        .with_state(state)
}