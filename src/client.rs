//! HTTP client for interacting with the FMP API.

use crate::endpoints::*;
use crate::error::{Error, Result};
use reqwest::{Client, Response};
use std::sync::Arc;
use std::time::Duration;

/// Base URL for the FMP API
pub const FMP_API_BASE_URL: &str = "https://financialmodelingprep.com/stable";

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct FmpConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub base_url: String,
    /// Request timeout in seconds
    pub timeout: Duration,
}

impl Default for FmpConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: FMP_API_BASE_URL.to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// Builder for creating an FmpClient with custom configuration
pub struct FmpClientBuilder {
    config: FmpConfig,
}

impl FmpClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: FmpConfig::default(),
        }
    }

    /// Set the API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.config.api_key = api_key.into();
        self
    }

    /// Set the base URL (useful for testing)
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Build the client
    pub fn build(self) -> Result<FmpClient> {
        let api_key = if self.config.api_key.is_empty() {
            std::env::var("FMP_API_KEY").map_err(|_| Error::MissingApiKey)?
        } else {
            self.config.api_key
        };

        let http_client = Client::builder().timeout(self.config.timeout).build()?;

        Ok(FmpClient {
            inner: Arc::new(FmpClientInner {
                http_client,
                api_key,
                base_url: self.config.base_url,
            }),
        })
    }
}

impl Default for FmpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner client data shared via Arc
struct FmpClientInner {
    http_client: Client,
    api_key: String,
    base_url: String,
}

/// Main client for interacting with the FMP API
#[derive(Clone)]
pub struct FmpClient {
    inner: Arc<FmpClientInner>,
}

impl FmpClient {
    /// Create a new FMP client with the API key from environment variable
    pub fn new() -> Result<Self> {
        FmpClientBuilder::new().build()
    }

    /// Create a new FMP client with the provided API key
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        FmpClientBuilder::new().api_key(api_key).build()
    }

    /// Get a builder for creating a customized client
    pub fn builder() -> FmpClientBuilder {
        FmpClientBuilder::new()
    }

    /// Get the API key
    pub(crate) fn api_key(&self) -> &str {
        &self.inner.api_key
    }

    /// Get the base URL
    #[allow(dead_code)]
    pub(crate) fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Build a full URL from a path
    pub(crate) fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.inner.base_url, path.trim_start_matches('/'))
    }

    /// Execute a GET request
    #[allow(dead_code)]
    pub(crate) async fn get<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.inner.http_client.get(url).send().await?;
        self.handle_response(response).await
    }

    /// Execute a GET request with query parameters
    pub(crate) async fn get_with_query<T, Q>(&self, url: &str, query: &Q) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize,
    {
        let response = self.inner.http_client.get(url).query(query).send().await?;
        self.handle_response(response).await
    }

    /// Handle the HTTP response
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text).map_err(|e| {
                eprintln!("Failed to parse response: {}", text);
                Error::from(e)
            })
        } else {
            let status_code = status.as_u16();
            let error_text = response.text().await.unwrap_or_default();

            match status_code {
                429 => Err(Error::RateLimitExceeded),
                404 => Err(Error::NotFound(error_text)),
                _ => Err(Error::api(status_code, error_text)),
            }
        }
    }

    // Endpoint accessor methods

    /// Access company search endpoints
    pub fn company_search(&self) -> CompanySearch {
        CompanySearch::new(self.clone())
    }

    /// Access quote endpoints
    pub fn quote(&self) -> Quote {
        Quote::new(self.clone())
    }

    /// Access stock directory endpoints
    pub fn stock_directory(&self) -> StockDirectory {
        StockDirectory::new(self.clone())
    }

    /// Access company information endpoints
    pub fn company_info(&self) -> CompanyInfo {
        CompanyInfo::new(self.clone())
    }

    /// Access financial statements endpoints
    pub fn financials(&self) -> Financials {
        Financials::new(self.clone())
    }

    /// Access historical price chart endpoints
    pub fn charts(&self) -> Charts {
        Charts::new(self.clone())
    }

    /// Access economics endpoints
    pub fn economics(&self) -> Economics {
        Economics::new(self.clone())
    }

    /// Access earnings, dividends, and splits endpoints
    pub fn corporate_actions(&self) -> CorporateActions {
        CorporateActions::new(self.clone())
    }

    /// Access news endpoints
    pub fn news(&self) -> News {
        News::new(self.clone())
    }

    /// Access analyst endpoints
    pub fn analyst(&self) -> Analyst {
        Analyst::new(self.clone())
    }

    /// Access market performance endpoints
    pub fn market_performance(&self) -> MarketPerformance {
        MarketPerformance::new(self.clone())
    }

    /// Access ETF and mutual fund endpoints
    pub fn etf(&self) -> Etf {
        Etf::new(self.clone())
    }

    /// Access SEC filings endpoints
    pub fn sec_filings(&self) -> SecFilings {
        SecFilings::new(self.clone())
    }

    /// Access insider trading endpoints
    pub fn insider_trades(&self) -> InsiderTrades {
        InsiderTrades::new(self.clone())
    }

    /// Access index endpoints
    pub fn indexes(&self) -> Indexes {
        Indexes::new(self.clone())
    }

    /// Access commodity endpoints
    pub fn commodities(&self) -> Commodities {
        Commodities::new(self.clone())
    }

    /// Access DCF valuation endpoints
    pub fn dcf(&self) -> Dcf {
        Dcf::new(self.clone())
    }

    /// Access forex endpoints
    pub fn forex(&self) -> Forex {
        Forex::new(self.clone())
    }

    /// Access cryptocurrency endpoints
    pub fn crypto(&self) -> Crypto {
        Crypto::new(self.clone())
    }

    /// Access technical indicators endpoints
    pub fn technical_indicators(&self) -> TechnicalIndicators {
        TechnicalIndicators::new(self.clone())
    }

    /// Access institutional ownership (Form 13F) endpoints
    pub fn institutional(&self) -> Institutional {
        Institutional::new(self.clone())
    }

    /// Access senate/congress trading endpoints
    pub fn congress(&self) -> Congress {
        Congress::new(self.clone())
    }

    /// Access ESG (Environmental, Social, Governance) endpoints
    pub fn esg(&self) -> Esg {
        Esg::new(self.clone())
    }

    /// Access market hours endpoints
    pub fn market_hours(&self) -> MarketHours {
        MarketHours::new(self.clone())
    }

    /// Access mutual funds endpoints
    pub fn mutual_funds(&self) -> MutualFunds {
        MutualFunds::new(self.clone())
    }

    /// Access earnings transcript endpoints
    pub fn transcripts(&self) -> Transcripts {
        Transcripts::new(self.clone())
    }

    /// Access bulk data endpoints
    pub fn bulk(&self) -> Bulk {
        Bulk::new(self.clone())
    }
}

impl Default for FmpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create FmpClient from environment variable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = FmpClient::builder()
            .api_key("test_key")
            .base_url("https://test.example.com")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        assert_eq!(client.api_key(), "test_key");
        assert_eq!(client.base_url(), "https://test.example.com");
    }

    #[test]
    fn test_build_url() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();

        assert_eq!(
            client.build_url("/quote"),
            format!("{}/quote", FMP_API_BASE_URL)
        );

        assert_eq!(
            client.build_url("quote"),
            format!("{}/quote", FMP_API_BASE_URL)
        );
    }
}
