use anyhow::Result;
use std::collections::HashMap;
use crate::api::narou::{NarouApiClient, NarouNovel, NarouOrder};

/// なろう小説API
pub struct NarouNovelApi {
    client: NarouApiClient,
    base_url: String,
}

impl NarouNovelApi {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: NarouApiClient::new()?,
            base_url: "https://api.syosetu.com/novelapi/api/".to_string(),
        })
    }

    /// 小説情報を検索
    pub async fn search(&self, params: NovelSearchParams) -> Result<Vec<NarouNovel>> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        
        // 検索条件
        if let Some(word) = params.word {
            query.insert("word".to_string(), word);
        }
        if let Some(notword) = params.notword {
            query.insert("notword".to_string(), notword);
        }
        if let Some(title) = params.title {
            query.insert("title".to_string(), title);
        }
        if let Some(writer) = params.writer {
            query.insert("writer".to_string(), writer);
        }
        if let Some(ncode) = params.ncode {
            query.insert("ncode".to_string(), ncode);
        }
        if let Some(userid) = params.userid {
            query.insert("userid".to_string(), userid.to_string());
        }
        if let Some(genre) = params.genre {
            query.insert("genre".to_string(), genre.to_string());
        }
        if let Some(biggenre) = params.biggenre {
            query.insert("biggenre".to_string(), biggenre.to_string());
        }
        if let Some(notbiggenre) = params.notbiggenre {
            query.insert("notbiggenre".to_string(), notbiggenre.to_string());
        }
        
        // 詳細フィルタ
        if let Some(istensei) = params.istensei {
            query.insert("istensei".to_string(), if istensei { "1" } else { "0" }.to_string());
        }
        if let Some(istenni) = params.istenni {
            query.insert("istenni".to_string(), if istenni { "1" } else { "0" }.to_string());
        }
        if let Some(isstop) = params.isstop {
            query.insert("stop".to_string(), if isstop { "1" } else { "0" }.to_string());
        }
        if let Some(isbl) = params.isbl {
            query.insert("isbl".to_string(), if isbl { "1" } else { "0" }.to_string());
        }
        if let Some(isgl) = params.isgl {
            query.insert("isgl".to_string(), if isgl { "1" } else { "0" }.to_string());
        }
        if let Some(iszankoku) = params.iszankoku {
            query.insert("iszankoku".to_string(), if iszankoku { "1" } else { "0" }.to_string());
        }
        if let Some(isr15) = params.isr15 {
            query.insert("isr15".to_string(), if isr15 { "1" } else { "0" }.to_string());
        }
        
        // 文字数フィルタ
        if let Some(length) = params.length {
            query.insert("length".to_string(), length);
        } else {
            if let Some(minlen) = params.minlen {
                query.insert("minlen".to_string(), minlen.to_string());
            }
            if let Some(maxlen) = params.maxlen {
                query.insert("maxlen".to_string(), maxlen.to_string());
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
        
        // 読了時間フィルタ
        if let Some(time) = params.time {
            query.insert("time".to_string(), time);
        }
        
        // 最終更新日時フィルタ
        if let Some(lastup) = params.lastup {
            query.insert("lastup".to_string(), lastup);
        }
        
        // タイプフィルタ
        if let Some(novel_type) = params.novel_type {
            query.insert("type".to_string(), novel_type);
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

    /// ncodeで小説情報を取得
    pub async fn get_by_ncode(&self, ncode: &str) -> Result<Option<NarouNovel>> {
        let params = NovelSearchParams {
            ncode: Some(ncode.to_string()),
            limit: Some(1),
            ..Default::default()
        };
        
        let results = self.search(params).await?;
        Ok(results.into_iter().next())
    }

    /// ユーザーIDで小説一覧を取得
    pub async fn get_by_userid(&self, userid: u32, limit: Option<u32>) -> Result<Vec<NarouNovel>> {
        let params = NovelSearchParams {
            userid: Some(userid),
            limit,
            order: Some(NarouOrder::New),
            ..Default::default()
        };
        
        self.search(params).await
    }

    /// ジャンルで小説を検索
    pub async fn search_by_genre(&self, genre: u32, limit: Option<u32>) -> Result<Vec<NarouNovel>> {
        let params = NovelSearchParams {
            genre: Some(genre),
            limit,
            order: Some(NarouOrder::FavNovelCnt),
            ..Default::default()
        };
        
        self.search(params).await
    }
}

/// 小説検索パラメータ
#[derive(Debug, Clone, Default)]
pub struct NovelSearchParams {
    // 検索条件
    pub word: Option<String>,  // 検索単語（スペース区切りでAND検索）
    pub notword: Option<String>,  // 除外単語（スペース区切り）
    pub title: Option<String>,  // タイトル検索
    pub writer: Option<String>,  // 作者名検索
    pub ncode: Option<String>,  // Nコード指定（-区切りで複数指定可）
    pub userid: Option<u32>,  // ユーザーID指定
    pub genre: Option<u32>,  // ジャンル指定
    pub biggenre: Option<u32>,  // 大ジャンル指定
    pub notbiggenre: Option<u32>,  // 大ジャンル除外
    
    // 詳細フィルタ
    pub istensei: Option<bool>,  // 転生要素
    pub istenni: Option<bool>,  // 転移要素
    pub isstop: Option<bool>,  // 長期連載停止中
    pub isbl: Option<bool>,  // ボーイズラブ
    pub isgl: Option<bool>,  // ガールズラブ
    pub iszankoku: Option<bool>,  // 残酷な描写あり
    pub isr15: Option<bool>,  // R15作品
    
    // 文字数フィルタ（length指定時はminlen/maxlenは無視）
    pub length: Option<String>,  // 文字数範囲（例: "1000-5000", "1000-", "-5000"）
    pub minlen: Option<u32>,  // 最小文字数
    pub maxlen: Option<u32>,  // 最大文字数
    
    // 会話率フィルタ
    pub kaiwaritu: Option<String>,  // 会話率範囲（例: "10-50", "50-", "30"）
    
    // 挿絵数フィルタ
    pub sasie: Option<String>,  // 挿絵数範囲（例: "1-5", "1-", "3"）
    
    // 読了時間フィルタ（分単位、文字数フィルタと併用不可）
    pub time: Option<String>,  // 読了時間範囲（例: "5-10", "60-", "30"）
    
    // 最終更新日時フィルタ
    pub lastup: Option<String>,  // 最終更新日時（YYYYMMDDhhmmss形式）
    
    // タイプフィルタ
    pub novel_type: Option<String>,  // "t": 短編, "r": 連載中, "er": 完結済連載
    
    // 出力制御
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<NarouOrder>,  // ソート順
    pub of: Option<String>,  // 出力項目指定
    
    // gzip圧縮
    pub gzip: Option<u8>,  // gzip圧縮レベル（1-5）
}