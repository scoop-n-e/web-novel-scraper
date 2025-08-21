use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouHallOfFameApiRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub ncode: String,
    pub libtype: Option<u8>,
    pub callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouHallOfFameApiResponse {
    #[serde(flatten)]
    pub rankings: Vec<HallOfFameEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallOfFameEntry {
    pub rtype: String,
    pub pt: u32,
    pub rank: u16,
}

pub struct NarouHallOfFameApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NarouHallOfFameApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.syosetu.com/rank/rankin/".to_string(),
        }
    }

    pub async fn get_rankings(&self, params: &NarouHallOfFameApiRequest) -> Result<NarouHallOfFameApiResponse, Box<dyn std::error::Error>> {
        let mut query_params = Vec::new();

        query_params.push(("ncode", params.ncode.clone()));

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

        let text = response.text().await?;
        
        let out_format = params.out.as_deref().unwrap_or("yaml");
        
        match out_format {
            "json" => {
                let data: serde_json::Value = serde_json::from_str(&text)?;
                let rankings = Self::parse_json_response(data)?;
                Ok(rankings)
            }
            "yaml" | _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let rankings = Self::parse_yaml_response(data)?;
                Ok(rankings)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value) -> Result<NarouHallOfFameApiResponse, Box<dyn std::error::Error>> {
        if let Some(arr) = data.as_array() {
            let rankings: Result<Vec<_>, _> = arr
                .iter()
                .map(|v| serde_json::from_value::<HallOfFameEntry>(v.clone()))
                .collect();

            Ok(NarouHallOfFameApiResponse {
                rankings: rankings?,
            })
        } else if let Some(obj) = data.as_object() {
            if obj.contains_key("rtype") && obj.contains_key("pt") && obj.contains_key("rank") {
                let entry = serde_json::from_value::<HallOfFameEntry>(data)?;
                Ok(NarouHallOfFameApiResponse {
                    rankings: vec![entry],
                })
            } else {
                Ok(NarouHallOfFameApiResponse {
                    rankings: vec![],
                })
            }
        } else {
            Ok(NarouHallOfFameApiResponse {
                rankings: vec![],
            })
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value) -> Result<NarouHallOfFameApiResponse, Box<dyn std::error::Error>> {
        if let Some(seq) = data.as_sequence() {
            let rankings: Result<Vec<_>, _> = seq
                .iter()
                .map(|v| serde_yaml::from_value::<HallOfFameEntry>(v.clone()))
                .collect();

            Ok(NarouHallOfFameApiResponse {
                rankings: rankings?,
            })
        } else if let Some(map) = data.as_mapping() {
            if map.contains_key("rtype") && map.contains_key("pt") && map.contains_key("rank") {
                let entry = serde_yaml::from_value::<HallOfFameEntry>(data)?;
                Ok(NarouHallOfFameApiResponse {
                    rankings: vec![entry],
                })
            } else {
                Ok(NarouHallOfFameApiResponse {
                    rankings: vec![],
                })
            }
        } else {
            Ok(NarouHallOfFameApiResponse {
                rankings: vec![],
            })
        }
    }
}

impl NarouHallOfFameApiRequest {
    pub fn new(ncode: String) -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            ncode,
            libtype: None,
            callback: None,
        }
    }
}