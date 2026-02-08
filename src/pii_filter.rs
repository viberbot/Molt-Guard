use regex::Regex;

pub struct PiiFilter {
    patterns: Vec<Regex>,
}

impl PiiFilter {
    pub fn new() -> Self {
        // Initial PII patterns. In a refined version, this would use a lightweight LLM (candle).
        let patterns = vec![
            // Email address
            Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
            // Phone number (US format)
            Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
        ];
        Self { patterns }
    }

    pub fn redact(&self, input: &str) -> String {
        let mut output = input.to_string();
        for pattern in &self.patterns {
            output = pattern.replace_all(&output, "[PII_REDACTED]").to_string();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_pii_email() {
        let filter = PiiFilter::new();
        let input = "Contact me at john.doe@example.com";
        let output = filter.redact(input);
        assert!(output.contains("[PII_REDACTED]"));
        assert!(!output.contains("john.doe"));
    }

    #[test]
    fn test_redact_pii_phone() {
        let filter = PiiFilter::new();
        let input = "My number is 555-123-4567";
        let output = filter.redact(input);
        assert!(output.contains("[PII_REDACTED]"));
        assert!(!output.contains("555-123"));
    }
}