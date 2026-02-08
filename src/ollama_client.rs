use anyhow::Result;
use serde::Deserialize;

pub struct OllamaClient {
    base_url: String,
    http_client: reqwest::Client,
}

#[derive(Deserialize)]
struct ModelList {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn check_model_exists(&self, model_name: &str) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to list models: {}", response.status()));
        }

        let list: ModelList = response.json().await?;
        Ok(list.models.iter().any(|m| m.name == model_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use serde_json::json;

    #[tokio::test]
    async fn test_check_model_exists() {
        let mock_server = MockServer::start().await;

        let tags_response = json!({
            "models": [
                { "name": "llama3:latest" },
                { "name": "prompt-guard:latest" }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tags_response))
            .mount(&mock_server)
            .await;

        let client = OllamaClient::new(&mock_server.uri());
        let exists = client.check_model_exists("prompt-guard:latest").await.unwrap();
        assert!(exists);

        let missing = client.check_model_exists("gpt-4").await.unwrap();
        assert!(!missing);
    }
}