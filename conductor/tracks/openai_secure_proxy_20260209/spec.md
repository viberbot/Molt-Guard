# Track Specification: OpenAI-Compatible Secure Proxy

## Overview
This track focuses on transforming the existing Rust-based security middleware into a fully functional, OpenAI-compatible HTTP proxy. This will allow any OpenAI-compatible client (like OpenClaw, Moltbot, or generic chat UIs) to connect to our secure gateway, which will transparently enforce Prompt Guard validation and output redaction before forwarding requests to the backend Ollama server.

## Objectives
- Integrate a web server framework (Axum) into the Rust project.
- Implement an OpenAI-compatible `/v1/chat/completions` endpoint.
- Wire up the existing security middleware (Prompt Guard, Secrets/PII filters) to the request/response lifecycle.
- Update `docker-compose.yml` to expose the proxy service.

## Functional Requirements
- **API Compatibility:** The proxy must accept POST requests to `/v1/chat/completions` conforming to the OpenAI API spec.
- **Request Flow:**
    1.  Receive request from client.
    2.  Extract user prompt.
    3.  **Security Check:** Validate prompt using `InputValidationMiddleware` (Prompt Guard).
    4.  **Forward:** If safe, forward the request to the backend Ollama server (preserving streaming if requested).
    5.  **Response Flow:** Receive response from Ollama.
    6.  **Security Check:** Scan and redact response using `SecretsFilter` and `PiiFilter`.
    7.  Return sanitized response to client.
- **Error Handling:** Return standard HTTP 4xx/5xx errors with clear messages if security checks fail.

## Non-Functional Requirements
- **Latency:** Proxy overhead should be minimal (< 50ms excluding model inference).
- **Streaming:** Support Server-Sent Events (SSE) for streaming responses if feasible (initial version may be non-streaming for simplicity of filtering).
