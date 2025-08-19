use anyhow::Result;
use fake_useragent::UserAgents;
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use std::sync::Arc;

pub struct HtmlFetcher {
    client: reqwest::Client,
    cookie_jar: Arc<Jar>,
}

impl HtmlFetcher {
    pub fn new() -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()?;

        Ok(Self { client, cookie_jar })
    }

    pub async fn fetch_with_options(
        &self,
        url: &str,
        cookies: Option<Vec<(&str, &str)>>,
        custom_user_agent: Option<&str>,
    ) -> Result<String> {
        let mut headers = HeaderMap::new();

        let user_agent = if let Some(ua) = custom_user_agent {
            ua.to_string()
        } else {
            let user_agents = UserAgents::new();
            user_agents.random().to_string()
        };
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
}