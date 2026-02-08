# Track Specification: Core Security Middleware & Hardened Environment

## Overview
This track focuses on building the foundational security layer for the Molt bot. It includes the Rust-based middleware for input/output filtering, integration with Ollama for Prompt Guard, and a hardened Dockerized deployment environment.

## Objectives
- Initialize a Rust-based Molt bot project.
- Implement middleware to intercept and validate inputs via Prompt Guard on Ollama.
- Implement output filtering for secrets and PII using Rust crates and lightweight LLM models.
- Orchestrate the environment (Molt, Vault) using Docker Compose.
- Apply security hardening to the Docker containers.

## Functional Requirements
- **Input Validation:** Every user prompt must be checked by Prompt Guard. High-risk prompts are blocked with an educational message.
- **Output Filtering:** Bot responses are scanned for secrets (regex) and PII (semantic LLM). Detected items are redacted.
- **Vault Integration:** Fetch Ollama IP and secrets from Vault at startup. Log security event metadata to Vault.

## Non-Functional Requirements
- **Performance:** Combined I/O filtering overhead should be < 500ms.
- **Safety:** Use Rust's memory safety and Distroless Docker images.
- **Hardening:** Non-root execution, read-only filesystem, and resource limits.
