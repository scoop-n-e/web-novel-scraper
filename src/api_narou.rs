use serde::{Deserialize, Serialize};
use serde_yaml;
use serde_json;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouNovelApiRequest {
    pub gzip: Option<u8>,
    pub out: Option<String>,
    pub of: Option<String>,
    pub lim: Option<u32>,
    pub st: Option<u32>,
    pub order: Option<String>,
    pub libtype: Option<u8>,
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
    pub opt: Option<String>,
    pub callback: Option<String>,
    pub updatetype: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouNovelApiResponse {
    pub allcount: u32,
    #[serde(flatten)]
    pub novels: Vec<NarouNovelInfo>,
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

pub struct NarouNovelApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl NarouNovelApiClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .unwrap();
        
        Self {
            client,
            base_url: "https://api.syosetu.com/novelapi/api/".to_string(),
        }
    }

    pub async fn search(&self, params: &NarouNovelApiRequest) -> Result<NarouNovelApiResponse, Box<dyn std::error::Error>> {
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
        if let Some(v) = &params.biggenre {
            query_params.push(("biggenre", v.clone()));
        }
        if let Some(v) = &params.notbiggenre {
            query_params.push(("notbiggenre", v.clone()));
        }
        if let Some(v) = &params.genre {
            query_params.push(("genre", v.clone()));
        }
        if let Some(v) = &params.notgenre {
            query_params.push(("notgenre", v.clone()));
        }
        if let Some(v) = &params.userid {
            query_params.push(("userid", v.clone()));
        }
        if let Some(v) = &params.isr15 {
            query_params.push(("isr15", v.to_string()));
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
        if let Some(v) = &params.notr15 {
            query_params.push(("notr15", v.to_string()));
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
        if let Some(v) = &params.updatetype {
            query_params.push(("updatetype", v.to_string()));
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
                let novels = Self::parse_json_response(data, &params.of)?;
                Ok(novels)
            }
            "yaml" => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let novels = Self::parse_yaml_response(data, &params.of)?;
                Ok(novels)
            }
            "php" => {
                return Err("PHP serialize format is not implemented".into());
            }
            "atom" => {
                return Err("Atom feed format is not implemented".into());
            }
            "jsonp" => {
                return Err("JSONP format is not implemented".into());
            }
            _ => {
                let data: serde_yaml::Value = serde_yaml::from_str(&text)?;
                let novels = Self::parse_yaml_response(data, &params.of)?;
                Ok(novels)
            }
        }
    }

    fn parse_json_response(data: serde_json::Value, of_param: &Option<String>) -> Result<NarouNovelApiResponse, Box<dyn std::error::Error>> {
        if let Some(arr) = data.as_array() {
            if arr.is_empty() {
                return Ok(NarouNovelApiResponse {
                    allcount: 0,
                    novels: vec![],
                });
            }

            let allcount = arr[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let novels: Result<Vec<_>, _> = arr[1..]
                .iter()
                .map(|v| Self::parse_novel_with_of(v.clone(), of_param))
                .collect();

            Ok(NarouNovelApiResponse {
                allcount,
                novels: novels?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_yaml_response(data: serde_yaml::Value, of_param: &Option<String>) -> Result<NarouNovelApiResponse, Box<dyn std::error::Error>> {
        if let Some(seq) = data.as_sequence() {
            if seq.is_empty() {
                return Ok(NarouNovelApiResponse {
                    allcount: 0,
                    novels: vec![],
                });
            }

            let allcount = seq[0]["allcount"]
                .as_u64()
                .ok_or("Missing allcount")? as u32;

            let novels: Result<Vec<_>, _> = seq[1..]
                .iter()
                .map(|v| {
                    let json_value = serde_json::to_value(v)?;
                    Self::parse_novel_with_of(json_value, of_param)
                })
                .collect();

            Ok(NarouNovelApiResponse {
                allcount,
                novels: novels?,
            })
        } else {
            Err("Invalid response format".into())
        }
    }

    fn parse_novel_with_of(value: serde_json::Value, of_param: &Option<String>) -> Result<NarouNovelInfo, Box<dyn std::error::Error>> {
        if of_param.is_none() {
            return Ok(serde_json::from_value::<NarouNovelInfo>(value)?);
        }

        let of_fields = of_param.as_ref().unwrap();
        let fields: Vec<&str> = of_fields.split('-').collect();
        
        let mut novel = NarouNovelInfo {
            title: None,
            ncode: None,
            userid: None,
            writer: None,
            story: None,
            biggenre: None,
            genre: None,
            gensaku: None,
            keyword: None,
            general_firstup: None,
            general_lastup: None,
            novel_type: None,
            end: None,
            general_all_no: None,
            length: None,
            time: None,
            isstop: None,
            isr15: None,
            isbl: None,
            isgl: None,
            iszankoku: None,
            istensei: None,
            istenni: None,
            global_point: None,
            daily_point: None,
            weekly_point: None,
            monthly_point: None,
            quarter_point: None,
            yearly_point: None,
            fav_novel_cnt: None,
            impression_cnt: None,
            review_cnt: None,
            all_point: None,
            all_hyoka_cnt: None,
            sasie_cnt: None,
            kaiwaritu: None,
            novelupdated_at: None,
            updated_at: None,
            weekly_unique: None,
        };

        for field in fields {
            match field {
                "t" => novel.title = value["title"].as_str().map(|s| s.to_string()),
                "n" => novel.ncode = value["ncode"].as_str().map(|s| s.to_string()),
                "u" => novel.userid = value["userid"].as_u64().map(|n| n as u32),
                "w" => novel.writer = value["writer"].as_str().map(|s| s.to_string()),
                "s" => novel.story = value["story"].as_str().map(|s| s.to_string()),
                "bg" => novel.biggenre = value["biggenre"].as_u64().map(|n| n as u8),
                "g" => novel.genre = value["genre"].as_u64().map(|n| n as u16),
                "k" => novel.keyword = value["keyword"].as_str().map(|s| s.to_string()),
                "gf" => novel.general_firstup = value["general_firstup"].as_str().map(|s| s.to_string()),
                "gl" => novel.general_lastup = value["general_lastup"].as_str().map(|s| s.to_string()),
                "nt" => novel.novel_type = value["noveltype"].as_u64().map(|n| n as u8)
                    .or_else(|| value["novel_type"].as_u64().map(|n| n as u8)),
                "e" => novel.end = value["end"].as_u64().map(|n| n as u8),
                "ga" => novel.general_all_no = value["general_all_no"].as_u64().map(|n| n as u32),
                "l" => novel.length = value["length"].as_u64().map(|n| n as u32),
                "ti" => novel.time = value["time"].as_u64().map(|n| n as u32),
                "i" => novel.isstop = value["isstop"].as_u64().map(|n| n as u8),
                "ir" => novel.isr15 = value["isr15"].as_u64().map(|n| n as u8),
                "ibl" => novel.isbl = value["isbl"].as_u64().map(|n| n as u8),
                "igl" => novel.isgl = value["isgl"].as_u64().map(|n| n as u8),
                "izk" => novel.iszankoku = value["iszankoku"].as_u64().map(|n| n as u8),
                "its" => novel.istensei = value["istensei"].as_u64().map(|n| n as u8),
                "iti" => novel.istenni = value["istenni"].as_u64().map(|n| n as u8),
                "gp" => novel.global_point = value["global_point"].as_u64().map(|n| n as u32),
                "dp" => novel.daily_point = value["daily_point"].as_u64().map(|n| n as u32),
                "wp" => novel.weekly_point = value["weekly_point"].as_u64().map(|n| n as u32),
                "mp" => novel.monthly_point = value["monthly_point"].as_u64().map(|n| n as u32),
                "qp" => novel.quarter_point = value["quarter_point"].as_u64().map(|n| n as u32),
                "yp" => novel.yearly_point = value["yearly_point"].as_u64().map(|n| n as u32),
                "f" => novel.fav_novel_cnt = value["fav_novel_cnt"].as_u64().map(|n| n as u32),
                "imp" => novel.impression_cnt = value["impression_cnt"].as_u64().map(|n| n as u32),
                "r" => novel.review_cnt = value["review_cnt"].as_u64().map(|n| n as u32),
                "a" => novel.all_point = value["all_point"].as_u64().map(|n| n as u32),
                "ah" => novel.all_hyoka_cnt = value["all_hyoka_cnt"].as_u64().map(|n| n as u32),
                "sa" => novel.sasie_cnt = value["sasie_cnt"].as_u64().map(|n| n as u32),
                "ka" => novel.kaiwaritu = value["kaiwaritu"].as_u64().map(|n| n as u8),
                "nu" => novel.novelupdated_at = value["novelupdated_at"].as_str().map(|s| s.to_string()),
                "ua" => novel.updated_at = value["updated_at"].as_str().map(|s| s.to_string()),
                _ => {}
            }
        }

        if value.get("weekly_unique").is_some() {
            novel.weekly_unique = value["weekly_unique"].as_u64().map(|n| n as u32);
        }

        Ok(novel)
    }
}

impl Default for NarouNovelApiRequest {
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
            opt: None,
            callback: None,
            updatetype: None,
        }
    }
}