# Product Guidelines: Hardened Molt Bot

## Interaction Tone & Style
- **Helpful & Concise:** The assistant should maintain a friendly and supportive demeanor. Security is a foundational pillar but should not overshadow the user experience.
- **Unobtrusive Security:** System-level security checks and logs should be secondary to the user's primary interaction loop unless a violation occurs.

## Security Communication
- **Transparent Rejection:** When a security boundary is hit (e.g., Prompt Guard blocks an input), the bot should provide a brief, educational explanation. 
    - *Example:* "I'm sorry, but I can't process that request as it appears to contain patterns associated with prompt injection."
- **Clarity over Mystery:** Avoid generic error messages. Help the user understand the security constraints without revealing sensitive architectural details.

## Data Handling & Privacy
- **Redaction by Default:** When the output filter detects a secret or PII, it should be replaced with a clear placeholder (e.g., `[SECRET_DETECTED]`, `[PII_REDACTED]`).
- **Context Preservation:** Aim to redact only the sensitive tokens, preserving as much of the surrounding context as possible to keep the response useful.

## Visual Identity (CLI/Interface)
- **Minimalist:** Use standard CLI formatting. Use subtle color cues (e.g., yellow for warnings, blue for system info) if supported by the terminal, ensuring they don't distract from the text.
