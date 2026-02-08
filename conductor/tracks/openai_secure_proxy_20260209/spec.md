# Track Specification: OpenAI-Compatible Secure Proxy

## Overview
This track focuses on transforming the existing Rust-based security middleware into a fully functional, OpenAI-compatible HTTP proxy. This will allow any OpenAI-compatible client (like OpenClaw, Moltbot, or generic chat UIs) to connect to our secure gateway. It will transparently enforce Prompt Guard validation and output redaction before forwarding requests to the backend Ollama server.

## Objectives
- Integrate a web server framework (Axum) into the Rust project.
- Implement an OpenAI-compatible `/v1/chat/completions` endpoint.
- Wire up the existing security middleware (Prompt Guard, Secrets/PII filters) to the request/response lifecycle.
- **New:** Support dual-mode validation: Run Prompt Guard/PII checks either on the remote Ollama server or locally (using `candle` or similar).
- **New:** Automatically manage and download required models (e.g., `prompt-guard`, `llama3`) on the Ollama server if they are missing.
- **New:** Create comprehensive integration tests for the proxy.
- Update `docker-compose.yml` to expose the proxy service.

## Functional Requirements
- **API Compatibility:** The proxy must accept POST requests to `/v1/chat/completions` conforming to the OpenAI API spec.
- **Request Flow:**
    1.  Receive request from client.
    2.  Extract user prompt.
    3.  **Security Check:** Validate prompt using `InputValidationMiddleware` (Prompt Guard).
        - *Mode A (Remote):* Query Ollama for validation.
        - *Mode B (Local):* Run validation model locally.
    4.  **Forward:** If safe, forward the request to the backend Ollama server (preserving streaming if requested).
    5.  **Response Flow:** Receive response from Ollama.
    6.  **Security Check:** Scan and redact response using `SecretsFilter` and `PiiFilter`.
    7.  Return sanitized response to client.
- **Model Management:** On startup, check if the configured models exist on the Ollama server. If not, trigger a download (pull).
- **Configuration:** Allow configuring the Ollama URL, Validation Mode (Local/Remote), and Model names via environment variables/Vault.
- **Error Handling:** Return standard HTTP 4xx/5xx errors with clear messages if security checks fail.

## Non-Functional Requirements
- **Latency:** Proxy overhead should be minimal (< 50ms excluding model inference).
- **Testability:** The proxy must be verifiable via automated integration tests.