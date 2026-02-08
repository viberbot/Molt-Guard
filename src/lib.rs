pub mod prompt_guard;
pub mod middleware;
pub mod secrets_filter;
pub mod pii_filter;
pub mod vault;
pub mod api_types;

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use crate::api_types::{ChatCompletionRequest, ChatCompletionResponse, Message, Choice};
use serde::{Deserialize, Serialize};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Molt Bot Secure Proxy" }))
        .route("/health", get(|| async { "OK" }))
        .route("/v1/chat/completions", post(chat_completions_handler))
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Message,
}

async fn chat_completions_handler(
    Json(payload): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, String)> {
    let ollama_url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://192.168.68.68:11434".to_string());

    let client = reqwest::Client::new();
    let url = format!("{}/api/chat", ollama_url);

    let ollama_request = OllamaChatRequest {
        model: payload.model.clone(),
        messages: payload.messages,
        stream: false, // Initially non-streaming for simplicity
    };

    let response = client.post(&url)
        .json(&ollama_request)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((response.status(), "Ollama backend error".to_string()));
    }

    let ollama_response: OllamaChatResponse = response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ChatCompletionResponse {
        id: "chatcmpl-proxy".to_string(),
        object: "chat.completion".to_string(),
        created: 0,
        model: payload.model,
        choices: vec![Choice {
            index: 0,
            message: ollama_response.message,
            finish_reason: Some("stop".to_string()),
        }],
        usage: None,
    }))
}