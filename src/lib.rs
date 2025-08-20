pub mod fetcher;
pub mod rating_scraper;

pub use fetcher::{FetchOptions, HtmlFetcher, UserAgentMode, RequestDelayConfig};
pub use rating_scraper::{NarouRatingScraper, RatingEntry};