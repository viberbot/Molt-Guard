# Implementation Plan: Core Security Middleware

This plan outlines the steps to build the core security middleware and hardened environment.

## Phase 1: Foundation & Project Scaffolding
- [ ] Task: Initialize Rust project and Cargo workspace.
    - [ ] Create `Cargo.toml` with necessary dependencies (tokio, reqwest, serde, etc.).
    - [ ] Set up basic project structure.
- [ ] Task: Configure Dockerized environment.
    - [ ] Create a multi-stage `Dockerfile` using Google Distroless.
    - [ ] Define `docker-compose.yml` with Molt bot and HashiCorp Vault.
- [ ] Task: Conductor - User Manual Verification 'Foundation & Project Scaffolding' (Protocol in workflow.md)

## Phase 2: Input Validation (Prompt Guard)
- [ ] Task: Implement Prompt Guard client.
    - [ ] Write tests for the Ollama Prompt Guard API client.
    - [ ] Implement the client in Rust to communicate with 192.168.68.68.
- [ ] Task: Integrate Input Validation Middleware.
    - [ ] Write tests for the input interceptor logic.
    - [ ] Implement middleware to block malicious prompts with educational messages.
- [ ] Task: Conductor - User Manual Verification 'Input Validation (Prompt Guard)' (Protocol in workflow.md)

## Phase 3: Output Filtering (Secrets & PII)
- [ ] Task: Implement Secrets Redaction.
    - [ ] Write tests for regex-based secret detection (similar to gitleaks).
    - [ ] Implement the redaction logic in Rust.
- [ ] Task: Implement PII Semantic Detection.
    - [ ] Write tests for the lightweight LLM PII scanner integration.
    - [ ] Implement in-process PII detection using a library like `candle`.
- [ ] Task: Conductor - User Manual Verification 'Output Filtering (Secrets & PII)' (Protocol in workflow.md)

## Phase 4: Vault Integration & Hardening
- [ ] Task: Implement Vault Configuration Fetching.
    - [ ] Write tests for Vault API integration.
    - [ ] Implement startup logic to fetch configuration from Vault.
- [ ] Task: Finalize Docker Hardening.
    - [ ] Configure non-root user, read-only FS, and resource limits in Docker Compose.
- [ ] Task: Conductor - User Manual Verification 'Vault Integration & Hardening' (Protocol in workflow.md)
