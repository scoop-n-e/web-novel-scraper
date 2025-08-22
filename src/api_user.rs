use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;
use flate2::read::GzDecoder;
use std::io::Read;

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
    pub userid: Option<u32>,
    pub name: Option<String>,
    pub yomikata: Option<String>,
    pub name1st: Option<String>,
    pub novel_cnt: Option<u32>,
    pub review_cnt: Option<u32>,
    pub novel_length: Option<u64>,
    pub sum_global_point: Option<u64>,
}

pub struct NarouUserApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NarouUserApiClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .unwrap();
        
        Self {
            client,
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
                let users = Self::parse_json_response(data, &params.of)?;
                Ok(users)
            }
            "yaml" => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let users = Self::parse_yaml_response(data, &params.of)?;
                Ok(users)
            }
            "php" => {
                return Err("PHP serialize format is not implemented".into());
            }
            "jsonp" => {
                return Err("JSONP format is not implemented".into());
            }
            _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let users = Self::parse_yaml_response(data, &params.of)?;
                Ok(users)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value, of_param: &Option<String>) -> Result<NarouUserApiResponse, Box<dyn std::error::Error>> {
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
                .map(|v| Self::parse_user_with_of(v.clone(), of_param))
                .collect();

            Ok(NarouUserApiResponse {
                allcount,
                users: users?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value, of_param: &Option<String>) -> Result<NarouUserApiResponse, Box<dyn std::error::Error>> {
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
                .map(|v| {
                    let json_value = serde_json::to_value(v)?;
                    Self::parse_user_with_of(json_value, of_param)
                })
                .collect();

            Ok(NarouUserApiResponse {
                allcount,
                users: users?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_user_with_of(value: serde_json::Value, of_param: &Option<String>) -> Result<NarouUserInfo, Box<dyn std::error::Error>> {
        if of_param.is_none() {
            return Ok(serde_json::from_value::<NarouUserInfo>(value)?);
        }

        let of_fields = of_param.as_ref().unwrap();
        let fields: Vec<&str> = of_fields.split('-').collect();
        
        let mut user = NarouUserInfo {
            userid: None,
            name: None,
            yomikata: None,
            name1st: None,
            novel_cnt: None,
            review_cnt: None,
            novel_length: None,
            sum_global_point: None,
        };

        for field in fields {
            match field {
                "u" => user.userid = value["userid"].as_u64().map(|n| n as u32),
                "n" => user.name = value["name"].as_str().map(|s| s.to_string()),
                "y" => user.yomikata = value["yomikata"].as_str().map(|s| s.to_string()),
                "n1" => user.name1st = value["name1st"].as_str().map(|s| s.to_string()),
                "nc" => user.novel_cnt = value["novel_cnt"].as_u64().map(|n| n as u32),
                "rc" => user.review_cnt = value["review_cnt"].as_u64().map(|n| n as u32),
                "nl" => user.novel_length = value["novel_length"].as_u64(),
                "sg" => user.sum_global_point = value["sum_global_point"].as_u64(),
                _ => {}
            }
        }

        Ok(user)
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