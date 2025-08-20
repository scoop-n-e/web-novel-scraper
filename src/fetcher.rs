use anyhow::Result;
use fake_useragent::UserAgents;
use rand::Rng;
use reqwest::{
    cookie::Jar,
    header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT},
    Client, StatusCode, Url,
};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Clone, Debug, PartialEq)]
pub enum UserAgentMode {
    Fixed(Option<String>),
    RandomEveryRequest,
}

impl Default for UserAgentMode {
    fn default() -> Self {
        Self::RandomEveryRequest
    }
}

#[derive(Clone, Debug)]
pub struct RequestDelayConfig {
    min_delay_ms: u64,
    max_delay_ms: u64,
    enabled: bool,
}

impl Default for RequestDelayConfig {
    fn default() -> Self {
        Self {
            min_delay_ms: 1000,
            max_delay_ms: 3000,
            enabled: true,
        }
    }
}

impl RequestDelayConfig {
    pub fn new(min_delay_ms: u64, max_delay_ms: u64) -> Self {
        assert!(min_delay_ms <= max_delay_ms, "min_delay must be <= max_delay");
        Self {
            min_delay_ms,
            max_delay_ms,
            enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            min_delay_ms: 0,
            max_delay_ms: 0,
            enabled: false,
        }
    }

    fn get_random_delay(&self) -> Duration {
        if !self.enabled {
            return Duration::from_millis(0);
        }

        if self.min_delay_ms == self.max_delay_ms {
            return Duration::from_millis(self.min_delay_ms);
        }

        let mut rng = rand::thread_rng();
        let delay_ms = rng.gen_range(self.min_delay_ms..=self.max_delay_ms);
        Duration::from_millis(delay_ms)
    }
}

#[derive(Clone)]
pub struct HtmlFetcher {
    client: Client,
    cookie_jar: Arc<Jar>,
    user_agent_mode: Arc<RwLock<UserAgentMode>>,
    delay_config: Arc<RwLock<RequestDelayConfig>>,
    last_request_time: Arc<RwLock<Option<Instant>>>,
}

pub struct FetchOptions<'a> {
    pub cookies: Option<Vec<(&'a str, &'a str)>>,
    pub custom_user_agent: Option<&'a str>,
}

impl<'a> Default for FetchOptions<'a> {
    fn default() -> Self {
        Self {
            cookies: None,
            custom_user_agent: None,
        }
    }
}

impl Default for HtmlFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default HtmlFetcher")
    }
}

impl HtmlFetcher {
    pub fn new() -> Result<Self> {
        Self::with_mode(UserAgentMode::default())
    }

    pub fn with_mode(mode: UserAgentMode) -> Result<Self> {
        Self::with_config(mode, std::time::Duration::from_secs(10))
    }

    pub fn with_config(mode: UserAgentMode, timeout: std::time::Duration) -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .timeout(timeout)
            .connection_verbose(false)
            .pool_max_idle_per_host(1)
            .build()?;

        Ok(Self {
            client,
            cookie_jar,
            user_agent_mode: Arc::new(RwLock::new(mode)),
            delay_config: Arc::new(RwLock::new(RequestDelayConfig::default())),
            last_request_time: Arc::new(RwLock::new(None)),
        })
    }

    pub fn with_fixed_user_agent(user_agent: String) -> Result<Self> {
        Self::with_mode(UserAgentMode::Fixed(Some(user_agent)))
    }

    pub fn set_user_agent(&self, user_agent: String) {
        *self.user_agent_mode.write().unwrap() = UserAgentMode::Fixed(Some(user_agent));
    }

    pub fn set_user_agent_from_random(&self) -> String {
        let ua = Self::generate_random_user_agent();
        self.set_user_agent(ua.clone());
        ua
    }

    pub fn set_random_mode(&self) {
        *self.user_agent_mode.write().unwrap() = UserAgentMode::RandomEveryRequest;
    }

    pub fn set_fixed_mode(&self) {
        let mut mode = self.user_agent_mode.write().unwrap();
        if !matches!(*mode, UserAgentMode::Fixed(_)) {
            let ua = Self::generate_random_user_agent();
            *mode = UserAgentMode::Fixed(Some(ua));
        }
    }

    pub fn get_current_user_agent(&self) -> Option<String> {
        match &*self.user_agent_mode.read().unwrap() {
            UserAgentMode::Fixed(ua) => ua.clone(),
            UserAgentMode::RandomEveryRequest => None,
        }
    }

    pub fn get_mode(&self) -> UserAgentMode {
        self.user_agent_mode.read().unwrap().clone()
    }

    pub fn set_delay_config(&self, config: RequestDelayConfig) {
        *self.delay_config.write().unwrap() = config;
    }

    pub fn get_delay_config(&self) -> RequestDelayConfig {
        self.delay_config.read().unwrap().clone()
    }

    pub fn disable_delay(&self) {
        self.delay_config.write().unwrap().enabled = false;
    }

    pub fn enable_delay(&self) {
        self.delay_config.write().unwrap().enabled = true;
    }

    async fn apply_request_delay(&self) {
        let config = self.delay_config.read().unwrap().clone();
        if !config.enabled {
            return;
        }

        let delay = config.get_random_delay();
        
        let should_wait = {
            let mut last_time = self.last_request_time.write().unwrap();
            if let Some(last) = *last_time {
                let elapsed = last.elapsed();
                if elapsed < delay {
                    Some(delay - elapsed)
                } else {
                    *last_time = Some(Instant::now());
                    None
                }
            } else {
                *last_time = Some(Instant::now());
                None
            }
        };

        if let Some(wait_duration) = should_wait {
            sleep(wait_duration).await;
            *self.last_request_time.write().unwrap() = Some(Instant::now());
        }
    }

    fn generate_random_user_agent() -> String {
        UserAgents::new().random().to_string()
    }

    fn resolve_user_agent(&self, custom_user_agent: Option<&str>) -> String {
        custom_user_agent.map(String::from).unwrap_or_else(|| {
            match &*self.user_agent_mode.read().unwrap() {
                UserAgentMode::Fixed(Some(ua)) => ua.clone(),
                UserAgentMode::Fixed(None) => {
                    let ua = Self::generate_random_user_agent();
                    *self.user_agent_mode.write().unwrap() = UserAgentMode::Fixed(Some(ua.clone()));
                    ua
                }
                UserAgentMode::RandomEveryRequest => Self::generate_random_user_agent(),
            }
        })
    }

    fn build_headers(&self, user_agent: &str, cookies: Option<Vec<(&str, &str)>>) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);

        if let Some(cookies) = cookies {
            let cookie_string = cookies
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join("; ");
            headers.insert(COOKIE, HeaderValue::from_str(&cookie_string)?);
        }

        Ok(headers)
    }

    pub async fn fetch_with_options(
        &self,
        url: &str,
        options: FetchOptions<'_>,
    ) -> Result<String> {
        self.apply_request_delay().await;

        let user_agent = self.resolve_user_agent(options.custom_user_agent);
        let headers = self.build_headers(&user_agent, options.cookies)?;

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        self.validate_response_status(response.status())?;
        
        let html = response.text().await?;
        Ok(html)
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        self.fetch_with_options(url, FetchOptions::default()).await
    }

    fn validate_response_status(&self, status: StatusCode) -> Result<()> {
        if !status.is_success() {
            anyhow::bail!("HTTP request failed with status: {}", status);
        }
        Ok(())
    }

    pub fn add_cookie(&self, url: &str, cookie_str: &str) -> Result<()> {
        let url = url.parse::<Url>()?;
        self.cookie_jar.add_cookie_str(cookie_str, &url);
        Ok(())
    }

    pub fn clear_cookies(&self) {
        // Cookie jar is wrapped in Arc and cannot be replaced
        // This would require redesigning the cookie storage approach
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetcher_creation() {
        let fetcher = HtmlFetcher::new();
        assert!(fetcher.is_ok());
    }

    #[test]
    fn test_user_agent_modes() {
        let fetcher = HtmlFetcher::new().unwrap();
        
        fetcher.set_user_agent("MyBot/1.0".to_string());
        assert_eq!(fetcher.get_current_user_agent(), Some("MyBot/1.0".to_string()));
        assert!(matches!(fetcher.get_mode(), UserAgentMode::Fixed(_)));
        
        fetcher.set_random_mode();
        assert_eq!(fetcher.get_current_user_agent(), None);
        assert_eq!(fetcher.get_mode(), UserAgentMode::RandomEveryRequest);
        
        let ua = fetcher.set_user_agent_from_random();
        assert_eq!(fetcher.get_current_user_agent(), Some(ua));
    }

    #[test]
    fn test_fixed_mode_initialization() {
        let fetcher = HtmlFetcher::with_fixed_user_agent("TestBot/2.0".to_string()).unwrap();
        assert_eq!(fetcher.get_current_user_agent(), Some("TestBot/2.0".to_string()));
    }

    #[test]
    fn test_mode_switching() {
        let fetcher = HtmlFetcher::new().unwrap();
        
        assert_eq!(fetcher.get_mode(), UserAgentMode::RandomEveryRequest);
        
        fetcher.set_fixed_mode();
        assert!(matches!(fetcher.get_mode(), UserAgentMode::Fixed(_)));
        assert!(fetcher.get_current_user_agent().is_some());
    }

    #[test]
    fn test_delay_config() {
        let fetcher = HtmlFetcher::new().unwrap();
        
        let default_config = fetcher.get_delay_config();
        assert!(default_config.enabled);
        assert_eq!(default_config.min_delay_ms, 1000);
        assert_eq!(default_config.max_delay_ms, 3000);
        
        let custom_config = RequestDelayConfig::new(500, 1500);
        fetcher.set_delay_config(custom_config);
        let retrieved_config = fetcher.get_delay_config();
        assert_eq!(retrieved_config.min_delay_ms, 500);
        assert_eq!(retrieved_config.max_delay_ms, 1500);
        assert!(retrieved_config.enabled);
        
        fetcher.disable_delay();
        assert!(!fetcher.get_delay_config().enabled);
        
        fetcher.enable_delay();
        assert!(fetcher.get_delay_config().enabled);
    }

    #[test]
    fn test_delay_config_disabled() {
        let config = RequestDelayConfig::disabled();
        assert!(!config.enabled);
        assert_eq!(config.get_random_delay(), Duration::from_millis(0));
    }

    #[test]
    fn test_delay_config_random_range() {
        let config = RequestDelayConfig::new(100, 500);
        for _ in 0..10 {
            let delay = config.get_random_delay();
            assert!(delay >= Duration::from_millis(100));
            assert!(delay <= Duration::from_millis(500));
        }
    }

    #[test]
    fn test_delay_config_fixed_delay() {
        let config = RequestDelayConfig::new(1000, 1000);
        let delay = config.get_random_delay();
        assert_eq!(delay, Duration::from_millis(1000));
    }

    #[tokio::test]
    async fn test_fetch_with_delay() {
        let fetcher = HtmlFetcher::new().unwrap();
        fetcher.set_delay_config(RequestDelayConfig::new(100, 200));
        
        let start = Instant::now();
        
        // First request should not have delay
        match fetcher.fetch("https://httpbin.org/status/200").await {
            Ok(_) | Err(_) => {}
        }
        
        // Second request should have delay
        match fetcher.fetch("https://httpbin.org/status/200").await {
            Ok(_) | Err(_) => {}
        }
        
        let elapsed = start.elapsed();
        
        // The second request should have been delayed
        // We check if at least 100ms passed (minimum delay)
        assert!(elapsed >= Duration::from_millis(100), "Expected delay was not applied");
    }
}