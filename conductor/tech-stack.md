# Technology Stack

## Core Application & Middleware
- **Language:** Rust
    - *Reason:* High performance, memory safety, and concurrency, ideal for a secure gateway.
- **Project Structure:** Cargo workspace (if applicable) for modularity.

## AI & Security Integration
- **LLM Backend:** Ollama or any OpenAI-compatible API (configured via environment).
- **Prompt Injection Defense:** Prompt Guard Model (running on Ollama)
    - *Integration:* Rust middleware to validate inputs via Ollama API.
- **Output Filtering (Secrets):** Rust-native crates / FFI
    - *Approach:* Regex-based secret scanning logic implemented directly in Rust.
- **Output Filtering (PII):** 
    - *Approach:* High-performance regex-based PII detection.

## Infrastructure & Deployment
- **Containerization:** Docker
    - **Base Image:** Google Distroless (cc-debian12) for maximum security and minimal attack surface.
- **Orchestration:** Docker Compose.