//! Cryptocurrency endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::crypto::{CryptoHistorical, CryptoIntraday, CryptoQuote, CryptoSymbol},
};
use serde::{Deserialize, Serialize};

/// Crypto API endpoints
pub struct Crypto {
    client: FmpClient,
}

impl Crypto {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get list of available cryptocurrencies
    ///
    /// Returns all available cryptocurrency symbols and their information.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let cryptos = client.crypto().get_crypto_list().await?;
    /// for crypto in cryptos.iter().take(10) {
    ///     println!("{}: {}", crypto.symbol, crypto.name.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crypto_list(&self) -> Result<Vec<CryptoSymbol>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/symbol/available-cryptocurrencies");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get real-time cryptocurrency quote
    ///
    /// Returns current price and market data for a cryptocurrency.
    ///
    /// # Arguments
    /// * `symbol` - Crypto symbol (e.g., "BTCUSD", "ETHUSD")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let quote = client.crypto().get_crypto_quote("BTCUSD").await?;
    /// if let Some(q) = quote.first() {
    ///     println!("BTC Price: ${:.2}", q.price.unwrap_or(0.0));
    ///     println!("24h Change: {:+.2}%", q.changes_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crypto_quote(&self, symbol: &str) -> Result<Vec<CryptoQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/quote/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical cryptocurrency prices
    ///
    /// Returns daily historical price data for a cryptocurrency.
    ///
    /// # Arguments
    /// * `symbol` - Crypto symbol (e.g., "BTCUSD")
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.crypto().get_crypto_historical("BTCUSD", None, None).await?;
    /// for day in history.iter().take(5) {
    ///     println!("{}: ${:.2} (Vol: {})", day.date, day.close, day.volume);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crypto_historical(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CryptoHistorical>> {
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
            .build_url(&format!("/historical-price-full/{}", symbol));

        #[derive(Deserialize)]
        struct Response {
            historical: Vec<CryptoHistorical>,
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

    /// Get intraday cryptocurrency prices
    ///
    /// Returns intraday price data at various intervals (1min, 5min, 15min, 30min, 1hour, 4hour).
    ///
    /// # Arguments
    /// * `symbol` - Crypto symbol (e.g., "BTCUSD")
    /// * `interval` - Time interval ("1min", "5min", "15min", "30min", "1hour", "4hour")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let intraday = client.crypto().get_crypto_intraday("BTCUSD", "5min").await?;
    /// for tick in intraday.iter().take(10) {
    ///     println!("{}: ${:.2}", tick.date, tick.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crypto_intraday(
        &self,
        symbol: &str,
        interval: &str,
    ) -> Result<Vec<CryptoIntraday>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical-chart/{}/{}", interval, symbol));
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
    async fn test_get_crypto_list() {
        let client = FmpClient::new().unwrap();
        let result = client.crypto().get_crypto_list().await;
        assert!(result.is_ok());
        let cryptos = result.unwrap();
        assert!(!cryptos.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_crypto_quote() {
        let client = FmpClient::new().unwrap();
        let result = client.crypto().get_crypto_quote("BTCUSD").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert!(quotes[0].price.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_crypto_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .crypto()
            .get_crypto_historical("BTCUSD", None, None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_crypto_intraday() {
        let client = FmpClient::new().unwrap();
        let result = client.crypto().get_crypto_intraday("BTCUSD", "5min").await;
        assert!(result.is_ok());
        let intraday = result.unwrap();
        assert!(!intraday.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_crypto_historical_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .crypto()
            .get_crypto_historical("BTCUSD", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_crypto_quotes() {
        let client = FmpClient::new().unwrap();
        for symbol in &["BTCUSD", "ETHUSD", "ADAUSD"] {
            let result = client.crypto().get_crypto_quote(symbol).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_intervals() {
        let client = FmpClient::new().unwrap();
        for interval in &["1min", "5min", "15min", "1hour"] {
            let result = client
                .crypto()
                .get_crypto_intraday("BTCUSD", interval)
                .await;
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
        let result = client.crypto().get_crypto_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.crypto().get_crypto_quote("INVALIDCRYPTO123").await;
        // Should return empty or error
        match result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable for invalid crypto
        }
    }
}
