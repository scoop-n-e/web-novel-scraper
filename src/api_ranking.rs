use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRankingApiRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub rtype: String,
    pub libtype: Option<u8>,
    pub callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRankingApiResponse {
    #[serde(flatten)]
    pub rankings: Vec<RankingEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    pub ncode: String,
    pub pt: u32,
    pub rank: u16,
}

pub struct NarouRankingApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NarouRankingApiClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .unwrap();
        
        Self {
            client,
            base_url: "https://api.syosetu.com/rank/rankget/".to_string(),
        }
    }

    pub async fn get_rankings(&self, params: &NarouRankingApiRequest) -> Result<NarouRankingApiResponse, Box<dyn std::error::Error>> {
        let mut query_params = Vec::new();

        query_params.push(("rtype", params.rtype.clone()));

        if let Some(v) = &params.gzip {
            query_params.push(("gzip", v.to_string()));
        }
        if let Some(v) = &params.out {
            query_params.push(("out", v.clone()));
        }
        if let Some(v) = &params.libtype {
            query_params.push(("libtype", v.to_string()));
        }
        if let Some(v) = &params.callback {
            query_params.push(("callback", v.clone()));
        }

        let response = self.client
            .get(&self.base_url)
            .query(&query_params)
            .send()
            .await?;

        let body_bytes = response.bytes().await?;
        
        let text = if params.gzip.is_some() {
            let mut decoder = GzDecoder::new(&body_bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;
            decompressed
        } else {
            String::from_utf8(body_bytes.to_vec())?
        };
        
        let out_format = params.out.as_deref().unwrap_or("yaml");
        
        match out_format {
            "json" => {
                let data: serde_json::Value = serde_json::from_str(&text)?;
                let rankings = Self::parse_json_response(data)?;
                Ok(rankings)
            }
            "yaml" => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let rankings = Self::parse_yaml_response(data)?;
                Ok(rankings)
            }
            "php" => {
                return Err("PHP serialize format is not implemented".into());
            }
            "jsonp" => {
                return Err("JSONP format is not implemented".into());
            }
            _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let rankings = Self::parse_yaml_response(data)?;
                Ok(rankings)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value) -> Result<NarouRankingApiResponse, Box<dyn std::error::Error>> {
        if let Some(arr) = data.as_array() {
            let rankings: Result<Vec<_>, _> = arr
                .iter()
                .map(|v| serde_json::from_value::<RankingEntry>(v.clone()))
                .collect();

            Ok(NarouRankingApiResponse {
                rankings: rankings?,
            })
        } else if let Some(obj) = data.as_object() {
            if obj.contains_key("ncode") && obj.contains_key("pt") && obj.contains_key("rank") {
                let entry = serde_json::from_value::<RankingEntry>(data)?;
                Ok(NarouRankingApiResponse {
                    rankings: vec![entry],
                })
            } else {
                Ok(NarouRankingApiResponse {
                    rankings: vec![],
                })
            }
        } else {
            Ok(NarouRankingApiResponse {
                rankings: vec![],
            })
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value) -> Result<NarouRankingApiResponse, Box<dyn std::error::Error>> {
        if let Some(seq) = data.as_sequence() {
            let rankings: Result<Vec<_>, _> = seq
                .iter()
                .map(|v| serde_yaml::from_value::<RankingEntry>(v.clone()))
                .collect();

            Ok(NarouRankingApiResponse {
                rankings: rankings?,
            })
        } else if let Some(map) = data.as_mapping() {
            if map.contains_key("ncode") && map.contains_key("pt") && map.contains_key("rank") {
                let entry = serde_yaml::from_value::<RankingEntry>(data)?;
                Ok(NarouRankingApiResponse {
                    rankings: vec![entry],
                })
            } else {
                Ok(NarouRankingApiResponse {
                    rankings: vec![],
                })
            }
        } else {
            Ok(NarouRankingApiResponse {
                rankings: vec![],
            })
        }
    }
}

impl NarouRankingApiRequest {
    pub fn new(rtype: String) -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            rtype,
            libtype: None,
            callback: None,
        }
    }

    pub fn daily(date: &str) -> Self {
        Self::new(format!("{}-d", date))
    }

    pub fn weekly(date: &str) -> Self {
        Self::new(format!("{}-w", date))
    }

    pub fn monthly(date: &str) -> Self {
        Self::new(format!("{}-m", date))
    }

    pub fn quarterly(date: &str) -> Self {
        Self::new(format!("{}-q", date))
    }
}