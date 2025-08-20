use anyhow::{Context, Result};
use std::collections::HashMap;
use crate::api::narou::{NarouApiClient, NarouRankinResponse};

/// なろう殿堂入りAPI
pub struct NarouRankinApi {
    client: NarouApiClient,
    base_url: String,
}

impl NarouRankinApi {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: NarouApiClient::new()?,
            base_url: "https://api.syosetu.com/rank/rankin/".to_string(),
        })
    }

    /// 殿堂入り情報を取得
    pub async fn get_rankin(&self, ncode: &str) -> Result<NarouRankinResponse> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        query.insert("ncode".to_string(), ncode.to_uppercase());  // Ncodeは大文字に正規化
        
        self.client.request(&self.base_url, &query).await
    }

    /// 複数作品の殿堂入り情報を一括取得
    pub async fn get_rankin_batch(&self, ncodes: Vec<&str>) -> Result<Vec<(String, Result<NarouRankinResponse>)>> {
        let mut results = Vec::new();
        
        for ncode in ncodes {
            let result = self.get_rankin(ncode).await;
            results.push((ncode.to_string(), result));
        }
        
        Ok(results)
    }
}