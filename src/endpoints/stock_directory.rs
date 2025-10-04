//! Stock directory and screener endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::company::{StockScreenerResult, TradableSymbol};
use serde::Serialize;

/// Stock directory API endpoints
pub struct StockDirectory {
    client: FmpClient,
}

/// Stock screener criteria
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenerCriteria {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_cap_more_than: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_cap_lower_than: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_more_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_lower_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_more_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_lower_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_more_than: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_lower_than: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dividend_more_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dividend_lower_than: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_etf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_actively_trading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl StockDirectory {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Screen stocks based on various criteria
    ///
    /// # Arguments
    /// * `criteria` - Screening criteria to filter stocks
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, endpoints::stock_directory::ScreenerCriteria};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let mut criteria = ScreenerCriteria::default();
    /// criteria.market_cap_more_than = Some(10_000_000_000); // $10B+
    /// criteria.beta_lower_than = Some(1.5);
    /// criteria.is_etf = Some(false);
    /// criteria.is_actively_trading = Some(true);
    /// criteria.limit = Some(100);
    ///
    /// let results = client.stock_directory().screen_stocks(&criteria).await?;
    /// for stock in results.iter().take(10) {
    ///     println!("{}: {} - ${:.2}B market cap",
    ///         stock.symbol,
    ///         stock.company_name,
    ///         stock.market_cap.unwrap_or(0.0) / 1_000_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screen_stocks(
        &self,
        criteria: &ScreenerCriteria,
    ) -> Result<Vec<StockScreenerResult>> {
        self.client
            .get_with_query("v3/stock-screener", criteria)
            .await
    }

    /// Get list of all available traded symbols
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let symbols = client.stock_directory().get_tradable_symbols().await?;
    /// println!("Total tradable symbols: {}", symbols.len());
    /// for symbol in symbols.iter().take(10) {
    ///     println!("{}: {}", symbol.symbol, symbol.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_tradable_symbols(&self) -> Result<Vec<TradableSymbol>> {
        self.client
            .get_with_query("v3/available-traded/list", &())
            .await
    }

    /// Get symbols by exchange
    ///
    /// # Arguments
    /// * `exchange` - Exchange code (e.g., "NASDAQ", "NYSE")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let symbols = client.stock_directory().get_symbols_by_exchange("NASDAQ").await?;
    /// println!("NASDAQ symbols: {}", symbols.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbols_by_exchange(&self, exchange: &str) -> Result<Vec<TradableSymbol>> {
        self.client
            .get_with_query(&format!("v3/symbol/{}", exchange), &())
            .await
    }

    /// Get list of all ETFs
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let etfs = client.stock_directory().get_etf_list().await?;
    /// println!("Total ETFs: {}", etfs.len());
    /// for etf in etfs.iter().take(10) {
    ///     println!("{}: {}", etf.symbol, etf.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_etf_list(&self) -> Result<Vec<TradableSymbol>> {
        self.client.get_with_query("v3/etf/list", &()).await
    }

    /// Check if symbol is available/valid
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol to check
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let symbols = client.stock_directory().is_symbol_available("AAPL").await?;
    /// if !symbols.is_empty() {
    ///     println!("AAPL is available: {}", symbols[0].name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_symbol_available(&self, symbol: &str) -> Result<Vec<TradableSymbol>> {
        self.client
            .get_with_query(&format!("v3/symbol/available-{}", symbol), &())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let _ = StockDirectory::new(client);
    }

    #[test]
    fn test_screener_criteria_default() {
        let criteria = ScreenerCriteria::default();
        assert!(criteria.market_cap_more_than.is_none());
        assert!(criteria.is_etf.is_none());
    }

    #[test]
    fn test_screener_criteria_builder() {
        let mut criteria = ScreenerCriteria::default();
        criteria.market_cap_more_than = Some(1_000_000_000);
        criteria.beta_lower_than = Some(1.5);
        criteria.is_actively_trading = Some(true);

        assert_eq!(criteria.market_cap_more_than, Some(1_000_000_000));
        assert_eq!(criteria.beta_lower_than, Some(1.5));
        assert_eq!(criteria.is_actively_trading, Some(true));
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_screen_stocks() {
        let client = FmpClient::new().unwrap();
        let mut criteria = ScreenerCriteria::default();
        criteria.market_cap_more_than = Some(10_000_000_000);
        criteria.is_etf = Some(false);
        criteria.limit = Some(10);

        let result = client.stock_directory().screen_stocks(&criteria).await;
        assert!(result.is_ok());
        let stocks = result.unwrap();
        assert!(!stocks.is_empty());
        assert!(stocks.len() <= 10);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_tradable_symbols() {
        let client = FmpClient::new().unwrap();
        let result = client.stock_directory().get_tradable_symbols().await;
        assert!(result.is_ok());
        let symbols = result.unwrap();
        assert!(!symbols.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_symbols_by_exchange() {
        let client = FmpClient::new().unwrap();
        let result = client
            .stock_directory()
            .get_symbols_by_exchange("NASDAQ")
            .await;
        assert!(result.is_ok());
        let symbols = result.unwrap();
        assert!(!symbols.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_list() {
        let client = FmpClient::new().unwrap();
        let result = client.stock_directory().get_etf_list().await;
        assert!(result.is_ok());
        let etfs = result.unwrap();
        assert!(!etfs.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_is_symbol_available() {
        let client = FmpClient::new().unwrap();
        let result = client.stock_directory().is_symbol_available("AAPL").await;
        assert!(result.is_ok());
        let symbols = result.unwrap();
        assert!(!symbols.is_empty());
        assert_eq!(symbols[0].symbol, "AAPL");
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_screen_stocks_empty_criteria() {
        let client = FmpClient::new().unwrap();
        let criteria = ScreenerCriteria::default();
        let result = client.stock_directory().screen_stocks(&criteria).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_screen_stocks_restrictive_criteria() {
        let client = FmpClient::new().unwrap();
        let mut criteria = ScreenerCriteria::default();
        criteria.market_cap_more_than = Some(1_000_000_000_000); // $1T+
        criteria.price_more_than = Some(500.0);
        criteria.limit = Some(5);

        let result = client.stock_directory().screen_stocks(&criteria).await;
        assert!(result.is_ok());
        // May return empty if criteria is too restrictive
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_is_symbol_available_invalid() {
        let client = FmpClient::new().unwrap();
        let result = client
            .stock_directory()
            .is_symbol_available("INVALID_XYZ123")
            .await;
        if let Ok(symbols) = result {
            assert!(symbols.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_symbols_by_exchange_invalid() {
        let client = FmpClient::new().unwrap();
        let result = client
            .stock_directory()
            .get_symbols_by_exchange("INVALID_EXCHANGE")
            .await;
        // Should either error or return empty list
        if let Ok(symbols) = result {
            assert!(symbols.is_empty());
        }
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.stock_directory().get_tradable_symbols().await;
        assert!(result.is_err());
    }
}
