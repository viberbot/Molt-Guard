use molt_guard::{create_app, AppState, prompt_guard::{ValidationMode, Sensitivity}, ollama_client::OllamaClient};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Configuration
    let ollama_url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://ollama:11434".to_string());
    
    let validation_mode_str = std::env::var("VALIDATION_MODE").unwrap_or_else(|_| "Local".to_string());
    let validation_mode = match validation_mode_str.as_str() {
        "Remote" => ValidationMode::Remote,
        _ => ValidationMode::Local,
    };

    let sensitivity_str = std::env::var("PROMPT_SENSITIVITY").unwrap_or_else(|_| "Medium".to_string());
    let sensitivity = match sensitivity_str.as_str() {
        "Low" => Sensitivity::Low,
        "High" => Sensitivity::High,
        _ => Sensitivity::Medium,
    };

    // Run model presence check in the background to avoid blocking server startup
    let ollama_url_clone = ollama_url.clone();
    tokio::spawn(async move {
        let client = OllamaClient::new(&ollama_url_clone);
        if let Err(e) = client.ensure_model_exists("llama3.1:latest").await {
            eprintln!("Warning: Failed to ensure model 'llama3.1:latest' exists: {}", e);
        }
    });

    let state = AppState {
        ollama_url,
        validation_mode,
        sensitivity,
    };

    // Define the app routes
    let app = create_app(state);

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    println!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
