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
    http::{StatusCode, Method, HeaderMap},
    response::{Response, IntoResponse},
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
        .route("/api/chat", post(ollama_chat_handler))
        .route("/api/generate", post(ollama_generate_handler))
        .fallback(proxy_fallback_handler)
        .with_state(state)
}

#[derive(Serialize, Deserialize, Clone)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(default)]
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Message,
    prompt_eval_count: Option<u64>,
    eval_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    #[serde(default)]
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

async fn proxy_fallback_handler(
    State(state): State<AppState>,
    method: Method,
    uri: axum::http::Uri,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, (StatusCode, String)> {
    let path_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");
    let url = format!("{}{}", state.ollama_url, path_query);
    
    let client = reqwest::Client::new();
    let mut rb = client.request(method, &url);
    
    for (key, value) in headers.iter() {
        if key != "host" && key != "content-length" {
            rb = rb.header(key, value);
        }
    }
    
    let bytes = axum::body::to_bytes(body, 100 * 1024 * 1024).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let res = rb.body(bytes)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut builder = Response::builder().status(res.status());
    for (key, value) in res.headers().iter() {
        if key != "transfer-encoding" {
            builder = builder.header(key, value);
        }
    }

    let res_bytes = res.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(builder.body(axum::body::Body::from(res_bytes)).unwrap())
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
    
    let prompt_guard = PromptGuardClient::new(&state.ollama_url, state.validation_mode, state.sensitivity);
    let middleware = InputValidationMiddleware::new(prompt_guard);

    if let Some(last_message) = payload.messages.last() {
        if last_message.role == "user" {
            if let Err(e) = middleware.process(&last_message.content).await {
                // RETURN AS LLM RESPONSE
                return Ok(Json(ChatCompletionResponse {
                    id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
                    object: "chat.completion".to_string(),
                    created: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    model: payload.model,
                    choices: vec![Choice {
                        index: 0,
                        message: Message {
                            role: "assistant".to_string(),
                            content: format!("🛡️ **Molt-Guard Security Alert**: {}", e),
                        },
                        finish_reason: Some("stop".to_string()),
                    }],
                    usage: None,
                    system_fingerprint: None,
                }));
            }
        }
    }

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
        let err_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Ollama error: {}", err_body)));
    }

    let ollama_response: OllamaChatResponse = response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let content = apply_filters(ollama_response.message.content);

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

async fn ollama_chat_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OllamaChatRequest>,
) -> Result<Response, (StatusCode, String)> {
    
    let prompt_guard = PromptGuardClient::new(&state.ollama_url, state.validation_mode, state.sensitivity);
    let middleware = InputValidationMiddleware::new(prompt_guard);

    if let Some(last_message) = payload.messages.last() {
        if last_message.role == "user" {
            if let Err(e) = middleware.process(&last_message.content).await {
                // RETURN AS OLLAMA RESPONSE
                let ollama_resp = serde_json::json!({
                    "model": payload.model,
                    "created_at": "2026-02-09T00:00:00Z",
                    "message": {
                        "role": "assistant",
                        "content": format!("🛡️ **Molt-Guard Security Alert**: {}", e)
                    },
                    "done": true
                });
                return Ok(Json(ollama_resp).into_response());
            }
        }
    }

    proxy_forward_json(&state.ollama_url, "/api/chat", Method::POST, headers, &payload).await
}

async fn ollama_generate_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OllamaGenerateRequest>,
) -> Result<Response, (StatusCode, String)> {
    
    let prompt_guard = PromptGuardClient::new(&state.ollama_url, state.validation_mode, state.sensitivity);
    let middleware = InputValidationMiddleware::new(prompt_guard);

    if let Err(e) = middleware.process(&payload.prompt).await {
        // RETURN AS OLLAMA GENERATE RESPONSE
        let ollama_resp = serde_json::json!({
            "model": payload.model,
            "created_at": "2026-02-09T00:00:00Z",
            "response": format!("🛡️ **Molt-Guard Security Alert**: {}", e),
            "done": true
        });
        return Ok(Json(ollama_resp).into_response());
    }

    proxy_forward_json(&state.ollama_url, "/api/generate", Method::POST, headers, &payload).await
}

async fn proxy_forward_json<T: Serialize>(base_url: &str, path: &str, method: Method, headers: HeaderMap, payload: &T) -> Result<Response, (StatusCode, String)> {
    let url = format!("{}{}", base_url, path);
    let client = reqwest::Client::new();
    
    let mut rb = client.request(method, &url);
    for (key, value) in headers.iter() {
        if key != "host" && key != "content-length" {
            rb = rb.header(key, value);
        }
    }

    let res = rb.json(payload)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut builder = Response::builder().status(res.status());
    for (key, value) in res.headers().iter() {
        if key != "transfer-encoding" {
            builder = builder.header(key, value);
        }
    }

    let res_bytes = res.bytes().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let body_str = String::from_utf8_lossy(&res_bytes);
    let redacted_str = apply_filters(body_str.to_string());
    
    Ok(builder.body(axum::body::Body::from(redacted_str)).unwrap())
}

fn apply_filters(content: String) -> String {
    let secrets_filter = SecretsFilter::new();
    let pii_filter = PiiFilter::new();
    let mut filtered = secrets_filter.redact(&content);
    filtered = pii_filter.redact(&filtered);
    filtered
}
