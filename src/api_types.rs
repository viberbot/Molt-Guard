use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_chat_completion_request_deserialization() {
        let data = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "user", "content": "Hello!"}
            ],
            "stream": false
        });
        let request: ChatCompletionRequest = serde_json::from_value(data).unwrap();
        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.messages[0].role, "user");
        assert_eq!(request.messages[0].content, "Hello!");
    }

    #[test]
    fn test_chat_completion_response_serialization() {
        let response = ChatCompletionResponse {
            id: "chatcmpl-123".to_string(),
            object: "chat.completion".to_string(),
            created: 1677652288,
            model: "gpt-3.5-turbo".to_string(),
            choices: vec![
                Choice {
                    index: 0,
                    message: Message {
                        role: "assistant".to_string(),
                        content: "Hello there!".to_string(),
                    },
                    finish_reason: Some("stop".to_string()),
                }
            ],
            usage: Some(Usage {
                prompt_tokens: 9,
                completion_tokens: 12,
                total_tokens: 21,
            }),
            system_fingerprint: None,
        };
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("Hello there!"));
    }
}
