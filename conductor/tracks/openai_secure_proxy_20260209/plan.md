# Implementation Plan: OpenAI-Compatible Secure Proxy

This plan outlines the steps to build the HTTP proxy layer.

## Phase 1: Web Server Foundation
- [ ] Task: Add Axum and Tokio dependencies.
    - [ ] Update `Cargo.toml` with `axum`, `tower`, `tower-http`.
    - [ ] Create basic server scaffold in `src/main.rs`.
- [ ] Task: Implement Health Check Endpoint.
    - [ ] Create a `/health` endpoint to verify the server is running.
    - [ ] Write a test to query the health endpoint.
- [ ] Task: Conductor - User Manual Verification 'Web Server Foundation' (Protocol in workflow.md)

## Phase 2: OpenAI Proxy Logic
- [ ] Task: Define OpenAI Data Structures.
    - [ ] Create `src/api_types.rs` with structs for `ChatCompletionRequest`, `ChatCompletionResponse`, etc.
    - [ ] Use `serde` for serialization/deserialization.
- [ ] Task: Implement `/v1/chat/completions` Handler (Basic Forwarding).
    - [ ] Create the handler that accepts the request and forwards it to Ollama *without* filtering first.
    - [ ] Verify connectivity to Ollama.
- [ ] Task: Conductor - User Manual Verification 'OpenAI Proxy Logic' (Protocol in workflow.md)

## Phase 3: Security Integration
- [ ] Task: Integrate Input Validation.
    - [ ] Update the handler to call `InputValidationMiddleware` before forwarding.
    - [ ] Return 400 Bad Request if Prompt Guard blocks the input.
- [ ] Task: Integrate Output Redaction.
    - [ ] Update the handler to capture the Ollama response.
    - [ ] Apply `SecretsFilter` and `PiiFilter` to the response content.
    - [ ] Return the sanitized response.
- [ ] Task: Conductor - User Manual Verification 'Security Integration' (Protocol in workflow.md)

## Phase 4: Docker & Deployment
- [ ] Task: Update Docker Configuration.
    - [ ] expose port 3000 in `docker-compose.yml`.
    - [ ] Update `Dockerfile` to expose the port.
- [ ] Task: Conductor - User Manual Verification 'Docker & Deployment' (Protocol in workflow.md)
