use serde::{Deserialize, Serialize};

/// ノクターン（R18）小説情報（仕様書順）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NocturneNovel {
    // allcountは別途管理
    pub title: Option<String>,
    pub ncode: Option<String>,
    pub writer: Option<String>,
    pub story: Option<String>,
    pub nocgenre: Option<u32>,  // ノクターンジャンル
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
    pub isbl: Option<u32>,  // 1:ボーイズラブ
    pub isgl: Option<u32>,  // 1:ガールズラブ
    pub iszankoku: Option<u32>,  // 1:残酷な描写あり
    pub istensei: Option<u32>,  // 1:転生
    pub istenni: Option<u32>,  // 1:転移
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
    
    // ノクターン専用フィールド（仕様書にないが実装上必要）
    pub xid: Option<String>,  // ノクターンID（XID）
    pub novel_no: Option<u32>,  // 小説番号
    pub nocturne: Option<u32>,  // 1:ノクターン, 2:ムーンライト, 3:ミッドナイト
    pub pc_or_k: Option<u32>,  // 1:PC投稿, 2:ケータイ投稿
}

/// ノクターンAPIレスポンス
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NocturneApiResponse<T> {
    Success(Vec<T>),
    Error { error: String },
}

/// ノクターンAPIソート順
#[derive(Debug, Clone, PartialEq)]
pub enum NocturneOrder {
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

impl NocturneOrder {
    pub fn as_str(&self) -> &str {
        match self {
            NocturneOrder::New => "new",
            NocturneOrder::FavNovelCnt => "favnovelcnt",
            NocturneOrder::ReviewCnt => "reviewcnt",
            NocturneOrder::Hyoka => "hyoka",
            NocturneOrder::HyokaAsc => "hyokaasc",
            NocturneOrder::DailyPoint => "dailypoint",
            NocturneOrder::WeeklyPoint => "weeklypoint",
            NocturneOrder::MonthlyPoint => "monthlypoint",
            NocturneOrder::QuarterPoint => "quarterpoint",
            NocturneOrder::YearlyPoint => "yearlypoint",
            NocturneOrder::ImpressionCnt => "impressioncnt",
            NocturneOrder::HyokaCnt => "hyokacnt",
            NocturneOrder::HyokaCntAsc => "hyokacntasc",
            NocturneOrder::Weekly => "weekly",
            NocturneOrder::LengthDesc => "lengthdesc",
            NocturneOrder::LengthAsc => "lengthasc",
            NocturneOrder::NcodeDesc => "ncodedesc",
            NocturneOrder::NcodeAsc => "ncodeasc",
            NocturneOrder::Old => "old",
        }
    }
}

/// ノクターンジャンル定義
#[derive(Debug, Clone, PartialEq)]
pub enum NocturneGenre {
    // ノクターンノベルズ（男性向け）
    NocAnotherWorld = 1,  // 異世界
    NocRealWorld = 2,  // 現実世界
    NocSF = 3,  // SF
    NocFantasy = 4,  // ファンタジー
    NocRomance = 5,  // 恋愛
    NocHorror = 6,  // ホラー
    NocMystery = 7,  // ミステリー
    NocScience = 8,  // 科学
    NocMecha = 9,  // メカ
    NocHistory = 10,  // 歴史
    
    // ムーンライトノベルズ（女性向け）
    MoonHetero = 101,  // ヘテロ
    MoonBL = 102,  // BL
    MoonPoetry = 103,  // 詩
    MoonEssay = 104,  // エッセイ
    MoonReplay = 105,  // リプレイ
    MoonOther = 106,  // その他
    
    // ミッドナイトノベルズ（BL）
    MidModern = 201,  // 現代（日本）
    MidOverseas = 202,  // 現代（海外）
    MidHistory = 203,  // 歴史（日本）
    MidHistoryOverseas = 204,  // 歴史（海外）
    MidFantasy = 205,  // ファンタジー
    MidSF = 206,  // SF
    MidOther = 207,  // その他
}

impl NocturneGenre {
    pub fn value(&self) -> u32 {
        match self {
            // ノクターンノベルズ
            NocturneGenre::NocAnotherWorld => 1,
            NocturneGenre::NocRealWorld => 2,
            NocturneGenre::NocSF => 3,
            NocturneGenre::NocFantasy => 4,
            NocturneGenre::NocRomance => 5,
            NocturneGenre::NocHorror => 6,
            NocturneGenre::NocMystery => 7,
            NocturneGenre::NocScience => 8,
            NocturneGenre::NocMecha => 9,
            NocturneGenre::NocHistory => 10,
            // ムーンライトノベルズ
            NocturneGenre::MoonHetero => 101,
            NocturneGenre::MoonBL => 102,
            NocturneGenre::MoonPoetry => 103,
            NocturneGenre::MoonEssay => 104,
            NocturneGenre::MoonReplay => 105,
            NocturneGenre::MoonOther => 106,
            // ミッドナイトノベルズ
            NocturneGenre::MidModern => 201,
            NocturneGenre::MidOverseas => 202,
            NocturneGenre::MidHistory => 203,
            NocturneGenre::MidHistoryOverseas => 204,
            NocturneGenre::MidFantasy => 205,
            NocturneGenre::MidSF => 206,
            NocturneGenre::MidOther => 207,
        }
    }
    
    pub fn site_name(&self) -> &str {
        match self.value() {
            1..=100 => "ノクターンノベルズ",
            101..=200 => "ムーンライトノベルズ",
            201..=300 => "ミッドナイトノベルズ",
            _ => "不明",
        }
    }
    
    pub fn site_code(&self) -> &str {
        match self.value() {
            1..=100 => "noc",
            101..=200 => "mnlt",
            201..=300 => "mid",
            _ => "unknown",
        }
    }
}

/// ノクターン出力形式
#[derive(Debug, Clone, PartialEq)]
pub enum NocturneOutputFormat {
    Json,
    Yaml,
    Php,
}

impl NocturneOutputFormat {
    pub fn as_str(&self) -> &str {
        match self {
            NocturneOutputFormat::Json => "json",
            NocturneOutputFormat::Yaml => "yaml",
            NocturneOutputFormat::Php => "php",
        }
    }
}