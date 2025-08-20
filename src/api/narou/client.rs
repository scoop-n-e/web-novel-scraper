use anyhow::{Context, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// なろうAPIクライアント（基底クラス）
pub struct NarouApiClient {
    client: Client,
}

impl NarouApiClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (compatible; NarouScraper/1.0)")
                .timeout(std::time::Duration::from_secs(30))
                .build()?,
        })
    }

    pub async fn request<T: DeserializeOwned>(&self, url: &str, params: &HashMap<String, String>) -> Result<T> {
        let response = self.client.get(url)
            .query(&params)
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", url))?;
        
        if !response.status().is_success() {
            anyhow::bail!("API request failed with status: {}", response.status());
        }
        
        let text = response.text().await?;
        
        // レスポンスが空の場合
        if text.trim().is_empty() {
            anyhow::bail!("Empty response from API");
        }
        
        // JSONパース
        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse API response: {}", &text[..text.len().min(500)]))
    }

    pub async fn request_with_gzip<T: DeserializeOwned>(
        &self, 
        url: &str, 
        params: &mut HashMap<String, String>,
        gzip_level: Option<u8>
    ) -> Result<T> {
        // gzip圧縮レベルを指定（1-5）
        if let Some(level) = gzip_level {
            if (1..=5).contains(&level) {
                params.insert("gzip".to_string(), level.to_string());
            }
        }
        
        self.request(url, params).await
    }
}