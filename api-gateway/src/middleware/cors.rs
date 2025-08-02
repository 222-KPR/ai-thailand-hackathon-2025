use axum::{extract::Request, http::Method, middleware::Next, response::Response};
use tower_http::cors::{Any, CorsLayer};

/// CORS middleware configuration
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}

/// Custom CORS middleware
pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    // Add CORS headers if needed
    response
}
