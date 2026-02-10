use anyhow::Result;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
struct PullModelRequest {
    name: String,
    stream: bool,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_with_client(base_url: &str, http_client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client,
        }
    }

    pub async fn check_model_exists(&self, model_name: &str) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.http_client.get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to list models: {}", response.status()));
        }

        let list: ModelList = response.json().await?;
        Ok(list.models.iter().any(|m| m.name == model_name))
    }

    pub async fn pull_model(&self, model_name: &str) -> Result<()> {
        let url = format!("{}/api/pull", self.base_url);
        let request = PullModelRequest {
            name: model_name.to_string(),
            stream: false,
        };

        let response = self.http_client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to pull model: {}", response.status()));
        }

        Ok(())
    }

    pub async fn ensure_model_exists(&self, model_name: &str) -> Result<()> {
        if !self.check_model_exists(model_name).await? {
            println!("Model {} missing, pulling...", model_name);
            self.pull_model(model_name).await?;
            println!("Model {} pulled successfully.", model_name);
        }
        Ok(())
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

    #[tokio::test]
    async fn test_pull_model() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/pull"))
            .and(wiremock::matchers::body_json(json!({"name": "prompt-guard:latest", "stream": false})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "success"})))
            .mount(&mock_server)
            .await;

        let client = OllamaClient::new(&mock_server.uri());
        let result = client.pull_model("prompt-guard:latest").await;
        assert!(result.is_ok());
    }
}