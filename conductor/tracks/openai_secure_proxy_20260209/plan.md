# Implementation Plan: OpenAI-Compatible Secure Proxy

This plan outlines the steps to build the HTTP proxy layer.

## Phase 1: Web Server Foundation [checkpoint: 8e89f34]
- [x] Task: Add Axum and Tokio dependencies. 6d73f67
    - [x] Update `Cargo.toml` with `axum`, `tower`, `tower-http`, `tokio`.
    - [x] Create basic server scaffold in `src/main.rs`.
- [x] Task: Implement Health Check Endpoint. e7b9bb4
    - [x] Create a `/health` endpoint to verify the server is running.
    - [x] Write a test to query the health endpoint.
- [x] Task: Conductor - User Manual Verification 'Web Server Foundation' (Protocol in workflow.md) 8e89f34

## Phase 2: OpenAI Proxy Logic & Integration Tests [checkpoint: e555ff3]
- [x] Task: Define OpenAI Data Structures. fab900b
    - [x] Create `src/api_types.rs` with structs for `ChatCompletionRequest`, `ChatCompletionResponse`, etc.
    - [x] Use `serde` for serialization/deserialization.
- [x] Task: Implement `/v1/chat/completions` Handler (Basic Forwarding). 95d709b
    - [x] Create the handler that accepts the request and forwards it to Ollama *without* filtering first.
- [x] Task: Create Proxy Integration Tests. 95d709b
    - [x] Create `tests/integration_test.rs`.
    - [x] Write a test that spins up the proxy and mocks the Ollama backend to verify forwarding.
- [x] Task: Conductor - User Manual Verification 'OpenAI Proxy Logic' (Protocol in workflow.md) e555ff3

## Phase 3: Model Management (Auto-Download)
- [ ] Task: Implement Model Availability Check.
    - [ ] Update `VaultClient` (or create `OllamaClient`) to list local models on the Ollama server.
- [ ] Task: Implement Auto-Download Logic.
    - [ ] If a required model (e.g., `prompt-guard`) is missing, trigger `ollama pull`.
    - [ ] Integrate this check into the server startup sequence.
- [ ] Task: Conductor - User Manual Verification 'Model Management' (Protocol in workflow.md)

## Phase 4: Advanced Security Integration (Dual-Mode)
- [ ] Task: Refactor Validation for Dual-Mode.
    - [ ] Update `PromptGuardClient` to support a "Local" mode (using `candle` or similar) vs "Remote" mode (Ollama).
    - [ ] Add configuration parsing for validation mode.
- [ ] Task: Integrate Input Validation Middleware.
    - [ ] Update the handler to call `InputValidationMiddleware` before forwarding.
    - [ ] Return 400 Bad Request if Prompt Guard blocks the input.
- [ ] Task: Integrate Output Redaction.
    - [ ] Update the handler to capture the Ollama response.
    - [ ] Apply `SecretsFilter` and `PiiFilter` to the response content.
    - [ ] Return the sanitized response.
- [ ] Task: Conductor - User Manual Verification 'Advanced Security Integration' (Protocol in workflow.md)

## Phase 5: Docker & Deployment
- [ ] Task: Update Docker Configuration.
    - [ ] Expose port 3000 in `docker-compose.yml`.
    - [ ] Update `Dockerfile` to expose the port.
    - [ ] Add new configuration variables to `docker-compose.yml`.
- [ ] Task: Conductor - User Manual Verification 'Docker & Deployment' (Protocol in workflow.md)