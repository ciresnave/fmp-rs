//! Institutional holdings endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::institutional::{
    CikMapper, FailToDeliver, InstitutionalHolder, InstitutionalPortfolioComposition,
    InstitutionalPortfolioDate,
};

/// Institutional API endpoints
pub struct Institutional {
    client: FmpClient,
}

impl Institutional {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get institutional holders for a symbol
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let holders = client.institutional().get_institutional_holders("AAPL").await?;
    ///     
    ///     println!("Top institutional holders:");
    ///     for holder in holders.iter().take(10) {
    ///         println!("  {}: {} shares", holder.holder, holder.shares);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_institutional_holders(
        &self,
        symbol: &str,
    ) -> Result<Vec<InstitutionalHolder>> {
        self.client
            .get(&format!("/api/v3/institutional-holder/{}", symbol))
            .await
    }

    /// Get portfolio composition by CIK
    ///
    /// # Arguments
    ///
    /// * `cik` - CIK number (e.g., "0001067983" for Berkshire Hathaway)
    /// * `date` - Optional date in YYYY-MM-DD format
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     // Berkshire Hathaway's portfolio
    ///     let portfolio = client.institutional()
    ///         .get_portfolio_composition("0001067983", None).await?;
    ///     
    ///     println!("Portfolio holdings:");
    ///     for holding in portfolio.iter().take(10) {
    ///         println!("  {}: {} shares (${:.2}B)",
    ///                  holding.symbol, holding.shares,
    ///                  holding.value / 1_000_000_000.0);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_portfolio_composition(
        &self,
        cik: &str,
        date: Option<&str>,
    ) -> Result<Vec<InstitutionalPortfolioComposition>> {
        let mut url = format!(
            "/api/v3/institutional-ownership/portfolio-composition?cik={}",
            cik
        );
        if let Some(date) = date {
            url.push_str(&format!("&date={}", date));
        }
        self.client.get(&url).await
    }

    /// Get institutional portfolio holdings summary
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `include_current_quarter` - Whether to include current quarter data
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let holdings = client.institutional()
    ///         .get_portfolio_holdings("AAPL", Some(true)).await?;
    ///     
    ///     for holding in &holdings {
    ///         println!("CIK {}: {} shares", holding.cik, holding.shares);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_portfolio_holdings(
        &self,
        symbol: &str,
        include_current_quarter: Option<bool>,
    ) -> Result<Vec<InstitutionalPortfolioComposition>> {
        let mut url = format!(
            "/api/v3/institutional-ownership/symbol-ownership?symbol={}",
            symbol
        );
        if let Some(include) = include_current_quarter {
            url.push_str(&format!("&includeCurrentQuarter={}", include));
        }
        self.client.get(&url).await
    }

    /// Get institutional holdings RSS feed (available dates)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let dates = client.institutional()
    ///         .get_institutional_holdings_rss().await?;
    ///     
    ///     println!("Available filing dates:");
    ///     for date in dates.iter().take(10) {
    ///         println!("  CIK {}: Q{} {}", date.cik, date.quarter, date.year);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_institutional_holdings_rss(&self) -> Result<Vec<InstitutionalPortfolioDate>> {
        self.client
            .get("/api/v4/institutional-ownership/rss_feed")
            .await
    }

    /// Get fail to deliver (FTD) data for a symbol
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let ftd = client.institutional().get_fail_to_deliver("GME").await?;
    ///     
    ///     println!("Fail to deliver data:");
    ///     for record in ftd.iter().take(10) {
    ///         println!("  {}: {} shares at ${:.2}",
    ///                  record.date, record.quantity, record.price);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_fail_to_deliver(&self, symbol: &str) -> Result<Vec<FailToDeliver>> {
        self.client
            .get(&format!("/api/v4/fail_to_deliver?symbol={}", symbol))
            .await
    }

    /// Get CIK mapper (CIK to company name mapping)
    ///
    /// # Arguments
    ///
    /// * `name` - Optional company name to search
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let mappings = client.institutional().get_cik_mapper(Some("Apple")).await?;
    ///     
    ///     for mapping in &mappings {
    ///         println!("{}: {}", mapping.cik, mapping.name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_cik_mapper(&self, name: Option<&str>) -> Result<Vec<CikMapper>> {
        let url = if let Some(name) = name {
            format!("/api/v3/cik_list?name={}", name)
        } else {
            "/api/v3/cik_list".to_string()
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
        let _ = Institutional::new(client);
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_institutional_holders() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_institutional_holders("AAPL")
            .await;
        assert!(result.is_ok());
        let holders = result.unwrap();
        assert!(!holders.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_portfolio_composition() {
        let client = FmpClient::new().unwrap();
        // Berkshire Hathaway CIK
        let result = client
            .institutional()
            .get_portfolio_composition("0001067983", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_portfolio_holdings() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_portfolio_holdings("AAPL", Some(true))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_institutional_holdings_rss() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_institutional_holdings_rss()
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_fail_to_deliver() {
        let client = FmpClient::new().unwrap();
        let result = client.institutional().get_fail_to_deliver("GME").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cik_mapper_with_name() {
        let client = FmpClient::new().unwrap();
        let result = client.institutional().get_cik_mapper(Some("Apple")).await;
        assert!(result.is_ok());
        let mappings = result.unwrap();
        assert!(!mappings.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cik_mapper_all() {
        let client = FmpClient::new().unwrap();
        let result = client.institutional().get_cik_mapper(None).await;
        assert!(result.is_ok());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_portfolio_composition_with_date() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_portfolio_composition("0001067983", Some("2024-09-30"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_portfolio_holdings_no_current_quarter() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_portfolio_holdings("AAPL", Some(false))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_institutional_holders_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .institutional()
            .get_institutional_holders("INVALID_XYZ123")
            .await;
        // Should either return empty vec or error
        if let Ok(holders) = result {
            assert!(holders.is_empty());
        }
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client
            .institutional()
            .get_institutional_holders("AAPL")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.institutional().get_institutional_holders("").await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_empty_cik() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client
            .institutional()
            .get_portfolio_composition("", None)
            .await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
