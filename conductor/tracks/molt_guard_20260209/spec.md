# Track Specification: Project Professionalization and "Molt-Guard" Rebranding

## Overview
This track focuses on transforming the current specialized Molt bot configuration into a professional, generic, and open-source-ready security proxy named **Molt-Guard**. The goal is to provide a plug-and-play solution that adds a security layer (Prompt Guard, PII/Secret filtering) to any OpenAI-compatible or Ollama-based workflow while ensuring no sensitive local configuration is leaked.

## Functional Requirements
- **Rebranding:** Rename the project and binary to `molt-guard`.
- **OpenAI Compatibility:** Ensure the API layer fully supports standard OpenAI-compatible request/response formats.
- **Generic Configuration:** 
    - Move all environment-specific values (Ollama IP, model names, port mappings) to `docker-compose.yml` environment variables.
    - Use `backend-ollama:11434` as the default internal identifier.
- **Security Enhancements:**
    - Implement configurable sensitivity levels for the Prompt Guard validation.
    - Remove HashiCorp Vault to simplify the architecture, using environment variables/secrets instead.
- **Professional Documentation:**
    - Update README.md with Mermaid.js diagrams illustrating the request/response security flow.
    - Add a "Security Audit" section explaining the hardening measures (Distroless, non-root, etc.).

## Non-Functional Requirements
- **Zero-Leak Policy:** Ensure no local IPs (e.g., `192.168.68.68`) or private identifiers remain in the codebase or git history.
- **Simplicity:** The stack should consist only of the Rust proxy and the required backend (Ollama), removing unnecessary dependencies like Vault.

## Acceptance Criteria
- [ ] Project renamed to `molt-guard` across all files (Cargo.toml, Dockerfile, etc.).
- [ ] Proxy successfully forwards requests using the generic `backend-ollama` hostname.
- [ ] README.md contains at least two Mermaid diagrams (Architecture and Security Flow).
- [ ] Git history is sanitized of local IPs and sensitive data.
- [ ] Vault service is removed from `docker-compose.yml` and the Rust codebase.
- [ ] Successful end-to-end test using a generic OpenAI client.

## Out of Scope
- Implementing a frontend GUI for configuration.
- Support for non-Ollama backends (e.g., Anthropic, Gemini) in this specific track.
