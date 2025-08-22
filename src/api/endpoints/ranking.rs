use crate::api::common::{ApiClient, ApiResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingRequest {
    pub rtype: String,
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub libtype: Option<u8>,
    pub callback: Option<String>,
}

impl RankingRequest {
    pub fn new(rtype: String) -> Self {
        Self {
            rtype,
            gzip: None,
            out: Some("json".to_string()),
            libtype: None,
            callback: None,
        }
    }
}

impl crate::api::common::request::ApiRequest for RankingRequest {
    fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        
        params.push(("rtype".to_string(), self.rtype.clone()));
        
        if let Some(ref value) = self.gzip {
            params.push(("gzip".to_string(), value.to_string()));
        }
        if let Some(ref value) = self.out {
            params.push(("out".to_string(), value.clone()));
        }
        if let Some(ref value) = self.libtype {
            params.push(("libtype".to_string(), value.to_string()));
        }
        if let Some(ref value) = self.callback {
            params.push(("callback".to_string(), value.clone()));
        }
        
        params
    }
    
    fn output_format(&self) -> crate::api::common::response::OutputFormat {
        if let Some(ref out) = self.out {
            crate::api::common::response::OutputFormat::from_str(out, self.callback.as_deref())
        } else {
            crate::api::common::response::OutputFormat::Yaml
        }
    }
    
    fn is_gzip(&self) -> bool {
        self.gzip.map(|v| v > 0).unwrap_or(false)
    }
    
    fn get_callback(&self) -> Option<String> {
        self.callback.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    pub ncode: String,
    pub pt: u32,
    pub rank: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingResponse {
    #[serde(flatten)]
    pub rankings: Vec<RankingEntry>,
}

impl ApiResponse for RankingResponse {
    fn from_json(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_json::Value = serde_json::from_slice(data)?;
        
        let rankings = if let Some(arr) = value.as_array() {
            arr.iter()
                .map(|v| serde_json::from_value::<RankingEntry>(v.clone()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?
        } else if let Some(obj) = value.as_object() {
            if obj.contains_key("ncode") && obj.contains_key("pt") && obj.contains_key("rank") {
                vec![serde_json::from_value::<RankingEntry>(value)
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        Ok(RankingResponse { rankings })
    }

    fn from_yaml(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_yaml::Value = serde_yaml::from_slice(data)?;
        
        let rankings = if let Some(seq) = value.as_sequence() {
            seq.iter()
                .map(|v| serde_yaml::from_value::<RankingEntry>(v.clone()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?
        } else if let Some(map) = value.as_mapping() {
            let ncode_key = serde_yaml::Value::String("ncode".to_string());
            let pt_key = serde_yaml::Value::String("pt".to_string());
            let rank_key = serde_yaml::Value::String("rank".to_string());
            
            if map.contains_key(&ncode_key) && map.contains_key(&pt_key) && map.contains_key(&rank_key) {
                vec![serde_yaml::from_value::<RankingEntry>(value)
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        Ok(RankingResponse { rankings })
    }
}

pub struct RankingApiClient;

#[async_trait]
impl ApiClient for RankingApiClient {
    type Request = RankingRequest;
    type Response = RankingResponse;

    fn base_url(&self) -> &str {
        "https://api.syosetu.com/rank/rankget/"
    }
}