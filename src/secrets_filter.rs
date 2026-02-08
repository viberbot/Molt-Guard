use regex::Regex;

pub struct SecretsFilter {
    patterns: Vec<Regex>,
}

impl SecretsFilter {
    pub fn new() -> Self {
        // Basic patterns for API keys, SSH keys, etc.
        // This would be expanded or loaded from Vault in a real scenario.
        let patterns = vec![
            // Generic high-entropy string (simulating an API key)
            Regex::new(r"[0-9a-zA-Z]{5}-[0-9a-zA-Z]{5}-[0-9a-zA-Z]{5}-[0-9a-zA-Z]{5}").unwrap(),
            // Potential AWS access key
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
        ];
        Self { patterns }
    }

    pub fn redact(&self, input: &str) -> String {
        let mut output = input.to_string();
        for pattern in &self.patterns {
            output = pattern.replace_all(&output, "[SECRET_DETECTED]").to_string();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_secrets_api_key() {
        let filter = SecretsFilter::new();
        let input = "My API key is 12345-ABCDE-67890-FGHIJ";
        let output = filter.redact(input);
        assert!(output.contains("[SECRET_DETECTED]"));
        assert!(!output.contains("12345-ABCDE"));
    }

    #[test]
    fn test_redact_secrets_multiple() {
        let filter = SecretsFilter::new();
        let input = "Key1: 12345-ABCDE-67890-FGHIJ, Key2: 09876-ZYXWV-54321-UTSRQ";
        let output = filter.redact(input);
        assert_eq!(output.matches("[SECRET_DETECTED]").count(), 2);
    }
}