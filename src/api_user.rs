use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouUserApiRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub of: Option<String>,
    pub lim: Option<u32>,
    pub st: Option<u32>,
    pub order: Option<String>,
    pub libtype: Option<u8>,
    pub word: Option<String>,
    pub notword: Option<String>,
    pub userid: Option<u32>,
    pub name1st: Option<String>,
    pub minnovel: Option<u32>,
    pub maxnovel: Option<u32>,
    pub minreview: Option<u32>,
    pub maxreview: Option<u32>,
    pub callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouUserApiResponse {
    pub allcount: u32,
    #[serde(flatten)]
    pub users: Vec<NarouUserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouUserInfo {
    pub userid: u32,
    pub name: String,
    pub yomikata: String,
    pub name1st: Option<String>,
    pub novel_cnt: u32,
    pub review_cnt: u32,
    pub novel_length: u64,
    pub sum_global_point: u64,
}

pub struct NarouUserApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NarouUserApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.syosetu.com/userapi/api/".to_string(),
        }
    }

    pub async fn search(&self, params: &NarouUserApiRequest) -> Result<NarouUserApiResponse, Box<dyn std::error::Error>> {
        let mut query_params = Vec::new();

        if let Some(v) = &params.gzip {
            query_params.push(("gzip", v.to_string()));
        }
        if let Some(v) = &params.out {
            query_params.push(("out", v.clone()));
        }
        if let Some(v) = &params.of {
            query_params.push(("of", v.clone()));
        }
        if let Some(v) = &params.lim {
            query_params.push(("lim", v.to_string()));
        }
        if let Some(v) = &params.st {
            query_params.push(("st", v.to_string()));
        }
        if let Some(v) = &params.order {
            query_params.push(("order", v.clone()));
        }
        if let Some(v) = &params.libtype {
            query_params.push(("libtype", v.to_string()));
        }
        if let Some(v) = &params.word {
            query_params.push(("word", v.clone()));
        }
        if let Some(v) = &params.notword {
            query_params.push(("notword", v.clone()));
        }
        if let Some(v) = &params.userid {
            query_params.push(("userid", v.to_string()));
        }
        if let Some(v) = &params.name1st {
            query_params.push(("name1st", v.clone()));
        }
        if let Some(v) = &params.minnovel {
            query_params.push(("minnovel", v.to_string()));
        }
        if let Some(v) = &params.maxnovel {
            query_params.push(("maxnovel", v.to_string()));
        }
        if let Some(v) = &params.minreview {
            query_params.push(("minreview", v.to_string()));
        }
        if let Some(v) = &params.maxreview {
            query_params.push(("maxreview", v.to_string()));
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
                let users = Self::parse_json_response(data)?;
                Ok(users)
            }
            "yaml" | _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let users = Self::parse_yaml_response(data)?;
                Ok(users)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value) -> Result<NarouUserApiResponse, Box<dyn std::error::Error>> {
        if let Some(arr) = data.as_array() {
            if arr.is_empty() {
                return Ok(NarouUserApiResponse {
                    allcount: 0,
                    users: vec![],
                });
            }

            let allcount = arr[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let users: Result<Vec<_>, _> = arr[1..]
                .iter()
                .map(|v| serde_json::from_value::<NarouUserInfo>(v.clone()))
                .collect();

            Ok(NarouUserApiResponse {
                allcount,
                users: users?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value) -> Result<NarouUserApiResponse, Box<dyn std::error::Error>> {
        if let Some(seq) = data.as_sequence() {
            if seq.is_empty() {
                return Ok(NarouUserApiResponse {
                    allcount: 0,
                    users: vec![],
                });
            }

            let allcount = seq[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let users: Result<Vec<_>, _> = seq[1..]
                .iter()
                .map(|v| serde_yaml::from_value::<NarouUserInfo>(v.clone()))
                .collect();

            Ok(NarouUserApiResponse {
                allcount,
                users: users?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }
}

impl Default for NarouUserApiRequest {
    fn default() -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            of: None,
            lim: Some(20),
            st: None,
            order: None,
            libtype: None,
            word: None,
            notword: None,
            userid: None,
            name1st: None,
            minnovel: None,
            maxnovel: None,
            minreview: None,
            maxreview: None,
            callback: None,
        }
    }
}