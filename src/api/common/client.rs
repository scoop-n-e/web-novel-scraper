use crate::api::common::{
    error::{ApiError, Result},
    request::ApiRequest,
    response::{ApiResponse, OutputFormat, ResponseProcessor},
};
use async_trait::async_trait;
use reqwest;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

#[async_trait]
pub trait ApiClient: Send + Sync {
    type Request: ApiRequest + Send + Sync;
    type Response: ApiResponse;

    fn base_url(&self) -> &str;

    fn build_query_params(&self, request: &Self::Request) -> Vec<(String, String)> {
        request.to_query_params()
    }

    async fn parse_response(&self, data: Vec<u8>, format: &OutputFormat, gzip: bool) -> Result<Self::Response> {
        ResponseProcessor::process(data, format, gzip)
    }
}

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .map_err(|e| ApiError::Other(e.to_string()))?;

        Ok(HttpClient { client })
    }

    pub fn with_user_agent(user_agent: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .build()
            .map_err(|e| ApiError::Other(e.to_string()))?;

        Ok(HttpClient { client })
    }

    pub async fn execute<C>(&self, api_client: &C, request: &C::Request) -> Result<C::Response>
    where
        C: ApiClient + Send + Sync,
        C::Request: Send + Sync,
    {
        let query_params = api_client.build_query_params(request);
        let url = api_client.base_url();
        let format = request.output_format();
        let is_gzip = request.is_gzip();

        let response = self
            .client
            .get(url)
            .query(&query_params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ApiError::Network(
                reqwest::Error::from(response.error_for_status().unwrap_err())
            ));
        }

        let data = response.bytes().await?.to_vec();
        api_client.parse_response(data, &format, is_gzip).await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}