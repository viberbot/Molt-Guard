use molt_config::create_app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Define the app routes
    let app = create_app();

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    println!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}