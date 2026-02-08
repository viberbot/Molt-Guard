use molt_config::{create_app, ollama_client::OllamaClient};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Check and pull required models
    let ollama_url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://192.168.68.68:11434".to_string());
    
    let client = OllamaClient::new(&ollama_url);
    
    // We want to ensure 'llama3.1:latest' exists as it's our default chat model
    // And 'prompt-guard:latest' if we were using it for validation (though we are using a placeholder logic currently)
    if let Err(e) = client.ensure_model_exists("llama3.1:latest").await {
        eprintln!("Warning: Failed to ensure model 'llama3.1:latest' exists: {}", e);
    }

    // Define the app routes
    let app = create_app();

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    println!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
