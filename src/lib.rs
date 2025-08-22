pub mod fetcher;
pub mod rating_scraper;
pub mod novel_scraper;
pub mod api_narou;
pub mod api_user;
pub mod api_hall_of_fame;
pub mod api_ranking;
pub mod api_nocturne;
pub mod api;

pub use fetcher::{FetchOptions, HtmlFetcher, UserAgentMode, RequestDelayConfig};
pub use rating_scraper::{NarouRatingScraper, RatingEntry};
pub use novel_scraper::{NarouNovelScraper, NovelContent, NovelType, Episode};

pub use api_narou::{NarouNovelApiClient, NarouNovelApiRequest, NarouNovelApiResponse, NarouNovelInfo};
pub use api_user::{NarouUserApiClient, NarouUserApiRequest, NarouUserApiResponse, NarouUserInfo};
pub use api_hall_of_fame::{NarouHallOfFameApiClient, NarouHallOfFameApiRequest, NarouHallOfFameApiResponse, HallOfFameEntry};
pub use api_ranking::{NarouRankingApiClient, NarouRankingApiRequest, NarouRankingApiResponse, RankingEntry};
pub use api_nocturne::{NocturneNovelApiClient, NocturneNovelApiRequest, NocturneNovelApiResponse, NocturneNovelInfo};