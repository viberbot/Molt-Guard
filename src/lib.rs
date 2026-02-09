pub mod prompt_guard;
pub mod middleware;
pub mod secrets_filter;
pub mod pii_filter;
pub mod api_types;
pub mod ollama_client;

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::StatusCode,
};
use crate::api_types::{ChatCompletionRequest, ChatCompletionResponse, Message, Choice, ListModelsResponse, ModelObject};
use crate::prompt_guard::{PromptGuardClient, ValidationMode, Sensitivity};
use crate::middleware::InputValidationMiddleware;
use crate::secrets_filter::SecretsFilter;
use crate::pii_filter::PiiFilter;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub ollama_url: String,
    pub validation_mode: ValidationMode,
    pub sensitivity: Sensitivity,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Molt-Guard Secure Proxy" }))
        .route("/health", get(|| async { "OK" }))
        .route("/v1/models", get(list_models_handler))
        .route("/v1/chat/completions", post(chat_completions_handler))
        .with_state(state)
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
    prompt_eval_count: Option<u64>,
    eval_count: Option<u64>,
}

async fn list_models_handler(
    State(state): State<AppState>,
) -> Result<Json<ListModelsResponse>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", state.ollama_url);

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    #[derive(Deserialize)]
    struct OllamaTagsResponse {
        models: Vec<OllamaModel>,
    }

    #[derive(Deserialize)]
    struct OllamaModel {
        name: String,
    }

    let ollama_tags: OllamaTagsResponse = response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let models = ollama_tags.models.into_iter().map(|m| ModelObject {
        id: m.name,
        object: "model".to_string(),
        created: 0,
        owned_by: "ollama".to_string(),
    }).collect();

    Ok(Json(ListModelsResponse {
        object: "list".to_string(),
        data: models,
    }))
}

async fn chat_completions_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, String)> {
    
    // 1. Input Validation
    let prompt_guard = PromptGuardClient::new(&state.ollama_url, state.validation_mode, state.sensitivity);
    let middleware = InputValidationMiddleware::new(prompt_guard);

    // Extract the latest user message for validation
    if let Some(last_message) = payload.messages.last() {
        if last_message.role == "user" {
            match middleware.process(&last_message.content).await {
                Ok(_) => {}, // Safe
                Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
            }
        }
    }

    // 2. Forward to Ollama
    let client = reqwest::Client::new();
    let url = format!("{}/api/chat", state.ollama_url);

    let ollama_request = OllamaChatRequest {
        model: payload.model.clone(),
        messages: payload.messages,
        stream: false,
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

    // 3. Output Redaction
    let secrets_filter = SecretsFilter::new();
    let pii_filter = PiiFilter::new();

    let mut content = ollama_response.message.content;
    content = secrets_filter.redact(&content);
    content = pii_filter.redact(&content);

    // Populate usage if available
    let usage = match (ollama_response.prompt_eval_count, ollama_response.eval_count) {
        (Some(p), Some(e)) => Some(crate::api_types::Usage {
            prompt_tokens: p,
            completion_tokens: e,
            total_tokens: p + e,
        }),
        _ => None,
    };

    Ok(Json(ChatCompletionResponse {
        id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        model: payload.model,
        choices: vec![Choice {
            index: 0,
            message: Message {
                role: ollama_response.message.role,
                content,
            },
            finish_reason: Some("stop".to_string()),
        }],
        usage,
        system_fingerprint: None,
    }))
}
