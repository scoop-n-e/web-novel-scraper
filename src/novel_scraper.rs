use anyhow::{Context, Result};
use std::collections::HashMap;

/// 小説の種類
#[derive(Debug, Clone, PartialEq)]
pub enum NovelType {
    /// 短編小説（1話完結）
    ShortStory,
    /// 長編小説（複数話）
    Serial { total_episodes: u32 },
}

/// エピソード情報（現時点ではHTMLをそのまま保持）
#[derive(Debug, Clone)]
pub struct Episode {
    /// エピソード番号（短編の場合は0）
    pub episode_number: u32,
    /// HTML全体
    pub html: String,
}

/// 小説本文取得結果
#[derive(Debug, Clone)]
pub struct NovelContent {
    /// 小説のncode
    pub ncode: String,
    /// 小説の種類
    pub novel_type: NovelType,
    /// エピソードリスト
    pub episodes: Vec<Episode>,
}

/// なろう小説本文スクレイパー
pub struct NarouNovelScraper {
    fetcher: crate::HtmlFetcher,
    /// ノクターンかどうか
    is_nocturne: bool,
}

impl NarouNovelScraper {
    /// 新しいスクレイパーを作成（なろう用）
    pub fn new(fetcher: crate::HtmlFetcher) -> Self {
        Self {
            fetcher,
            is_nocturne: false,
        }
    }

    /// 新しいスクレイパーを作成（ノクターン用）
    pub fn new_nocturne(fetcher: crate::HtmlFetcher) -> Self {
        Self {
            fetcher,
            is_nocturne: true,
        }
    }

    /// 小説の全エピソードを取得
    pub async fn fetch_all_episodes(
        &self,
        ncode: &str,
        novel_type: NovelType,
    ) -> Result<NovelContent> {
        let mut episodes = Vec::new();

        match &novel_type {
            NovelType::ShortStory => {
                // 短編の場合は1ページのみ
                let url = self.build_novel_url(ncode, None);
                println!("Fetching short story: {}", url);
                
                let html = self.fetcher.fetch(&url).await
                    .with_context(|| format!("Failed to fetch short story {}", ncode))?;
                
                episodes.push(Episode {
                    episode_number: 0,
                    html,
                });
            }
            NovelType::Serial { total_episodes } => {
                // 長編の場合は各エピソードを順次取得
                for episode_num in 1..=*total_episodes {
                    let url = self.build_novel_url(ncode, Some(episode_num));
                    println!("Fetching episode {}/{}: {}", episode_num, total_episodes, url);
                    
                    let html = self.fetcher.fetch(&url).await
                        .with_context(|| format!("Failed to fetch episode {} of {}", episode_num, ncode))?;
                    
                    episodes.push(Episode {
                        episode_number: episode_num,
                        html,
                    });
                    
                    // 進捗表示
                    if episode_num % 10 == 0 {
                        println!("Progress: {}/{} episodes fetched", episode_num, total_episodes);
                    }
                }
            }
        }

        Ok(NovelContent {
            ncode: ncode.to_string(),
            novel_type,
            episodes,
        })
    }

    /// 小説URLを構築
    fn build_novel_url(&self, ncode: &str, episode_number: Option<u32>) -> String {
        let base_domain = if self.is_nocturne {
            "https://novel18.syosetu.com"
        } else {
            "https://ncode.syosetu.com"
        };

        match episode_number {
            Some(num) => format!("{}/{}/{}/", base_domain, ncode, num),
            None => format!("{}/{}/", base_domain, ncode),
        }
    }

    /// 一括取得（バッチ処理用）
    pub async fn fetch_episodes_batch(
        &self,
        ncode: &str,
        episode_numbers: Vec<u32>,
    ) -> Result<HashMap<u32, Episode>> {
        let mut episodes_map = HashMap::new();

        for episode_num in episode_numbers {
            let url = self.build_novel_url(ncode, Some(episode_num));
            println!("Fetching episode {}: {}", episode_num, url);
            
            let html = self.fetcher.fetch(&url).await
                .with_context(|| format!("Failed to fetch episode {} of {}", episode_num, ncode))?;
            
            episodes_map.insert(episode_num, Episode {
                episode_number: episode_num,
                html,
            });
        }

        Ok(episodes_map)
    }
}

impl NovelContent {
    /// エピソード数を取得
    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }

    /// 特定のエピソードを取得
    pub fn get_episode(&self, episode_number: u32) -> Option<&Episode> {
        match self.novel_type {
            NovelType::ShortStory => {
                if episode_number == 0 {
                    self.episodes.first()
                } else {
                    None
                }
            }
            NovelType::Serial { .. } => {
                self.episodes.iter()
                    .find(|e| e.episode_number == episode_number)
            }
        }
    }

    /// 総文字数を概算（HTML含む）
    pub fn total_size_bytes(&self) -> usize {
        self.episodes.iter()
            .map(|e| e.html.len())
            .sum()
    }
}