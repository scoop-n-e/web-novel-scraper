use anyhow::{Context, Result};
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
    
    // 出力制御
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<NarouOrder>,  // ソート順
    pub of: Option<String>,  // 出力項目指定（t-n-u-w-s-bg-g-k-gf-gl-nt-e-ga-l-ti-isr15-isbl-isgl-izk-its-iti）
}