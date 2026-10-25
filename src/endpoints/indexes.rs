//! Indexes endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::indexes::{IndexConstituent, IndexHistorical, IndexQuote, IndexSymbol},
};
use serde::{Deserialize, Serialize};

/// Indexes API endpoints
pub struct Indexes {
    client: FmpClient,
}

impl Indexes {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get list of available market indexes
    ///
    /// Returns all available market index symbols (S&P 500, Nasdaq, Dow Jones, etc.).
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let indexes = client.indexes().get_index_list().await?;
    /// for index in indexes.iter().take(10) {
    ///     println!("{}: {}", index.symbol, index.name.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_list(&self) -> Result<Vec<IndexSymbol>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/symbol/available-indexes");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get real-time index quote
    ///
    /// Returns current level and market data for an index.
    ///
    /// # Arguments
    /// * `symbol` - Index symbol (e.g., "^GSPC" for S&P 500, "^DJI" for Dow Jones)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let quote = client.indexes().get_index_quote("^GSPC").await?;
    /// if let Some(q) = quote.first() {
    ///     println!("S&P 500: {:.2}", q.price.unwrap_or(0.0));
    ///     println!("Change: {:+.2}%", q.changes_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_quote(&self, symbol: &str) -> Result<Vec<IndexQuote>> {
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

    /// Get historical index data
    ///
    /// Returns daily historical data for an index.
    ///
    /// # Arguments
    /// * `symbol` - Index symbol (e.g., "^GSPC")
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.indexes().get_index_historical("^GSPC", None, None).await?;
    /// for day in history.iter().take(5) {
    ///     println!("{}: {:.2}", day.date, day.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_historical(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IndexHistorical>> {
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
            historical: Vec<IndexHistorical>,
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

    /// Get index constituents
    ///
    /// Returns all component stocks that make up an index (e.g., S&P 500 companies).
    ///
    /// # Arguments
    /// * `symbol` - Index symbol (e.g., "^GSPC" for S&P 500 constituents)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let constituents = client.indexes().get_index_constituents("^GSPC").await?;
    /// println!("S&P 500 has {} components", constituents.len());
    /// for stock in constituents.iter().take(10) {
    ///     println!("{}: {} ({})",
    ///         stock.symbol,
    ///         stock.name.as_deref().unwrap_or("N/A"),
    ///         stock.sector.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_constituents(&self, symbol: &str) -> Result<Vec<IndexConstituent>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("{}_constituent", symbol));
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
    async fn test_get_index_list() {
        let client = FmpClient::new().unwrap();
        let result = client.indexes().get_index_list().await;
        assert!(result.is_ok());
        let indexes = result.unwrap();
        assert!(!indexes.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_index_quote() {
        let client = FmpClient::new().unwrap();
        let result = client.indexes().get_index_quote("^GSPC").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert!(quotes[0].price.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_index_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .indexes()
            .get_index_historical("^GSPC", None, None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_index_constituents() {
        let client = FmpClient::new().unwrap();
        let result = client.indexes().get_index_constituents("^GSPC").await;
        assert!(result.is_ok());
        let constituents = result.unwrap();
        assert!(!constituents.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_index_historical_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .indexes()
            .get_index_historical("^GSPC", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_major_indexes() {
        let client = FmpClient::new().unwrap();
        // S&P 500, Nasdaq, Dow Jones
        for symbol in &["^GSPC", "^IXIC", "^DJI"] {
            let result = client.indexes().get_index_quote(symbol).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_dow_jones_constituents() {
        let client = FmpClient::new().unwrap();
        let result = client.indexes().get_index_constituents("^DJI").await;
        assert!(result.is_ok());
        let constituents = result.unwrap();
        // Dow Jones has 30 companies
        assert!(constituents.len() <= 30);
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.indexes().get_index_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.indexes().get_index_quote("INVALIDINDEX123").await;
        // Should return empty or error
        if let Ok(quotes) = result {
            assert!(quotes.is_empty())
        }
    }
}
