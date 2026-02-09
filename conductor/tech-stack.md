# Technology Stack

## Core Application & Middleware
- **Language:** Rust
    - *Reason:* High performance, memory safety, and concurrency, ideal for a secure gateway.
- **Project Structure:** Cargo workspace (if applicable) for modularity.

## AI & Security Integration
- **LLM Backend:** Ollama (running remotely on backend-ollama)
- **Prompt Injection Defense:** Prompt Guard Model (running on Ollama)
    - *Integration:* Rust middleware to validate inputs via Ollama API.
- **Output Filtering (Secrets):** Rust-native crates / FFI
    - *Approach:* Regex-based secret scanning logic similar to `gitleaks` implemented directly in Rust or via FFI.
- **Output Filtering (PII):** Rust-native LLM runtime (e.g., `candle`)
    - *Approach:* In-process execution of a lightweight PII detection model.

## Infrastructure & Deployment
- **Containerization:** Docker
    - **Base Image:** Google Distroless (cc-debian12) for maximum security and minimal attack surface.
- **Orchestration:** Docker Compose (for coordinating Molt, Vault, and potential local services).

## Configuration & Secrets Management
- **Secrets Management:** HashiCorp Vault
    - *Role:* Secure storage for sensitive configuration, API keys, and filtering rules.
    - *Setup:* Self-hosted Vault instance (likely via Docker).
