pub mod fetcher;
pub mod rating_scraper;
pub mod novel_scraper;
pub mod api;

pub use fetcher::{FetchOptions, HtmlFetcher, UserAgentMode, RequestDelayConfig};
pub use rating_scraper::{NarouRatingScraper, RatingEntry};
pub use novel_scraper::{NarouNovelScraper, NovelContent, NovelType, Episode};
pub use api::narou;
pub use api::nocturne;