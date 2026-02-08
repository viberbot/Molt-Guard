# Implementation Plan: Core Security Middleware

This plan outlines the steps to build the core security middleware and hardened environment.

## Phase 1: Foundation & Project Scaffolding [checkpoint: 96aeb6c]
- [x] Task: Initialize Rust project and Cargo workspace. 76dc756
    - [x] Create `Cargo.toml` with necessary dependencies (tokio, reqwest, serde, etc.).
    - [x] Set up basic project structure.
- [x] Task: Configure Dockerized environment. 41b8df4
    - [x] Create a multi-stage `Dockerfile` using Google Distroless.
    - [x] Define `docker-compose.yml` with Molt bot and HashiCorp Vault.
- [x] Task: Conductor - User Manual Verification 'Foundation & Project Scaffolding' (Protocol in workflow.md) 96aeb6c

## Phase 2: Input Validation (Prompt Guard) [checkpoint: 448bae7]
- [x] Task: Implement Prompt Guard client. ac05342
    - [x] Write tests for the Ollama Prompt Guard API client.
    - [x] Implement the client in Rust to communicate with 192.168.68.68.
- [x] Task: Integrate Input Validation Middleware. a10740b
    - [x] Write tests for the input interceptor logic.
    - [x] Implement middleware to block malicious prompts with educational messages.
- [x] Task: Conductor - User Manual Verification 'Input Validation (Prompt Guard)' (Protocol in workflow.md) 448bae7

## Phase 3: Output Filtering (Secrets & PII) [checkpoint: a99bcc4]
- [x] Task: Implement Secrets Redaction. cde5f63
    - [x] Write tests for regex-based secret detection (similar to gitleaks).
    - [x] Implement the redaction logic in Rust.
- [x] Task: Implement PII Semantic Detection. 5f27cde
    - [x] Write tests for the lightweight LLM PII scanner integration.
    - [x] Implement in-process PII detection using a library like `candle`.
- [x] Task: Conductor - User Manual Verification 'Output Filtering (Secrets & PII)' (Protocol in workflow.md) a99bcc4

## Phase 4: Vault Integration & Hardening
- [x] Task: Implement Vault Configuration Fetching. 91fdab1
    - [ ] Write tests for Vault API integration.
    - [ ] Implement startup logic to fetch configuration from Vault.
- [ ] Task: Finalize Docker Hardening.
    - [ ] Configure non-root user, read-only FS, and resource limits in Docker Compose.
- [ ] Task: Conductor - User Manual Verification 'Vault Integration & Hardening' (Protocol in workflow.md)
