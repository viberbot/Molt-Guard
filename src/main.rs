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

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Define the app routes
    let app = Router::new()
        .route("/", get(|| async { "Molt Bot Secure Proxy" }));

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sanity() {
        assert_eq!(1 + 1, 2);
    }
}
