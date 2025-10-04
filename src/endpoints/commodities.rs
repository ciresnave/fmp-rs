//! Commodities endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::commodities::{
        CommodityHistorical, CommodityIntraday, CommodityQuote, CommoditySymbol,
    },
};
use serde::{Deserialize, Serialize};

/// Commodities API endpoints
pub struct Commodities {
    client: FmpClient,
}

impl Commodities {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get list of available commodities
    ///
    /// Returns all available commodity symbols (gold, silver, oil, etc.).
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let commodities = client.commodities().get_commodity_list().await?;
    /// for commodity in commodities.iter().take(10) {
    ///     println!("{}: {}", commodity.symbol, commodity.name.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_commodity_list(&self) -> Result<Vec<CommoditySymbol>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/symbol/available-commodities");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get real-time commodity quote
    ///
    /// Returns current price and market data for a commodity.
    ///
    /// # Arguments
    /// * `symbol` - Commodity symbol (e.g., "GCUSD" for gold, "CLUSD" for crude oil)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let quote = client.commodities().get_commodity_quote("GCUSD").await?;
    /// if let Some(q) = quote.first() {
    ///     println!("Gold Price: ${:.2}/oz", q.price.unwrap_or(0.0));
    ///     println!("Change: {:+.2}%", q.changes_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_commodity_quote(&self, symbol: &str) -> Result<Vec<CommodityQuote>> {
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

    /// Get historical commodity prices
    ///
    /// Returns daily historical price data for a commodity.
    ///
    /// # Arguments
    /// * `symbol` - Commodity symbol (e.g., "GCUSD" for gold)
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.commodities().get_commodity_historical("GCUSD", None, None).await?;
    /// for day in history.iter().take(5) {
    ///     println!("{}: ${:.2}/oz", day.date, day.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_commodity_historical(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CommodityHistorical>> {
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
            historical: Vec<CommodityHistorical>,
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

    /// Get intraday commodity prices
    ///
    /// Returns intraday price data at various intervals.
    ///
    /// # Arguments
    /// * `symbol` - Commodity symbol (e.g., "GCUSD")
    /// * `interval` - Time interval ("1min", "5min", "15min", "30min", "1hour", "4hour")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let intraday = client.commodities().get_commodity_intraday("GCUSD", "5min").await?;
    /// for tick in intraday.iter().take(10) {
    ///     println!("{}: ${:.2}", tick.date, tick.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_commodity_intraday(
        &self,
        symbol: &str,
        interval: &str,
    ) -> Result<Vec<CommodityIntraday>> {
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
    async fn test_get_commodity_list() {
        let client = FmpClient::new().unwrap();
        let result = client.commodities().get_commodity_list().await;
        assert!(result.is_ok());
        let commodities = result.unwrap();
        assert!(!commodities.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_commodity_quote() {
        let client = FmpClient::new().unwrap();
        let result = client.commodities().get_commodity_quote("GCUSD").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert!(quotes[0].price.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_commodity_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .commodities()
            .get_commodity_historical("GCUSD", None, None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_commodity_intraday() {
        let client = FmpClient::new().unwrap();
        let result = client
            .commodities()
            .get_commodity_intraday("GCUSD", "5min")
            .await;
        assert!(result.is_ok());
        let intraday = result.unwrap();
        assert!(!intraday.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_commodity_historical_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .commodities()
            .get_commodity_historical("GCUSD", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_commodities() {
        let client = FmpClient::new().unwrap();
        // Gold, Crude Oil, Silver
        for symbol in &["GCUSD", "CLUSD", "SIUSD"] {
            let result = client.commodities().get_commodity_quote(symbol).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_intervals() {
        let client = FmpClient::new().unwrap();
        for interval in &["1min", "5min", "15min", "1hour"] {
            let result = client
                .commodities()
                .get_commodity_intraday("GCUSD", interval)
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
        let result = client.commodities().get_commodity_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .commodities()
            .get_commodity_quote("INVALIDCOMMODITY123")
            .await;
        // Should return empty or error
        match result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable for invalid commodity
        }
    }
}
