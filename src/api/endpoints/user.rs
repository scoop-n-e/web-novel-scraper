use crate::api::common::{ApiClient, ApiResponse, ApiRequest};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
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

impl UserRequest {
    pub fn new() -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            of: None,
            lim: None,
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

impl ApiRequest for UserRequest {
    fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        
        if let Some(gzip) = self.gzip { params.push(("gzip".to_string(), gzip.to_string())); }
        if let Some(ref out) = self.out { params.push(("out".to_string(), out.clone())); }
        if let Some(ref value) = self.of { params.push(("of".to_string(), value.clone())); }
        if let Some(ref value) = self.lim { params.push(("lim".to_string(), value.to_string())); }
        if let Some(ref value) = self.st { params.push(("st".to_string(), value.to_string())); }
        if let Some(ref value) = self.order { params.push(("order".to_string(), value.clone())); }
        if let Some(ref value) = self.word { params.push(("word".to_string(), value.clone())); }
        if let Some(ref value) = self.notword { params.push(("notword".to_string(), value.clone())); }
        if let Some(ref value) = self.userid { params.push(("userid".to_string(), value.to_string())); }
        if let Some(ref value) = self.name1st { params.push(("name1st".to_string(), value.clone())); }
        if let Some(ref value) = self.minnovel { params.push(("minnovel".to_string(), value.to_string())); }
        if let Some(ref value) = self.maxnovel { params.push(("maxnovel".to_string(), value.to_string())); }
        if let Some(ref value) = self.minreview { params.push(("minreview".to_string(), value.to_string())); }
        if let Some(ref value) = self.maxreview { params.push(("maxreview".to_string(), value.to_string())); }
        if let Some(ref value) = self.libtype { params.push(("libtype".to_string(), value.to_string())); }
        if let Some(ref callback) = self.callback { params.push(("callback".to_string(), callback.clone())); }
        
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub allcount: Option<u32>,
    #[serde(flatten)]
    pub users: Vec<NarouUserInfo>,
}

impl ApiResponse for UserResponse {
    fn from_json(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_json::Value = serde_json::from_slice(data)?;
        
        if let Some(arr) = value.as_array() {
            let mut allcount = None;
            let mut users = Vec::new();
            
            for (i, item) in arr.iter().enumerate() {
                if i == 0 {
                    if let Some(obj) = item.as_object() {
                        if let Some(count) = obj.get("allcount") {
                            allcount = count.as_u64().map(|v| v as u32);
                            continue;
                        }
                    }
                }
                
                let user = serde_json::from_value::<NarouUserInfo>(item.clone())
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?;
                users.push(user);
            }
            
            Ok(UserResponse { allcount, users })
        } else {
            Ok(UserResponse {
                allcount: None,
                users: vec![],
            })
        }
    }

    fn from_yaml(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_yaml::Value = serde_yaml::from_slice(data)?;
        
        if let Some(seq) = value.as_sequence() {
            let mut allcount = None;
            let mut users = Vec::new();
            
            for (i, item) in seq.iter().enumerate() {
                if i == 0 {
                    if let Some(map) = item.as_mapping() {
                        let allcount_key = serde_yaml::Value::String("allcount".to_string());
                        if let Some(count) = map.get(&allcount_key) {
                            allcount = count.as_u64().map(|v| v as u32);
                            continue;
                        }
                    }
                }
                
                let user = serde_yaml::from_value::<NarouUserInfo>(item.clone())
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?;
                users.push(user);
            }
            
            Ok(UserResponse { allcount, users })
        } else {
            Ok(UserResponse {
                allcount: None,
                users: vec![],
            })
        }
    }
}

pub struct UserApiClient;

#[async_trait]
impl ApiClient for UserApiClient {
    type Request = UserRequest;
    type Response = UserResponse;

    fn base_url(&self) -> &str {
        "https://api.syosetu.com/userapi/api/"
    }

    fn build_query_params(&self, request: &Self::Request) -> Vec<(String, String)> {
        let mut params = request.to_query_params();
        
        if let Some(ref of_fields) = request.of {
            let mapped = self.map_of_fields(of_fields);
            if let Some(index) = params.iter().position(|(k, _)| k == "of") {
                params[index].1 = mapped;
            }
        }
        
        params
    }
}

impl UserApiClient {
    fn map_of_fields(&self, of_fields: &str) -> String {
        let fields: Vec<&str> = of_fields.split('-').collect();
        let mut mapped_fields = Vec::new();
        
        for field in fields {
            let mapped = match field {
                "u" => "userid",
                "n" => "name",
                "y" => "yomikata",
                "n1" => "name1st",
                "nc" => "novel_cnt",
                "rc" => "review_cnt",
                "nl" => "novel_length",
                "sg" => "sum_global_point",
                _ => field,
            };
            mapped_fields.push(mapped);
        }
        
        mapped_fields.join("-")
    }
}