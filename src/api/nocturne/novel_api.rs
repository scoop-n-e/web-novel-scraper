use anyhow::{Context, Result};
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
        
        // ノクターン専用パラメータ
        if let Some(nocgenre) = params.nocgenre {
            query.insert("nocgenre".to_string(), nocgenre.to_string());
        }
        if let Some(notnocgenre) = params.notnocgenre {
            query.insert("notnocgenre".to_string(), notnocgenre.to_string());
        }
        
        // 詳細フィルタ
        if let Some(length_min) = params.length_min {
            query.insert("minlen".to_string(), length_min.to_string());
        }
        if let Some(length_max) = params.length_max {
            query.insert("maxlen".to_string(), length_max.to_string());
        }
        if let Some(kaiwaritu_min) = params.kaiwaritu_min {
            query.insert("kaiwaritu".to_string(), kaiwaritu_min.to_string());
        }
        if let Some(kaiwaritu_max) = params.kaiwaritu_max {
            query.insert("kaiwaritu2".to_string(), kaiwaritu_max.to_string());
        }
        if let Some(sasie_min) = params.sasie_min {
            query.insert("sasie".to_string(), sasie_min.to_string());
        }
        if let Some(sasie_max) = params.sasie_max {
            query.insert("sasie2".to_string(), sasie_max.to_string());
        }
        
        // 状態フィルタ
        if let Some(is_stop) = params.is_stop {
            query.insert("stop".to_string(), if is_stop { "1" } else { "0" }.to_string());
        }
        if let Some(is_end) = params.is_end {
            query.insert("end".to_string(), if is_end { "1" } else { "0" }.to_string());
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
        
        self.client.request(&self.base_url, &query).await
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

    /// ユーザーIDでR18小説一覧を取得
    pub async fn get_by_userid(&self, userid: u32, limit: Option<u32>) -> Result<Vec<NocturneNovel>> {
        let params = NocturneSearchParams {
            userid: Some(userid),
            limit,
            order: Some(NocturneOrder::New),
            ..Default::default()
        };
        
        self.search(params).await
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
        let mut modified_params = params;
        // ノクターンノベルズのジャンルのみに絞る
        if let Some(genre) = modified_params.nocgenre {
            if genre > 100 {
                return Ok(Vec::new());  // ノクターン以外のジャンルは除外
            }
        }
        self.search(modified_params).await
    }

    /// ムーンライトノベルズの小説を検索
    pub async fn search_moonlight(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        let mut modified_params = params;
        // ムーンライトノベルズのジャンルのみに絞る
        if let Some(genre) = modified_params.nocgenre {
            if genre < 101 || genre > 200 {
                return Ok(Vec::new());  // ムーンライト以外のジャンルは除外
            }
        }
        self.search(modified_params).await
    }

    /// ミッドナイトノベルズの小説を検索
    pub async fn search_midnight(&self, params: NocturneSearchParams) -> Result<Vec<NocturneNovel>> {
        let mut modified_params = params;
        // ミッドナイトノベルズのジャンルのみに絞る
        if let Some(genre) = modified_params.nocgenre {
            if genre < 201 || genre > 300 {
                return Ok(Vec::new());  // ミッドナイト以外のジャンルは除外
            }
        }
        self.search(modified_params).await
    }
}

/// ノクターン小説検索パラメータ
#[derive(Debug, Clone, Default)]
pub struct NocturneSearchParams {
    // 基本検索
    pub word: Option<String>,  // 検索単語（スペース区切りでAND検索）
    pub notword: Option<String>,  // 除外単語（スペース区切り）
    pub title: Option<String>,  // タイトル検索
    pub writer: Option<String>,  // 作者名検索
    pub ncode: Option<String>,  // Nコード指定（-区切りで複数指定可）
    pub userid: Option<u32>,  // ユーザーID指定
    
    // ノクターン専用
    pub nocgenre: Option<u32>,  // ノクターンジャンル指定
    pub notnocgenre: Option<u32>,  // ノクターンジャンル除外
    
    // 文字数フィルタ
    pub length_min: Option<u32>,  // 最小文字数
    pub length_max: Option<u32>,  // 最大文字数
    
    // 会話率フィルタ
    pub kaiwaritu_min: Option<u32>,  // 最小会話率（0-100）
    pub kaiwaritu_max: Option<u32>,  // 最大会話率（0-100）
    
    // 挿絵数フィルタ
    pub sasie_min: Option<u32>,  // 最小挿絵数
    pub sasie_max: Option<u32>,  // 最大挿絵数
    
    // 状態フィルタ
    pub is_stop: Option<bool>,  // 長期連載停止中
    pub is_end: Option<bool>,  // 完結済み
    
    // 出力制御
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<NocturneOrder>,  // ソート順
    pub of: Option<String>,  // 出力項目指定
}