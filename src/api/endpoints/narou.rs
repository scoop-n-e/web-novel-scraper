use crate::api::common::{ApiClient, ApiResponse, ApiRequest};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub of: Option<String>,
    pub lim: Option<u32>,
    pub st: Option<u32>,
    pub order: Option<String>,
    pub word: Option<String>,
    pub notword: Option<String>,
    pub title: Option<u8>,
    pub ex: Option<u8>,
    pub keyword: Option<u8>,
    pub wname: Option<u8>,
    pub biggenre: Option<String>,
    pub notbiggenre: Option<String>,
    pub genre: Option<String>,
    pub notgenre: Option<String>,
    pub userid: Option<String>,
    pub isr15: Option<u8>,
    pub isbl: Option<u8>,
    pub isgl: Option<u8>,
    pub iszankoku: Option<u8>,
    pub istensei: Option<u8>,
    pub istenni: Option<u8>,
    pub istt: Option<u8>,
    pub notr15: Option<u8>,
    pub notbl: Option<u8>,
    pub notgl: Option<u8>,
    pub notzankoku: Option<u8>,
    pub nottensei: Option<u8>,
    pub nottenni: Option<u8>,
    pub minlen: Option<u32>,
    pub maxlen: Option<u32>,
    pub length: Option<String>,
    pub kaiwaritu: Option<String>,
    pub sasie: Option<String>,
    pub mintime: Option<u32>,
    pub maxtime: Option<u32>,
    pub time: Option<String>,
    pub ncode: Option<String>,
    pub r#type: Option<String>,
    pub buntai: Option<String>,
    pub stop: Option<u8>,
    pub lastup: Option<String>,
    pub lastupdate: Option<String>,
    pub ispickup: Option<u8>,
    pub libtype: Option<u8>,
    pub opt: Option<String>,
    pub callback: Option<String>,
    pub updatetype: Option<u8>,
}

impl NarouRequest {
    pub fn new() -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            of: None,
            lim: None,
            st: None,
            order: None,
            word: None,
            notword: None,
            title: None,
            ex: None,
            keyword: None,
            wname: None,
            biggenre: None,
            notbiggenre: None,
            genre: None,
            notgenre: None,
            userid: None,
            isr15: None,
            isbl: None,
            isgl: None,
            iszankoku: None,
            istensei: None,
            istenni: None,
            istt: None,
            notr15: None,
            notbl: None,
            notgl: None,
            notzankoku: None,
            nottensei: None,
            nottenni: None,
            minlen: None,
            maxlen: None,
            length: None,
            kaiwaritu: None,
            sasie: None,
            mintime: None,
            maxtime: None,
            time: None,
            ncode: None,
            r#type: None,
            buntai: None,
            stop: None,
            lastup: None,
            lastupdate: None,
            ispickup: None,
            libtype: None,
            opt: None,
            callback: None,
            updatetype: None,
        }
    }
}

impl crate::api::common::request::ApiRequest for NarouRequest {
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
        if let Some(ref value) = self.title { params.push(("title".to_string(), value.to_string())); }
        if let Some(ref value) = self.ex { params.push(("ex".to_string(), value.to_string())); }
        if let Some(ref value) = self.keyword { params.push(("keyword".to_string(), value.to_string())); }
        if let Some(ref value) = self.wname { params.push(("wname".to_string(), value.to_string())); }
        if let Some(ref value) = self.biggenre { params.push(("biggenre".to_string(), value.clone())); }
        if let Some(ref value) = self.notbiggenre { params.push(("notbiggenre".to_string(), value.clone())); }
        if let Some(ref value) = self.genre { params.push(("genre".to_string(), value.clone())); }
        if let Some(ref value) = self.notgenre { params.push(("notgenre".to_string(), value.clone())); }
        if let Some(ref value) = self.userid { params.push(("userid".to_string(), value.clone())); }
        if let Some(ref value) = self.isr15 { params.push(("isr15".to_string(), value.to_string())); }
        if let Some(ref value) = self.isbl { params.push(("isbl".to_string(), value.to_string())); }
        if let Some(ref value) = self.isgl { params.push(("isgl".to_string(), value.to_string())); }
        if let Some(ref value) = self.iszankoku { params.push(("iszankoku".to_string(), value.to_string())); }
        if let Some(ref value) = self.istensei { params.push(("istensei".to_string(), value.to_string())); }
        if let Some(ref value) = self.istenni { params.push(("istenni".to_string(), value.to_string())); }
        if let Some(ref value) = self.istt { params.push(("istt".to_string(), value.to_string())); }
        if let Some(ref value) = self.notr15 { params.push(("notr15".to_string(), value.to_string())); }
        if let Some(ref value) = self.notbl { params.push(("notbl".to_string(), value.to_string())); }
        if let Some(ref value) = self.notgl { params.push(("notgl".to_string(), value.to_string())); }
        if let Some(ref value) = self.notzankoku { params.push(("notzankoku".to_string(), value.to_string())); }
        if let Some(ref value) = self.nottensei { params.push(("nottensei".to_string(), value.to_string())); }
        if let Some(ref value) = self.nottenni { params.push(("nottenni".to_string(), value.to_string())); }
        if let Some(ref value) = self.minlen { params.push(("minlen".to_string(), value.to_string())); }
        if let Some(ref value) = self.maxlen { params.push(("maxlen".to_string(), value.to_string())); }
        if let Some(ref value) = self.length { params.push(("length".to_string(), value.clone())); }
        if let Some(ref value) = self.kaiwaritu { params.push(("kaiwaritu".to_string(), value.clone())); }
        if let Some(ref value) = self.sasie { params.push(("sasie".to_string(), value.clone())); }
        if let Some(ref value) = self.mintime { params.push(("mintime".to_string(), value.to_string())); }
        if let Some(ref value) = self.maxtime { params.push(("maxtime".to_string(), value.to_string())); }
        if let Some(ref value) = self.time { params.push(("time".to_string(), value.clone())); }
        if let Some(ref value) = self.ncode { params.push(("ncode".to_string(), value.clone())); }
        if let Some(ref value) = self.r#type { params.push(("type".to_string(), value.clone())); }
        if let Some(ref value) = self.buntai { params.push(("buntai".to_string(), value.clone())); }
        if let Some(ref value) = self.stop { params.push(("stop".to_string(), value.to_string())); }
        if let Some(ref value) = self.lastup { params.push(("lastup".to_string(), value.clone())); }
        if let Some(ref value) = self.lastupdate { params.push(("lastupdate".to_string(), value.clone())); }
        if let Some(ref value) = self.ispickup { params.push(("ispickup".to_string(), value.to_string())); }
        if let Some(ref value) = self.libtype { params.push(("libtype".to_string(), value.to_string())); }
        if let Some(ref value) = self.opt { params.push(("opt".to_string(), value.clone())); }
        if let Some(ref callback) = self.callback { params.push(("callback".to_string(), callback.clone())); }
        if let Some(ref value) = self.updatetype { params.push(("updatetype".to_string(), value.to_string())); }
        
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
pub struct NarouNovelInfo {
    pub title: Option<String>,
    pub ncode: Option<String>,
    pub userid: Option<u32>,
    pub writer: Option<String>,
    pub story: Option<String>,
    pub biggenre: Option<u8>,
    pub genre: Option<u16>,
    pub gensaku: Option<String>,
    pub keyword: Option<String>,
    pub general_firstup: Option<String>,
    pub general_lastup: Option<String>,
    pub novel_type: Option<u8>,
    pub end: Option<u8>,
    pub general_all_no: Option<u32>,
    pub length: Option<u32>,
    pub time: Option<u32>,
    pub isstop: Option<u8>,
    pub isr15: Option<u8>,
    pub isbl: Option<u8>,
    pub isgl: Option<u8>,
    pub iszankoku: Option<u8>,
    pub istensei: Option<u8>,
    pub istenni: Option<u8>,
    pub global_point: Option<u32>,
    pub daily_point: Option<u32>,
    pub weekly_point: Option<u32>,
    pub monthly_point: Option<u32>,
    pub quarter_point: Option<u32>,
    pub yearly_point: Option<u32>,
    pub fav_novel_cnt: Option<u32>,
    pub impression_cnt: Option<u32>,
    pub review_cnt: Option<u32>,
    pub all_point: Option<u32>,
    pub all_hyoka_cnt: Option<u32>,
    pub sasie_cnt: Option<u32>,
    pub kaiwaritu: Option<u8>,
    pub novelupdated_at: Option<String>,
    pub updated_at: Option<String>,
    pub weekly_unique: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouResponse {
    pub allcount: Option<u32>,
    #[serde(flatten)]
    pub novels: Vec<NarouNovelInfo>,
}

impl ApiResponse for NarouResponse {
    fn from_json(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_json::Value = serde_json::from_slice(data)?;
        
        if let Some(arr) = value.as_array() {
            let mut allcount = None;
            let mut novels = Vec::new();
            
            for (i, item) in arr.iter().enumerate() {
                if i == 0 {
                    if let Some(obj) = item.as_object() {
                        if let Some(count) = obj.get("allcount") {
                            allcount = count.as_u64().map(|v| v as u32);
                            continue;
                        }
                    }
                }
                
                let novel = serde_json::from_value::<NarouNovelInfo>(item.clone())
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?;
                novels.push(novel);
            }
            
            Ok(NarouResponse { allcount, novels })
        } else {
            Ok(NarouResponse {
                allcount: None,
                novels: vec![],
            })
        }
    }

    fn from_yaml(data: &[u8]) -> crate::api::common::Result<Self> {
        let value: serde_yaml::Value = serde_yaml::from_slice(data)?;
        
        if let Some(seq) = value.as_sequence() {
            let mut allcount = None;
            let mut novels = Vec::new();
            
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
                
                let novel = serde_yaml::from_value::<NarouNovelInfo>(item.clone())
                    .map_err(|e| crate::api::common::ApiError::Deserialization(e.to_string()))?;
                novels.push(novel);
            }
            
            Ok(NarouResponse { allcount, novels })
        } else {
            Ok(NarouResponse {
                allcount: None,
                novels: vec![],
            })
        }
    }
}

pub struct NarouApiClient;

#[async_trait]
impl ApiClient for NarouApiClient {
    type Request = NarouRequest;
    type Response = NarouResponse;

    fn base_url(&self) -> &str {
        "https://api.syosetu.com/novelapi/api/"
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

impl NarouApiClient {
    fn map_of_fields(&self, of_fields: &str) -> String {
        let fields: Vec<&str> = of_fields.split('-').collect();
        let mut mapped_fields = Vec::new();
        
        for field in fields {
            let mapped = match field {
                "t" => "title",
                "n" => "ncode",
                "u" => "userid",
                "w" => "writer",
                "s" => "story",
                "bg" => "biggenre",
                "g" => "genre",
                "gp" => "global_point",
                "dp" => "daily_point",
                "wp" => "weekly_point",
                "mp" => "monthly_point",
                "qp" => "quarter_point",
                "yp" => "yearly_point",
                "f" => "fav_novel_cnt",
                "im" => "impression_cnt",
                "r" => "review_cnt",
                "a" => "all_point",
                "ah" => "all_hyoka_cnt",
                "sa" => "sasie_cnt",
                "ka" => "kaiwaritu",
                "nu" => "novelupdated_at",
                "ua" => "updated_at",
                "nt" => "novel_type",
                "e" => "end",
                "ga" => "general_all_no",
                "l" => "length",
                "ti" => "time",
                "i" => "isstop",
                "ir" => "isr15",
                "ibl" => "isbl",
                "igl" => "isgl",
                "izk" => "iszankoku",
                "its" => "istensei",
                "iti" => "istenni",
                "gf" => "general_firstup",
                "gl" => "general_lastup",
                "k" => "keyword",
                "gs" => "gensaku",
                "wu" => "weekly_unique",
                _ => field,
            };
            mapped_fields.push(mapped);
        }
        
        mapped_fields.join("-")
    }
}