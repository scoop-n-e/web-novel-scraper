use anyhow::Result;
use std::collections::HashMap;
use crate::api::narou::{NarouApiClient, NarouUser};

/// なろうユーザー検索API
pub struct NarouUserApi {
    client: NarouApiClient,
    base_url: String,
}

impl NarouUserApi {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: NarouApiClient::new()?,
            base_url: "https://api.syosetu.com/userapi/api/".to_string(),
        })
    }

    /// ユーザー情報を検索
    pub async fn search(&self, params: UserSearchParams) -> Result<Vec<NarouUser>> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        
        // 検索条件
        if let Some(word) = params.word {
            query.insert("word".to_string(), word);
        }
        if let Some(notword) = params.notword {
            query.insert("notword".to_string(), notword);
        }
        if let Some(userid) = params.userid {
            query.insert("userid".to_string(), userid.to_string());
        }
        if let Some(name) = params.name {
            query.insert("name".to_string(), name);
        }
        if let Some(name1st) = params.name1st {
            query.insert("name1st".to_string(), name1st);
        }
        
        // フィルタ条件
        if let Some(novel_min) = params.novel_min {
            query.insert("minnovel".to_string(), novel_min.to_string());
        }
        if let Some(novel_max) = params.novel_max {
            query.insert("maxnovel".to_string(), novel_max.to_string());
        }
        if let Some(review_min) = params.review_min {
            query.insert("minreview".to_string(), review_min.to_string());
        }
        if let Some(review_max) = params.review_max {
            query.insert("maxreview".to_string(), review_max.to_string());
        }
        
        // 出力制御
        if let Some(limit) = params.limit {
            query.insert("lim".to_string(), limit.to_string());
        }
        if let Some(start) = params.start {
            query.insert("st".to_string(), start.to_string());
        }
        if let Some(order) = params.order {
            query.insert("order".to_string(), order);
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

    /// ユーザーIDでユーザー情報を取得
    pub async fn get_by_userid(&self, userid: u32) -> Result<Option<NarouUser>> {
        let params = UserSearchParams {
            userid: Some(userid),
            limit: Some(1),
            ..Default::default()
        };
        
        let results = self.search(params).await?;
        Ok(results.into_iter().next())
    }

    /// ユーザー名でユーザー情報を検索
    pub async fn search_by_name(&self, name: &str, limit: Option<u32>) -> Result<Vec<NarouUser>> {
        let params = UserSearchParams {
            name: Some(name.to_string()),
            limit,
            ..Default::default()
        };
        
        self.search(params).await
    }
}

/// ユーザー検索パラメータ
#[derive(Debug, Clone, Default)]
pub struct UserSearchParams {
    // 検索条件
    pub word: Option<String>,  // 検索単語
    pub notword: Option<String>,  // 除外単語
    pub userid: Option<u32>,  // ユーザーID指定（-区切りで複数指定可）
    pub name: Option<String>,  // ユーザー名指定
    pub name1st: Option<String>,  // 頭文字指定（ひらがな・カタカナ・英数字・その他）
    
    // フィルタ条件
    pub novel_min: Option<u32>,  // 作品投稿数最小
    pub novel_max: Option<u32>,  // 作品投稿数最大
    pub review_min: Option<u32>,  // レビュー投稿数最小
    pub review_max: Option<u32>,  // レビュー投稿数最大
    
    // 出力制御
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<String>,  // ソート順（userid, name, novel, review, sumglobalpoint）
    pub of: Option<String>,  // 出力項目指定（u-n-y-nc-rc-nl-sg）
    
    // gzip圧縮
    pub gzip: Option<u8>,  // gzip圧縮レベル（1-5）
}