//! Insider trading endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::institutional::{InsiderTrade, InsiderTradingRss};

/// Insider Trading API endpoints
pub struct InsiderTrades {
    client: FmpClient,
}

impl InsiderTrades {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get insider trading transactions for a symbol
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Optional limit on number of results
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let trades = client.insider_trades()
    ///         .get_insider_trading("AAPL", Some(20)).await?;
    ///     
    ///     println!("Recent insider trades:");
    ///     for trade in &trades {
    ///         println!("  {}: {} {} {} shares at ${:.2}",
    ///                  trade.transaction_date,
    ///                  trade.reporting_name,
    ///                  trade.transaction_type,
    ///                  trade.securities_transacted,
    ///                  trade.price);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_insider_trading(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<InsiderTrade>> {
        let mut url = format!("/api/v4/insider-trading?symbol={}", symbol);
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }
        self.client.get(&url).await
    }

    /// Get insider trading RSS feed (recent filings)
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional limit on number of results
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let filings = client.insider_trades()
    ///         .get_insider_trading_rss(Some(50)).await?;
    ///     
    ///     println!("Recent insider trading filings:");
    ///     for filing in &filings {
    ///         println!("  {}: {} ({})",
    ///                  filing.filing_date,
    ///                  filing.symbol,
    ///                  filing.company_name.as_deref().unwrap_or("N/A"));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_insider_trading_rss(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<InsiderTradingRss>> {
        let url = if let Some(limit) = limit {
            format!("/api/v4/insider-trading-rss-feed?limit={}", limit)
        } else {
            "/api/v4/insider-trading-rss-feed".to_string()
        };
        self.client.get(&url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let _ = InsiderTrades::new(client);
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading() {
        let client = FmpClient::new().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("AAPL", Some(20))
            .await;
        assert!(result.is_ok());
        let trades = result.unwrap();
        assert!(!trades.is_empty());
        assert!(trades.len() <= 20);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading_rss() {
        let client = FmpClient::new().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading_rss(Some(50))
            .await;
        assert!(result.is_ok());
        let filings = result.unwrap();
        assert!(!filings.is_empty());
        assert!(filings.len() <= 50);
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("AAPL", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading_rss_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client.insider_trades().get_insider_trading_rss(None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("INVALID_XYZ123", Some(10))
            .await;
        // Should either return empty vec or error
        if let Ok(trades) = result {
            assert!(trades.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_trading_large_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("AAPL", Some(100))
            .await;
        assert!(result.is_ok());
        let trades = result.unwrap();
        assert!(trades.len() <= 100);
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("AAPL", Some(10))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client
            .insider_trades()
            .get_insider_trading("", Some(10))
            .await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
