# Initial Concept

hey we want to have a hardened molt bot config here that talks to my ollama server running on backend-ollama I don't know if we can also instll the prompt guard model on ollama and if there a way to configure molt to use it for input validation and output filtering.

# Product Definition: Hardened Molt Bot

## Vision
A highly secure, personal local LLM assistant powered by Molt and Ollama, specifically designed to mitigate prompt injection and prevent sensitive data leakage through advanced input validation and output filtering.

## Target Audience
- Personal use for a secure local LLM assistant.

## Primary Security Goals
- **Prompt Injection Prevention:** Mitigate jailbreak and injection attempts by leveraging the Prompt Guard model.
- **Data Exfiltration Prevention:** Implement strict output filtering to detect and block sensitive information (secrets, PII) before it leaves the bot.
- **Local Sovereignty:** Maintain a strict connection to the local Ollama server (backend-ollama) to ensure data remains within the private network.

## Key Features
- **Prompt Guard Integration:** Automated setup of the Prompt Guard model on the remote Ollama server and middleware to validate user inputs.
- **Multi-Layered Output Filtering:** 
    - Integration with specialized secrets scanning (e.g., `gitleaks`).
    - Semantic PII detection using a lightweight LLM-based scanner.
- **Dockerized Deployment:** Fully containerized configuration for isolation, portability, and ease of management.

## Deployment Strategy
- Docker-based deployment to ensure a consistent and isolated environment for the Molt bot and its security middleware.