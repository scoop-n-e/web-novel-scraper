use anyhow::Result;
use std::collections::HashMap;
use crate::api::nocturne::{NocturneApiClient, NocturneNovel, NocturneOrder, NocturneGenre};

/// ノクターン小説API（R18専用）
pub struct NocturneNovelApi {
    client: NocturneApiClient,
    base_url: String,
}

impl NocturneNovelApi {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: NocturneApiClient::new()?,
            base_url: "https://api.syosetu.com/novel18api/api/".to_string(),
        })
    }

    /// R18小説情報を検索
    pub async fn search(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        
        // 基本検索パラメータ
        if let Some(word) = params.word {
            query.insert("word".to_string(), word);
        }
        if let Some(notword) = params.notword {
            query.insert("notword".to_string(), notword);
        }
        if let Some(ncode) = params.ncode {
            query.insert("ncode".to_string(), ncode);
        }
        if let Some(xid) = params.xid {
            query.insert("xid".to_string(), xid);
        }
        
        // 検索範囲指定
        if let Some(title) = params.title {
            query.insert("title".to_string(), if title { "1" } else { "0" }.to_string());
        }
        if let Some(ex) = params.ex {
            query.insert("ex".to_string(), if ex { "1" } else { "0" }.to_string());
        }
        if let Some(keyword) = params.keyword {
            query.insert("keyword".to_string(), if keyword { "1" } else { "0" }.to_string());
        }
        if let Some(wname) = params.wname {
            query.insert("wname".to_string(), if wname { "1" } else { "0" }.to_string());
        }
        
        // ノクターン専用パラメータ
        if let Some(nocgenre) = params.nocgenre {
            query.insert("nocgenre".to_string(), nocgenre.to_string());
        }
        if let Some(notnocgenre) = params.notnocgenre {
            query.insert("notnocgenre".to_string(), notnocgenre.to_string());
        }
        
        // 要素フィルタ
        if let Some(isbl) = params.isbl {
            query.insert("isbl".to_string(), if isbl { "1" } else { "0" }.to_string());
        }
        if let Some(isgl) = params.isgl {
            query.insert("isgl".to_string(), if isgl { "1" } else { "0" }.to_string());
        }
        if let Some(iszankoku) = params.iszankoku {
            query.insert("iszankoku".to_string(), if iszankoku { "1" } else { "0" }.to_string());
        }
        if let Some(istensei) = params.istensei {
            query.insert("istensei".to_string(), if istensei { "1" } else { "0" }.to_string());
        }
        if let Some(istenni) = params.istenni {
            query.insert("istenni".to_string(), if istenni { "1" } else { "0" }.to_string());
        }
        if let Some(istt) = params.istt {
            query.insert("istt".to_string(), if istt { "1" } else { "0" }.to_string());
        }
        
        // 要素除外フィルタ
        if let Some(notbl) = params.notbl {
            query.insert("notbl".to_string(), if notbl { "1" } else { "0" }.to_string());
        }
        if let Some(notgl) = params.notgl {
            query.insert("notgl".to_string(), if notgl { "1" } else { "0" }.to_string());
        }
        if let Some(notzankoku) = params.notzankoku {
            query.insert("notzankoku".to_string(), if notzankoku { "1" } else { "0" }.to_string());
        }
        if let Some(nottensei) = params.nottensei {
            query.insert("nottensei".to_string(), if nottensei { "1" } else { "0" }.to_string());
        }
        if let Some(nottenni) = params.nottenni {
            query.insert("nottenni".to_string(), if nottenni { "1" } else { "0" }.to_string());
        }
        
        // 詳細フィルタ
        // 文字数フィルタ
        if let Some(length) = params.length {
            query.insert("length".to_string(), length);
        } else {
            if let Some(length_min) = params.length_min {
                query.insert("minlen".to_string(), length_min.to_string());
            }
            if let Some(length_max) = params.length_max {
                query.insert("maxlen".to_string(), length_max.to_string());
            }
        }
        
        // 読了時間フィルタ
        if let Some(time) = params.time {
            query.insert("time".to_string(), time);
        } else {
            if let Some(mintime) = params.mintime {
                query.insert("mintime".to_string(), mintime.to_string());
            }
            if let Some(maxtime) = params.maxtime {
                query.insert("maxtime".to_string(), maxtime.to_string());
            }
        }
        // 会話率フィルタ
        if let Some(kaiwaritu) = params.kaiwaritu {
            query.insert("kaiwaritu".to_string(), kaiwaritu);
        }
        
        // 挿絵数フィルタ
        if let Some(sasie) = params.sasie {
            query.insert("sasie".to_string(), sasie);
        }
        
        // 状態フィルタ
        if let Some(stop) = params.stop {
            query.insert("stop".to_string(), stop.to_string());
        }
        
        // タイプフィルタ
        if let Some(novel_type) = params.novel_type {
            query.insert("type".to_string(), novel_type);
        }
        
        // 文体フィルタ
        if let Some(buntai) = params.buntai {
            query.insert("buntai".to_string(), buntai);
        }
        
        // 日付フィルタ
        if let Some(lastup) = params.lastup {
            query.insert("lastup".to_string(), lastup);
        }
        if let Some(lastupdate) = params.lastupdate {
            query.insert("lastupdate".to_string(), lastupdate);
        }
        
        // ピックアップ
        if let Some(ispickup) = params.ispickup {
            query.insert("ispickup".to_string(), if ispickup { "1" } else { "0" }.to_string());
        }
        
        // 出力制御
        if let Some(limit) = params.limit {
            query.insert("lim".to_string(), limit.to_string());
        }
        if let Some(start) = params.start {
            query.insert("st".to_string(), start.to_string());
        }
        if let Some(order) = params.order {
            query.insert("order".to_string(), order.as_str().to_string());
        }
        if let Some(of) = params.of {
            query.insert("of".to_string(), of);
        }
        
        // gzip圧縮対応
        if let Some(gzip_level) = params.gzip {
            self.client.request_with_gzip(&self.base_url, &mut query, Some(gzip_level)).await
        } else {
            self.client.request(&self.base_url, &query).await
        }
    }

    /// ncodeでR18小説情報を取得
    pub async fn get_by_ncode(&self, ncode: &str) -> Result<Option<NocturneNovel>> {
        let params = NocturneSearchParams {
            ncode: Some(ncode.to_string()),
            limit: Some(1),
            ..Default::default()
        };
        
        let results = self.search(params).await?;
        Ok(results.into_iter().next())
    }


    /// ジャンル別にR18小説を検索
    pub async fn search_by_genre(&self, genre: NocturneGenre, limit: Option<u32>) -> Result<Vec<NocturneNovel>> {
        let params = NocturneSearchParams {
            nocgenre: Some(genre.value()),
            limit,
            order: Some(NocturneOrder::FavNovelCnt),
            ..Default::default()
        };
        
        self.search(params).await
    }

    /// ノクターンノベルズの小説を検索
    pub async fn search_nocturne(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        // ノクターンノベルズのジャンルのみに絞る
        if let Some(genre) = params.nocgenre {
            if genre > 100 {
                return Ok(Vec::new());  // ノクターン以外のジャンルは除外
            }
        }
        self.search(params).await
    }

    /// ムーンライトノベルズの小説を検索
    pub async fn search_moonlight(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        // ムーンライトノベルズのジャンルのみに絞る
        if let Some(genre) = params.nocgenre {
            if genre < 101 || genre > 200 {
                return Ok(Vec::new());  // ムーンライト以外のジャンルは除外
            }
        }
        self.search(params).await
    }

    /// ミッドナイトノベルズの小説を検索
    pub async fn search_midnight(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        // ミッドナイトノベルズのジャンルのみに絞る
        if let Some(genre) = params.nocgenre {
            if genre < 201 || genre > 300 {
                return Ok(Vec::new());  // ミッドナイト以外のジャンルは除外
            }
        }
        self.search(params).await
    }
}

/// ノクターン小説検索パラメータ
#[derive(Debug, Clone, Default)]
pub struct NocturneSearchParams {
    // 基本検索
    pub word: Option<String>,  // 検索単語（スペース区切りでAND検索）
    pub notword: Option<String>,  // 除外単語（スペース区切り）
    pub ncode: Option<String>,  // Nコード指定（-区切りで複数指定可）
    pub xid: Option<String>,  // XID指定（-区切りで複数指定可）
    
    // 検索範囲指定（1で対象にする）
    pub title: Option<bool>,  // タイトルを検索対象に
    pub ex: Option<bool>,  // あらすじを検索対象に
    pub keyword: Option<bool>,  // キーワードを検索対象に
    pub wname: Option<bool>,  // 作者名を検索対象に
    
    // ノクターン専用
    pub nocgenre: Option<u32>,  // ノクターンジャンル指定（1-4）
    pub notnocgenre: Option<u32>,  // ノクターンジャンル除外
    
    // 要素フィルタ
    pub isbl: Option<bool>,  // ボーイズラブ
    pub isgl: Option<bool>,  // ガールズラブ
    pub iszankoku: Option<bool>,  // 残酷な描写あり
    pub istensei: Option<bool>,  // 異世界転生
    pub istenni: Option<bool>,  // 異世界転移
    pub istt: Option<bool>,  // 転生または転移
    
    // 要素除外フィルタ
    pub notbl: Option<bool>,  // ボーイズラブ除外
    pub notgl: Option<bool>,  // ガールズラブ除外
    pub notzankoku: Option<bool>,  // 残酷な描写除外
    pub nottensei: Option<bool>,  // 異世界転生除外
    pub nottenni: Option<bool>,  // 異世界転移除外
    
    // 文字数フィルタ（length指定時はmin/maxは無視）
    pub length: Option<String>,  // 文字数範囲（例: "1000-5000", "1000-", "-5000"）
    pub length_min: Option<u32>,  // 最小文字数
    pub length_max: Option<u32>,  // 最大文字数
    
    // 読了時間フィルタ（文字数フィルタと併用不可）
    pub time: Option<String>,  // 読了時間範囲（分単位）
    pub mintime: Option<u32>,  // 最小読了時間（分）
    pub maxtime: Option<u32>,  // 最大読了時間（分）
    
    // 会話率フィルタ
    pub kaiwaritu: Option<String>,  // 会話率範囲（例: "10-50", "50-", "30"）
    
    // 挿絵数フィルタ
    pub sasie: Option<String>,  // 挿絵数範囲（例: "1-5", "1-", "3"）
    
    // 状態フィルタ
    pub stop: Option<u32>,  // 1:長期連載停止除外, 2:長期連載停止のみ
    
    // タイプフィルタ
    pub novel_type: Option<String>,  // "t": 短編, "r": 連載中, "er": 完結済連載, "re": 全連載, "ter": 短編+完結済
    
    // 文体フィルタ
    pub buntai: Option<String>,  // 文体指定（1,2,4,6）
    
    // 日付フィルタ
    pub lastup: Option<String>,  // 最終掲載日（thisweek, lastweek, sevenday, thismonth, lastmonth, タイムスタンプ）
    pub lastupdate: Option<String>,  // 最終更新日（thisweek, lastweek, sevenday, thismonth, lastmonth, タイムスタンプ）
    
    // ピックアップ
    pub ispickup: Option<bool>,  // ピックアップ作品のみ
    
    // 出力制御
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<NocturneOrder>,  // ソート順
    pub of: Option<String>,  // 出力項目指定
    
    // gzip圧縮
    pub gzip: Option<u8>,  // gzip圧縮レベル（1-5）
}