use anyhow::Result;
use std::collections::HashMap;
use crate::api::narou::{NarouApiClient, NarouRankinResponse, NarouRankinRecord};

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
    pub async fn get_rankin(&self, ncode: &str, gzip: Option<u8>) -> Result<NarouRankinResponse> {
        self.get_rankin_with_options(ncode, gzip, None, None).await
    }
    
    /// 殿堂入り情報を取得（拡張オプション付き）
    pub async fn get_rankin_with_options(
        &self, 
        ncode: &str, 
        gzip: Option<u8>,
        libtype: Option<u8>,
        callback: Option<String>
    ) -> Result<NarouRankinResponse> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        query.insert("ncode".to_string(), ncode.to_uppercase());  // Ncodeは大文字に正規化
        
        // 出力形式制御（注：現在はJSON固定のため、これらのパラメータは効果なし）
        if let Some(libtype) = libtype {
            query.insert("libtype".to_string(), libtype.to_string());
        }
        if let Some(callback) = callback {
            query.insert("callback".to_string(), callback);
        }
        
        // gzip圧縮対応（殿堂入りAPIではgzipは動作しない可能性がある）
        if let Some(level) = gzip {
            if (1..=5).contains(&level) && level != 3 {  // レベル3は避ける
                query.insert("gzip".to_string(), level.to_string());
            }
        }
        
        // 殿堂入りAPIは配列を返すので、通常のrequestメソッドを使用
        let records: Vec<NarouRankinRecord> = self.client.request(&self.base_url, &query).await?;
        Ok(NarouRankinResponse { rank: records })
    }

    /// 複数作品の殿堂入り情報を一括取得
    pub async fn get_rankin_batch(&self, ncodes: Vec<&str>, gzip: Option<u8>) -> Result<Vec<(String, Result<NarouRankinResponse>)>> {
        let mut results = Vec::new();
        
        for ncode in ncodes {
            let result = self.get_rankin(ncode, gzip).await;
            results.push((ncode.to_string(), result));
        }
        
        Ok(results)
    }
}