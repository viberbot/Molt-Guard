use molt_guard::{create_app, AppState, prompt_guard::{ValidationMode, Sensitivity}, ollama_client::OllamaClient};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    println!("Molt-Guard starting up...");

    // Configuration
    let ollama_url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://ollama:11434".to_string());
    println!("Using Ollama URL: {}", ollama_url);
    
    let validation_mode_str = std::env::var("VALIDATION_MODE").unwrap_or_else(|_| "Local".to_string());
    let validation_mode = match validation_mode_str.as_str() {
        "Remote" => ValidationMode::Remote,
        _ => ValidationMode::Local,
    };
    println!("Validation Mode: {:?}", validation_mode);

    let sensitivity_str = std::env::var("PROMPT_SENSITIVITY").unwrap_or_else(|_| "Medium".to_string());
    let sensitivity = match sensitivity_str.as_str() {
        "Low" => Sensitivity::Low,
        "High" => Sensitivity::High,
        _ => Sensitivity::Medium,
    };
    println!("Sensitivity: {:?}", sensitivity);

    let guard_model = std::env::var("GUARD_MODEL").unwrap_or_else(|_| "granite3-guardian:latest".to_string());
    println!("Guard Model: {}", guard_model);

    // Initialize pooled HTTP client
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    // Run model presence check in the background
    let ollama_url_clone = ollama_url.clone();
    let guard_model_clone = guard_model.clone();
    let http_client_clone = http_client.clone();
    tokio::spawn(async move {
        let client = OllamaClient::new_with_client(&ollama_url_clone, http_client_clone);
        if let Err(e) = client.ensure_model_exists(&guard_model_clone).await {
            eprintln!("Warning: Failed to ensure guard model '{}' exists: {}", guard_model_clone, e);
        } else {
            println!("Ensured specialized security model '{}' exists on backend.", guard_model_clone);
        }
    });

    let state = AppState {
        ollama_url,
        validation_mode,
        sensitivity,
        guard_model,
        http_client,
    };

    // Define the app routes
    let app = create_app(state);

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    println!("Listening on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    println!("TcpListener bound successfully.");
    
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
