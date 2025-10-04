//! Historical price chart endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::charts::{HistoricalDividend, HistoricalPrice, IntradayPrice, StockSplit};
use crate::models::common::Timeframe;
use serde::Serialize;

/// Historical price chart API endpoints
pub struct Charts {
    client: FmpClient,
}

impl Charts {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get historical daily prices (EOD - End of Day)
    pub async fn get_historical_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<HistoricalPrice>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/historical-price-eod/full");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get intraday prices
    pub async fn get_intraday_prices(
        &self,
        symbol: &str,
        timeframe: Timeframe,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical-chart/{}", timeframe));
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get 1-minute intraday prices
    pub async fn get_1min_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::OneMinute, from, to)
            .await
    }

    /// Get 5-minute intraday prices
    pub async fn get_5min_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::FiveMinutes, from, to)
            .await
    }

    /// Get 15-minute intraday prices
    pub async fn get_15min_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::FifteenMinutes, from, to)
            .await
    }

    /// Get 30-minute intraday prices
    pub async fn get_30min_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::ThirtyMinutes, from, to)
            .await
    }

    /// Get 1-hour intraday prices
    pub async fn get_1hour_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::OneHour, from, to)
            .await
    }

    /// Get 4-hour intraday prices
    pub async fn get_4hour_prices(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IntradayPrice>> {
        self.get_intraday_prices(symbol, Timeframe::FourHours, from, to)
            .await
    }

    /// Get historical dividend data
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let dividends = client.charts().get_historical_dividends("AAPL").await?;
    /// for div in dividends.iter().take(5) {
    ///     println!("{}: ${:.2} (Adjusted: ${:.2})",
    ///         div.date, div.dividend, div.adj_dividend);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_dividends(&self, symbol: &str) -> Result<Vec<HistoricalDividend>> {
        self.client
            .get_with_query(
                &format!("v3/historical-price-full/stock_dividend/{}", symbol),
                &(),
            )
            .await
    }

    /// Get stock split history
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let splits = client.charts().get_stock_splits("AAPL").await?;
    /// for split in splits {
    ///     println!("{}: {}:{} split", split.date, split.numerator, split.denominator);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_stock_splits(&self, symbol: &str) -> Result<Vec<StockSplit>> {
        self.client
            .get_with_query(
                &format!("v3/historical-price-full/stock_split/{}", symbol),
                &(),
            )
            .await
    }

    /// Get survivor bias free EOD prices
    ///
    /// This includes data for delisted companies, providing a more complete
    /// picture of historical market data without survivor bias.
    ///
    /// # Arguments
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let prices = client.charts().get_survivor_bias_free_eod("2024-01-01").await?;
    /// println!("Total symbols on 2024-01-01: {}", prices.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_survivor_bias_free_eod(&self, date: &str) -> Result<Vec<HistoricalPrice>> {
        self.client
            .get_with_query(
                &format!("v4/batch-request-end-of-day-prices?date={}", date),
                &(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let _ = Charts::new(client);
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_prices() {
        let client = FmpClient::new().unwrap();
        let result = client
            .charts()
            .get_historical_prices("AAPL", None, None)
            .await;
        assert!(result.is_ok());
        let prices = result.unwrap();
        assert!(!prices.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_prices_with_date_range() {
        let client = FmpClient::new().unwrap();
        let result = client
            .charts()
            .get_historical_prices("AAPL", Some("2024-01-01"), Some("2024-12-31"))
            .await;
        assert!(result.is_ok());
        let prices = result.unwrap();
        assert!(!prices.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_1min_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_1min_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_5min_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_5min_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_15min_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_15min_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_30min_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_30min_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_1hour_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_1hour_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_4hour_prices() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_4hour_prices("AAPL", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_dividends() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_historical_dividends("AAPL").await;
        assert!(result.is_ok());
        let dividends = result.unwrap();
        assert!(!dividends.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_stock_splits() {
        let client = FmpClient::new().unwrap();
        let result = client.charts().get_stock_splits("AAPL").await;
        assert!(result.is_ok());
        // AAPL has had splits in the past
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_survivor_bias_free_eod() {
        let client = FmpClient::new().unwrap();
        let result = client
            .charts()
            .get_survivor_bias_free_eod("2024-01-01")
            .await;
        assert!(result.is_ok());
        let prices = result.unwrap();
        assert!(!prices.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_prices_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .charts()
            .get_historical_prices("INVALID_SYMBOL_XYZ123", None, None)
            .await;
        // Should handle gracefully
        if let Ok(prices) = result {
            assert!(prices.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_dividends_no_dividends() {
        let client = FmpClient::new().unwrap();
        // Some symbols may not have dividends
        let result = client.charts().get_historical_dividends("TSLA").await;
        assert!(result.is_ok());
        // May be empty if no dividends
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_stock_splits_no_splits() {
        let client = FmpClient::new().unwrap();
        // Some symbols may not have had splits
        let result = client.charts().get_stock_splits("MSFT").await;
        assert!(result.is_ok());
        // May be empty if no splits
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_intraday_prices_with_date_range() {
        let client = FmpClient::new().unwrap();
        let result = client
            .charts()
            .get_intraday_prices(
                "AAPL",
                Timeframe::FiveMinutes,
                Some("2024-01-01"),
                Some("2024-01-02"),
            )
            .await;
        assert!(result.is_ok());
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client
            .charts()
            .get_historical_prices("AAPL", None, None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.charts().get_historical_prices("", None, None).await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
