use serde::{Deserialize, Serialize};

/// なろう小説情報（仕様書順）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouNovel {
    // allcountは別途管理
    pub title: Option<String>,
    pub ncode: Option<String>,
    pub userid: Option<u32>,
    pub writer: Option<String>,
    pub story: Option<String>,
    pub biggenre: Option<u32>,
    pub genre: Option<u32>,
    pub gensaku: Option<String>,  // 原作（未使用）
    pub keyword: Option<String>,
    pub general_firstup: Option<String>,  // 初回掲載日
    pub general_lastup: Option<String>,  // 最終掲載日
    pub novel_type: Option<u32>,  // 1:連載, 2:短編
    pub end: Option<u32>,  // 0:連載中, 1:完結済
    pub general_all_no: Option<u32>,  // 全話数
    pub length: Option<u32>,  // 文字数
    pub time: Option<u32>,  // 読了時間（分）
    pub isstop: Option<u32>,  // 1:長期連載停止中
    pub isr15: Option<u32>,  // 1:R15
    pub isbl: Option<u32>,  // 1:ボーイズラブ
    pub isgl: Option<u32>,  // 1:ガールズラブ
    pub iszankoku: Option<u32>,  // 1:残酷な描写あり
    pub istensei: Option<u32>,  // 1:転生
    pub istenni: Option<u32>,  // 1:転移
    // pc_or_kは仕様書にないが保持
    pub global_point: Option<u32>,  // 総合評価ポイント
    pub daily_point: Option<u32>,  // 日間ポイント
    pub weekly_point: Option<u32>,  // 週間ポイント
    pub monthly_point: Option<u32>,  // 月間ポイント
    pub quarter_point: Option<u32>,  // 四半期ポイント
    pub yearly_point: Option<u32>,  // 年間ポイント
    pub fav_novel_cnt: Option<u32>,  // ブックマーク数
    pub impression_cnt: Option<u32>,  // 感想数
    pub review_cnt: Option<u32>,  // レビュー数
    pub all_point: Option<u32>,  // 評価ポイント
    pub all_hyoka_cnt: Option<u32>,  // 評価者数
    pub sasie_cnt: Option<u32>,  // 挿絵数
    pub kaiwaritu: Option<u32>,  // 会話率
    pub novelupdated_at: Option<String>,  // 最終更新日時
    pub updated_at: Option<String>,  // システム更新日時
    pub pc_or_k: Option<u32>,  // 1:PC投稿, 2:ケータイ投稿
}

/// なろうユーザー情報（仕様書順）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouUser {
    // allcountは別途管理
    pub userid: Option<u32>,
    pub name: Option<String>,
    pub yomikata: Option<String>,  // 読み方
    pub name1st: Option<String>,  // 頭文字
    pub novel_cnt: Option<u32>,  // 投稿作品数
    pub review_cnt: Option<u32>,  // レビュー投稿数
    pub novel_length: Option<u32>,  // 累計文字数
    pub sum_global_point: Option<u32>,  // 総合評価ポイント合計
    // 互換性のために旧名称も保持
    pub sumglobalpoint: Option<u32>,  // 総合評価ポイント合計（旧名）
}

/// なろうランキング情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRankingItem {
    pub rank: u32,
    pub pt: u32,  // ポイント
    pub ncode: String,
}

/// なろう殿堂入り情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRankinResponse {
    pub rank: Vec<NarouRankinRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarouRankinRecord {
    pub pt: u32,    // ポイント
    pub rank: u32,   // 順位
    pub rtype: String,  // ランキングタイプ（例: "20130501-d"）
}

/// なろうAPIレスポンス（配列形式）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NarouApiResponse<T> {
    Success(Vec<T>),
    Error { error: String },
}

/// なろうAPIソート順
#[derive(Debug, Clone, PartialEq)]
pub enum NarouOrder {
    New,  // 新着更新順
    FavNovelCnt,  // ブックマーク数順
    ReviewCnt,  // レビュー数順
    Hyoka,  // 総合ポイント高い順
    HyokaAsc,  // 総合ポイント低い順
    DailyPoint,  // 日間ポイント順
    WeeklyPoint,  // 週間ポイント順
    MonthlyPoint,  // 月間ポイント順
    QuarterPoint,  // 四半期ポイント順
    YearlyPoint,  // 年間ポイント順
    ImpressionCnt,  // 感想数順
    HyokaCnt,  // 評価者数順
    HyokaCntAsc,  // 評価者数少ない順
    Weekly,  // 週間ユニークユーザ順
    LengthDesc,  // 文字数多い順
    LengthAsc,  // 文字数少ない順
    NcodeDesc,  // Nコード降順
    NcodeAsc,  // Nコード昇順
    Old,  // 古い順
}

impl NarouOrder {
    pub fn as_str(&self) -> &str {
        match self {
            NarouOrder::New => "new",
            NarouOrder::FavNovelCnt => "favnovelcnt",
            NarouOrder::ReviewCnt => "reviewcnt",
            NarouOrder::Hyoka => "hyoka",
            NarouOrder::HyokaAsc => "hyokaasc",
            NarouOrder::DailyPoint => "dailypoint",
            NarouOrder::WeeklyPoint => "weeklypoint",
            NarouOrder::MonthlyPoint => "monthlypoint",
            NarouOrder::QuarterPoint => "quarterpoint",
            NarouOrder::YearlyPoint => "yearlypoint",
            NarouOrder::ImpressionCnt => "impressioncnt",
            NarouOrder::HyokaCnt => "hyokacnt",
            NarouOrder::HyokaCntAsc => "hyokacntasc",
            NarouOrder::Weekly => "weekly",
            NarouOrder::LengthDesc => "lengthdesc",
            NarouOrder::LengthAsc => "lengthasc",
            NarouOrder::NcodeDesc => "ncodedesc",
            NarouOrder::NcodeAsc => "ncodeasc",
            NarouOrder::Old => "old",
        }
    }
}

/// なろう出力形式
#[derive(Debug, Clone, PartialEq)]
pub enum NarouOutputFormat {
    Json,
    Yaml,
    Php,
}

impl NarouOutputFormat {
    pub fn as_str(&self) -> &str {
        match self {
            NarouOutputFormat::Json => "json",
            NarouOutputFormat::Yaml => "yaml",
            NarouOutputFormat::Php => "php",
        }
    }
}