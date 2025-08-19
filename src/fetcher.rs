use anyhow::Result;
use fake_useragent::UserAgents;
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub enum UserAgentMode {
    Fixed,
    RandomEveryRequest,
}

pub struct HtmlFetcher {
    client: reqwest::Client,
    cookie_jar: Arc<Jar>,
    user_agent_mode: Arc<RwLock<UserAgentMode>>,
    fixed_user_agent: Arc<RwLock<Option<String>>>,
}

impl HtmlFetcher {
    pub fn new() -> Result<Self> {
        Self::with_mode(UserAgentMode::RandomEveryRequest)
    }

    pub fn with_mode(mode: UserAgentMode) -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()?;

        Ok(Self {
            client,
            cookie_jar,
            user_agent_mode: Arc::new(RwLock::new(mode)),
            fixed_user_agent: Arc::new(RwLock::new(None)),
        })
    }

    pub fn with_fixed_user_agent(user_agent: String) -> Result<Self> {
        let fetcher = Self::with_mode(UserAgentMode::Fixed)?;
        fetcher.set_user_agent(user_agent);
        Ok(fetcher)
    }

    pub fn set_user_agent(&self, user_agent: String) {
        *self.user_agent_mode.write().unwrap() = UserAgentMode::Fixed;
        *self.fixed_user_agent.write().unwrap() = Some(user_agent);
    }

    pub fn set_user_agent_from_random(&self) -> String {
        let user_agents = UserAgents::new();
        let ua = user_agents.random().to_string();
        self.set_user_agent(ua.clone());
        ua
    }

    pub fn set_random_mode(&self) {
        *self.user_agent_mode.write().unwrap() = UserAgentMode::RandomEveryRequest;
        *self.fixed_user_agent.write().unwrap() = None;
    }

    pub fn set_fixed_mode(&self) {
        *self.user_agent_mode.write().unwrap() = UserAgentMode::Fixed;
        if self.fixed_user_agent.read().unwrap().is_none() {
            self.set_user_agent_from_random();
        }
    }

    pub fn get_current_user_agent(&self) -> Option<String> {
        self.fixed_user_agent.read().unwrap().clone()
    }

    pub fn get_mode(&self) -> UserAgentMode {
        self.user_agent_mode.read().unwrap().clone()
    }

    fn resolve_user_agent(&self, custom_user_agent: Option<&str>) -> String {
        if let Some(ua) = custom_user_agent {
            return ua.to_string();
        }

        let mode = self.user_agent_mode.read().unwrap();
        match &*mode {
            UserAgentMode::Fixed => {
                let fixed_ua = self.fixed_user_agent.read().unwrap();
                fixed_ua.clone().unwrap_or_else(|| {
                    drop(fixed_ua);
                    self.set_user_agent_from_random()
                })
            }
            UserAgentMode::RandomEveryRequest => {
                let user_agents = UserAgents::new();
                user_agents.random().to_string()
            }
        }
    }

    pub async fn fetch_with_options(
        &self,
        url: &str,
        cookies: Option<Vec<(&str, &str)>>,
        custom_user_agent: Option<&str>,
    ) -> Result<String> {
        let mut headers = HeaderMap::new();

        let user_agent = self.resolve_user_agent(custom_user_agent);
        headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent)?);

        if let Some(cookies) = cookies {
            let cookie_string = cookies
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join("; ");
            headers.insert(COOKIE, HeaderValue::from_str(&cookie_string)?);
        }

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        let html = response.text().await?;
        Ok(html)
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        self.fetch_with_options(url, None, None).await
    }

    pub fn add_cookie(&self, url: &str, cookie_str: &str) -> Result<()> {
        let url = url.parse::<reqwest::Url>()?;
        self.cookie_jar.add_cookie_str(cookie_str, &url);
        Ok(())
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
        
        fetcher.set_random_mode();
        assert_eq!(fetcher.get_current_user_agent(), None);
        
        let ua = fetcher.set_user_agent_from_random();
        assert_eq!(fetcher.get_current_user_agent(), Some(ua));
    }

    #[test]
    fn test_fixed_mode_initialization() {
        let fetcher = HtmlFetcher::with_fixed_user_agent("TestBot/2.0".to_string()).unwrap();
        assert_eq!(fetcher.get_current_user_agent(), Some("TestBot/2.0".to_string()));
    }
}