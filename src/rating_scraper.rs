use anyhow::{Context, Result};

/// 評価した小説の情報
#[derive(Debug, Clone)]
pub struct RatingEntry {
    /// 小説のncode（URLから取得）
    pub ncode: String,
    /// 評価ポイント（1.0-5.0、0.5刻み）
    pub rating_point: f32,
    /// 初回評価日
    pub first_rating_date: String,
    /// 最終評価日（存在する場合）
    pub last_rating_date: Option<String>,
}

/// なろう評価ページのスクレイパー
pub struct NarouRatingScraper {
    fetcher: crate::HtmlFetcher,
}

impl NarouRatingScraper {
    /// 新しいスクレイパーを作成
    pub fn new(fetcher: crate::HtmlFetcher) -> Self {
        Self { fetcher }
    }

    /// 指定ユーザーの評価した小説を全件取得
    pub async fn fetch_all_ratings(&self, user_id: u32) -> Result<Vec<RatingEntry>> {
        let mut all_entries = Vec::new();
        let mut page = 1;

        loop {
            // ページのURL生成
            let url = if page == 1 {
                format!("https://mypage.syosetu.com/mypagenovelhyoka/list/userid/{}/", user_id)
            } else {
                format!("https://mypage.syosetu.com/mypagenovelhyoka/list/userid/{}/?p={}", user_id, page)
            };

            println!("Fetching page {}: {}", page, url);

            // HTMLを取得
            let html = self.fetcher.fetch(&url).await
                .with_context(|| format!("Failed to fetch page {}", page))?;

            // ページから評価エントリを抽出
            let entries = Self::parse_rating_page(&html)?;
            
            if entries.is_empty() {
                // エントリが無い場合は終了
                break;
            }

            all_entries.extend(entries);

            // 次のページへのリンクがあるか確認
            if !Self::has_next_page(&html) {
                break;
            }

            page += 1;
            
            // fetcherの遅延機能が自動的にリクエスト間の遅延を処理
        }

        Ok(all_entries)
    }

    /// HTMLから評価エントリを抽出
    fn parse_rating_page(html: &str) -> Result<Vec<RatingEntry>> {
        let mut entries = Vec::new();

        // 各評価アイテムを抽出
        // パターン: <a href="https://ncode.syosetu.com/XXX" class="c-panel__list-item
        let item_pattern = r#"<a href="https://ncode\.syosetu\.com/([^"]+)"[^>]*class="[^"]*c-panel__list-item"#;
        let item_regex = regex::Regex::new(item_pattern)?;

        for cap in item_regex.captures_iter(html) {
            let ncode = cap.get(1)
                .ok_or_else(|| anyhow::anyhow!("Failed to extract ncode"))?
                .as_str()
                .to_string();

            // 該当アイテムのHTMLセクションを取得
            let item_start = cap.get(0).unwrap().start();
            let item_end = html[item_start..]
                .find("</a>")
                .map(|i| item_start + i + 4)
                .unwrap_or(html.len());
            let item_html = &html[item_start..item_end];

            // 初回評価日を抽出
            let date_pattern = r"初回評価日：(\d{4}年\d{1,2}月\d{1,2}日)";
            let date_regex = regex::Regex::new(date_pattern)?;
            let first_rating_date = date_regex.captures(item_html)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .ok_or_else(|| anyhow::anyhow!("Failed to extract first rating date for {}", ncode))?;

            // 最終評価日を抽出（存在する場合）
            let last_date_pattern = r"最終評価日：(\d{4}年\d{1,2}月\d{1,2}日)";
            let last_date_regex = regex::Regex::new(last_date_pattern)?;
            let last_rating_date = last_date_regex.captures(item_html)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string());

            // 評価ポイントを抽出（整数または小数）
            let score_pattern = r#"data-score="([0-9]+(?:\.[0-9]+)?)"#;
            let score_regex = regex::Regex::new(score_pattern)?;
            let rating_point = score_regex.captures(item_html)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .ok_or_else(|| anyhow::anyhow!("Failed to extract rating point for {}", ncode))?;

            entries.push(RatingEntry {
                ncode,
                rating_point,
                first_rating_date,
                last_rating_date,
            });
        }

        Ok(entries)
    }

    /// 次のページへのリンクがあるか確認
    fn has_next_page(html: &str) -> bool {
        // 「次へ」リンクが無効化されていないか確認
        // 最終ページでは: <span class="c-pager__item is-disabled" title="次へ">
        // それ以外では: <a href="?p=X" class="c-pager__item" title="次へ">
        
        if html.contains(r#"<span class="c-pager__item is-disabled" title="次へ">"#) {
            return false;
        }

        html.contains(r#"title="次へ">"#) && html.contains(r#"<a href="?p="#)
    }
}

// 正規表現を使用するための依存関係
use once_cell::sync::Lazy;
static _INIT: Lazy<()> = Lazy::new(|| {
    // 正規表現の初期化（実際はregex crateで自動的に行われる）
});