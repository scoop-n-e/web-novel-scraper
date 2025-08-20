use anyhow::{Context, Result};
use std::collections::HashMap;
use crate::api::narou::{NarouApiClient, NarouRankingItem};

/// なろう小説ランキングAPI
pub struct NarouRankingApi {
    client: NarouApiClient,
    base_url: String,
}

impl NarouRankingApi {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: NarouApiClient::new()?,
            base_url: "https://api.syosetu.com/rank/rankget/".to_string(),
        })
    }

    /// ランキングを取得
    pub async fn get_ranking(&self, ranking_type: RankingType, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        let mut query = HashMap::new();
        query.insert("out".to_string(), "json".to_string());
        
        // 日付とランキングタイプを結合（例: 20130501-d）
        let rtype = format!("{}-{}", 
            date.unwrap_or_else(|| "20130501".to_string()),
            ranking_type.as_str()
        );
        query.insert("rtype".to_string(), rtype);
        
        self.client.request(&self.base_url, &query).await
    }

    /// 特定日付の日間ランキング
    pub async fn get_daily_ranking(&self, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        self.get_ranking(RankingType::Daily, date).await
    }

    /// 特定日付の週間ランキング
    pub async fn get_weekly_ranking(&self, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        self.get_ranking(RankingType::Weekly, date).await
    }

    /// 特定日付の月間ランキング
    pub async fn get_monthly_ranking(&self, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        self.get_ranking(RankingType::Monthly, date).await
    }

    /// 特定日付の四半期ランキング
    pub async fn get_quarterly_ranking(&self, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        self.get_ranking(RankingType::Quarter, date).await
    }

    /// 特定日付の年間ランキング
    pub async fn get_yearly_ranking(&self, date: Option<String>) -> Result<Vec<NarouRankingItem>> {
        self.get_ranking(RankingType::Yearly, date).await
    }
}

/// ランキングタイプ
#[derive(Debug, Clone, PartialEq)]
pub enum RankingType {
    Daily,  // 日間
    Weekly,  // 週間
    Monthly,  // 月間
    Quarter,  // 四半期
    Yearly,  // 年間
}

impl RankingType {
    pub fn as_str(&self) -> &str {
        match self {
            RankingType::Daily => "d",
            RankingType::Weekly => "w",
            RankingType::Monthly => "m",
            RankingType::Quarter => "q",
            RankingType::Yearly => "y",
        }
    }
}