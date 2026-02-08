mod prompt_guard;
mod middleware;
mod secrets_filter;
mod pii_filter;
mod vault;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Molt Bot Secure Proxy" }))
        .route("/health", get(|| async { "OK" }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Define the app routes
    let app = create_app();

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
