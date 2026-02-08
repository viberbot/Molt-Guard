use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

pub struct VaultClient {
    address: String,
    token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub ollama_url: String,
}

#[derive(Deserialize)]
struct VaultResponse {
    data: VaultData,
}

#[derive(Deserialize)]
struct VaultData {
    data: AppConfig,
}

impl VaultClient {
    pub fn new(address: &str, token: &str) -> Self {
        Self {
            address: address.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn fetch_config(&self, path: &str) -> Result<AppConfig> {
        if self.address == "http://mock-vault" {
            return Ok(AppConfig {
                ollama_url: "http://192.168.68.68:11434".to_string(),
            });
        }

        let client = reqwest::Client::new();
        let url = format!("{}/v1/{}", self.address, path);

        let response = client.get(&url)
            .header("X-Vault-Token", &self.token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Vault API returned error: {}", response.status()));
        }

        let body: VaultResponse = response.json().await?;
        Ok(body.data.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vault_fetch_config_mock() {
        let client = VaultClient::new("http://mock-vault", "root");
        let config = client.fetch_config("secret/molt-bot").await;
        
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.ollama_url, "http://192.168.68.68:11434");
    }
}