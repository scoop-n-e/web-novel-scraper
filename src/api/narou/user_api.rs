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
        if let Some(minnovel) = params.minnovel {
            query.insert("minnovel".to_string(), minnovel.to_string());
        }
        if let Some(maxnovel) = params.maxnovel {
            query.insert("maxnovel".to_string(), maxnovel.to_string());
        }
        if let Some(minreview) = params.minreview {
            query.insert("minreview".to_string(), minreview.to_string());
        }
        if let Some(maxreview) = params.maxreview {
            query.insert("maxreview".to_string(), maxreview.to_string());
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
        
        // 出力形式制御（注：現在はJSON固定のため、これらのパラメータは効果なし）
        if let Some(libtype) = params.libtype {
            query.insert("libtype".to_string(), libtype.to_string());
        }
        if let Some(callback) = params.callback {
            query.insert("callback".to_string(), callback);
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

/// ユーザー検索パラメータ（仕様書準拠）
#[derive(Debug, Clone, Default)]
pub struct UserSearchParams {
    // 出力制御GETパラメータ（仕様書順）
    pub gzip: Option<u8>,  // gzip圧縮レベル（1-5）
    // out は内部で自動設定 (json)
    pub of: Option<String>,  // 出力項目指定（u-n-y-nc-rc-nl-sg）
    pub limit: Option<u32>,  // 最大出力数（1-500、デフォルト20）
    pub start: Option<u32>,  // 表示開始位置（1-2000）
    pub order: Option<String>,  // ソート順（new, userid, name, novelcnt, reviewcnt, novellength, sumglobalpoint, old）
    pub libtype: Option<u8>,  // YAMLライブラリ選択（1:従来、2:新ライブラリ）
    pub callback: Option<String>,  // JSONP用コールバック関数名
    
    // 条件抽出パラメータ（仕様書順）
    pub word: Option<String>,  // 検索単語
    pub notword: Option<String>,  // 除外単語
    pub userid: Option<u32>,  // ユーザーID指定（-区切りで複数指定可）
    pub name: Option<String>,  // ユーザー名指定
    pub name1st: Option<String>,  // 頭文字指定（ひらがな・カタカナ・英数字・その他）
    pub minnovel: Option<u32>,  // 作品投稿数最小
    pub maxnovel: Option<u32>,  // 作品投稿数最大
    pub minreview: Option<u32>,  // レビュー投稿数最小
    pub maxreview: Option<u32>,  // レビュー投稿数最大
}