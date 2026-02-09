# Initial Concept

hey we want to have a hardened molt bot config here that talks to my ollama server running on backend-ollama I don't know if we can also instll the prompt guard model on ollama and if there a way to configure molt to use it for input validation and output filtering.

# Product Definition: Molt-Guard

## Vision
A high-performance, professional-grade security proxy designed to harden LLM workflows. Molt-Guard acts as a secure gateway between AI clients and backends, providing automated prompt injection protection and high-speed output redaction.

## Target Audience
- Developers and teams seeking to secure local or remote LLM backends (Ollama, OpenAI-compatible).

## Primary Security Goals
- **Prompt Injection Prevention:** Mitigate jailbreak and injection attempts by leveraging the Prompt Guard model with configurable sensitivity.
- **Data Exfiltration Prevention:** Implement strict output filtering to detect and block sensitive information (secrets, PII) before it leaves the proxy.
- **Transparent Security:** Provide a generic, OpenAI-compatible API that can be dropped into existing workflows without breaking functionality.

## Key Features
- **OpenAI-Compatible Gateway:** A generic proxy that supports standard OpenAI and Ollama-native request formats.
- **Prompt Guard Integration:** Configurable validation using the Prompt Guard model.
- **Multi-Layered Output Filtering:** 
    - Integration with specialized secrets scanning.
    - High-speed PII redaction (Email, Phone Numbers).
- **Transparent Fallback:** Intelligent forwarding of non-chat requests to ensure full backend compatibility.

## Deployment Strategy
- Docker-based deployment to ensure a consistent and isolated environment for the proxy and its security middleware.
