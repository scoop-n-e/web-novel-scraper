use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NocturneNovelApiRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub of: Option<String>,
    pub lim: Option<u32>,
    pub st: Option<u32>,
    pub order: Option<String>,
    pub libtype: Option<u8>,
    pub updatetype: Option<u8>,
    pub word: Option<String>,
    pub notword: Option<String>,
    pub title: Option<u8>,
    pub ex: Option<u8>,
    pub keyword: Option<u8>,
    pub wname: Option<u8>,
    pub nocgenre: Option<String>,
    pub notnocgenre: Option<String>,
    pub xid: Option<String>,
    pub isbl: Option<u8>,
    pub isgl: Option<u8>,
    pub iszankoku: Option<u8>,
    pub istensei: Option<u8>,
    pub istenni: Option<u8>,
    pub istt: Option<u8>,
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
    pub opt: Option<String>,
    pub callback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NocturneNovelApiResponse {
    pub allcount: u32,
    #[serde(flatten)]
    pub novels: Vec<NocturneNovelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NocturneNovelInfo {
    pub title: String,
    pub ncode: String,
    pub writer: String,
    pub story: String,
    pub nocgenre: u8,
    pub gensaku: String,
    pub keyword: String,
    pub general_firstup: String,
    pub general_lastup: String,
    pub novel_type: u8,
    pub end: u8,
    pub general_all_no: u32,
    pub length: u32,
    pub time: u32,
    pub isstop: u8,
    pub isbl: u8,
    pub isgl: u8,
    pub iszankoku: u8,
    pub istensei: u8,
    pub istenni: u8,
    pub global_point: u32,
    pub daily_point: u32,
    pub weekly_point: u32,
    pub monthly_point: u32,
    pub quarter_point: u32,
    pub yearly_point: u32,
    pub fav_novel_cnt: u32,
    pub impression_cnt: u32,
    pub review_cnt: u32,
    pub all_point: u32,
    pub all_hyoka_cnt: u32,
    pub sasie_cnt: u32,
    pub kaiwaritu: u8,
    pub novelupdated_at: String,
    pub updated_at: String,
    pub weekly_unique: Option<u32>,
}

pub struct NocturneNovelApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NocturneNovelApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.syosetu.com/novel18api/api/".to_string(),
        }
    }

    pub async fn search(&self, params: &NocturneNovelApiRequest) -> Result<NocturneNovelApiResponse, Box<dyn std::error::Error>> {
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
        if let Some(v) = &params.updatetype {
            query_params.push(("updatetype", v.to_string()));
        }
        if let Some(v) = &params.word {
            query_params.push(("word", v.clone()));
        }
        if let Some(v) = &params.notword {
            query_params.push(("notword", v.clone()));
        }
        if let Some(v) = &params.title {
            query_params.push(("title", v.to_string()));
        }
        if let Some(v) = &params.ex {
            query_params.push(("ex", v.to_string()));
        }
        if let Some(v) = &params.keyword {
            query_params.push(("keyword", v.to_string()));
        }
        if let Some(v) = &params.wname {
            query_params.push(("wname", v.to_string()));
        }
        if let Some(v) = &params.nocgenre {
            query_params.push(("nocgenre", v.clone()));
        }
        if let Some(v) = &params.notnocgenre {
            query_params.push(("notnocgenre", v.clone()));
        }
        if let Some(v) = &params.xid {
            query_params.push(("xid", v.clone()));
        }
        if let Some(v) = &params.isbl {
            query_params.push(("isbl", v.to_string()));
        }
        if let Some(v) = &params.isgl {
            query_params.push(("isgl", v.to_string()));
        }
        if let Some(v) = &params.iszankoku {
            query_params.push(("iszankoku", v.to_string()));
        }
        if let Some(v) = &params.istensei {
            query_params.push(("istensei", v.to_string()));
        }
        if let Some(v) = &params.istenni {
            query_params.push(("istenni", v.to_string()));
        }
        if let Some(v) = &params.istt {
            query_params.push(("istt", v.to_string()));
        }
        if let Some(v) = &params.notbl {
            query_params.push(("notbl", v.to_string()));
        }
        if let Some(v) = &params.notgl {
            query_params.push(("notgl", v.to_string()));
        }
        if let Some(v) = &params.notzankoku {
            query_params.push(("notzankoku", v.to_string()));
        }
        if let Some(v) = &params.nottensei {
            query_params.push(("nottensei", v.to_string()));
        }
        if let Some(v) = &params.nottenni {
            query_params.push(("nottenni", v.to_string()));
        }
        if let Some(v) = &params.minlen {
            query_params.push(("minlen", v.to_string()));
        }
        if let Some(v) = &params.maxlen {
            query_params.push(("maxlen", v.to_string()));
        }
        if let Some(v) = &params.length {
            query_params.push(("length", v.clone()));
        }
        if let Some(v) = &params.kaiwaritu {
            query_params.push(("kaiwaritu", v.clone()));
        }
        if let Some(v) = &params.sasie {
            query_params.push(("sasie", v.clone()));
        }
        if let Some(v) = &params.mintime {
            query_params.push(("mintime", v.to_string()));
        }
        if let Some(v) = &params.maxtime {
            query_params.push(("maxtime", v.to_string()));
        }
        if let Some(v) = &params.time {
            query_params.push(("time", v.clone()));
        }
        if let Some(v) = &params.ncode {
            query_params.push(("ncode", v.clone()));
        }
        if let Some(v) = &params.r#type {
            query_params.push(("type", v.clone()));
        }
        if let Some(v) = &params.buntai {
            query_params.push(("buntai", v.clone()));
        }
        if let Some(v) = &params.stop {
            query_params.push(("stop", v.to_string()));
        }
        if let Some(v) = &params.lastup {
            query_params.push(("lastup", v.clone()));
        }
        if let Some(v) = &params.lastupdate {
            query_params.push(("lastupdate", v.clone()));
        }
        if let Some(v) = &params.ispickup {
            query_params.push(("ispickup", v.to_string()));
        }
        if let Some(v) = &params.opt {
            query_params.push(("opt", v.clone()));
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
                let novels = Self::parse_json_response(data)?;
                Ok(novels)
            }
            "yaml" | _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let novels = Self::parse_yaml_response(data)?;
                Ok(novels)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value) -> Result<NocturneNovelApiResponse, Box<dyn std::error::Error>> {
        if let Some(arr) = data.as_array() {
            if arr.is_empty() {
                return Ok(NocturneNovelApiResponse {
                    allcount: 0,
                    novels: vec![],
                });
            }

            let allcount = arr[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let novels: Result<Vec<_>, _> = arr[1..]
                .iter()
                .map(|v| serde_json::from_value::<NocturneNovelInfo>(v.clone()))
                .collect();

            Ok(NocturneNovelApiResponse {
                allcount,
                novels: novels?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value) -> Result<NocturneNovelApiResponse, Box<dyn std::error::Error>> {
        if let Some(seq) = data.as_sequence() {
            if seq.is_empty() {
                return Ok(NocturneNovelApiResponse {
                    allcount: 0,
                    novels: vec![],
                });
            }

            let allcount = seq[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let novels: Result<Vec<_>, _> = seq[1..]
                .iter()
                .map(|v| serde_yaml::from_value::<NocturneNovelInfo>(v.clone()))
                .collect();

            Ok(NocturneNovelApiResponse {
                allcount,
                novels: novels?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }
}

impl Default for NocturneNovelApiRequest {
    fn default() -> Self {
        Self {
            gzip: None,
            out: Some("json".to_string()),
            of: None,
            lim: Some(20),
            st: None,
            order: None,
            libtype: None,
            updatetype: None,
            word: None,
            notword: None,
            title: None,
            ex: None,
            keyword: None,
            wname: None,
            nocgenre: None,
            notnocgenre: None,
            xid: None,
            isbl: None,
            isgl: None,
            iszankoku: None,
            istensei: None,
            istenni: None,
            istt: None,
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
            opt: None,
            callback: None,
        }
    }
}