//! Forex (foreign exchange) endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::crypto::{ForexHistorical, ForexIntraday, ForexPair, ForexQuote},
};
use serde::Serialize;

/// Forex API endpoints
pub struct Forex {
    client: FmpClient,
}

impl Forex {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get list of available forex pairs
    ///
    /// Returns all available forex currency pairs.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let pairs = client.forex().get_forex_list().await?;
    /// for pair in pairs.iter().take(10) {
    ///     println!("{}: {}", pair.symbol, pair.name.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_forex_list(&self) -> Result<Vec<ForexPair>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url("/symbol/available-forex-currency-pairs");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get real-time forex quote
    ///
    /// Returns current exchange rate for a forex pair.
    ///
    /// # Arguments
    /// * `pair` - Forex pair (e.g., "EURUSD", "GBPUSD")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let quote = client.forex().get_forex_quote("EURUSD").await?;
    /// if let Some(q) = quote.first() {
    ///     println!("EUR/USD: {:.5}", q.price.unwrap_or(0.0));
    ///     println!("Change: {:+.2}%", q.changes_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_forex_quote(&self, pair: &str) -> Result<Vec<ForexQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/quote/{}", pair));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical forex rates
    ///
    /// Returns daily historical exchange rates for a forex pair.
    ///
    /// # Arguments
    /// * `pair` - Forex pair (e.g., "EURUSD")
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.forex().get_forex_historical("EURUSD", None, None).await?;
    /// for day in history.iter().take(5) {
    ///     println!("{}: {:.5}", day.date, day.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_forex_historical(
        &self,
        pair: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<ForexHistorical>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical-price-full/{}", pair));

        #[derive(serde::Deserialize)]
        struct Response {
            historical: Vec<ForexHistorical>,
        }

        let response: Response = self
            .client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await?;

        Ok(response.historical)
    }

    /// Get intraday forex rates
    ///
    /// Returns intraday exchange rate data at various intervals.
    ///
    /// # Arguments
    /// * `pair` - Forex pair (e.g., "EURUSD")
    /// * `interval` - Time interval ("1min", "5min", "15min", "30min", "1hour", "4hour")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let intraday = client.forex().get_forex_intraday("EURUSD", "5min").await?;
    /// for tick in intraday.iter().take(10) {
    ///     println!("{}: {:.5}", tick.date, tick.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_forex_intraday(
        &self,
        pair: &str,
        interval: &str,
    ) -> Result<Vec<ForexIntraday>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical-chart/{}/{}", interval, pair));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_forex_list() {
        let client = FmpClient::new().unwrap();
        let result = client.forex().get_forex_list().await;
        assert!(result.is_ok());
        let pairs = result.unwrap();
        assert!(!pairs.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_forex_quote() {
        let client = FmpClient::new().unwrap();
        let result = client.forex().get_forex_quote("EURUSD").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert!(quotes[0].price.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_forex_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .forex()
            .get_forex_historical("EURUSD", None, None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_forex_intraday() {
        let client = FmpClient::new().unwrap();
        let result = client.forex().get_forex_intraday("EURUSD", "5min").await;
        assert!(result.is_ok());
        let intraday = result.unwrap();
        assert!(!intraday.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_forex_historical_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .forex()
            .get_forex_historical("EURUSD", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_forex_pairs() {
        let client = FmpClient::new().unwrap();
        for pair in &["EURUSD", "GBPUSD", "USDJPY"] {
            let result = client.forex().get_forex_quote(pair).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_forex_intervals() {
        let client = FmpClient::new().unwrap();
        for interval in &["1min", "5min", "15min", "1hour"] {
            let result = client.forex().get_forex_intraday("EURUSD", interval).await;
            assert!(result.is_ok());
        }
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.forex().get_forex_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_pair() {
        let client = FmpClient::new().unwrap();
        let result = client.forex().get_forex_quote("INVALIDPAIR123").await;
        // Should return empty or error
        if let Ok(data) = result {
            assert!(data.is_empty())
        }
    }
}
