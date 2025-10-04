//! ETF endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::etf::{
    CountryWeighting, EtfHolder, EtfHolding, EtfInfo, EtfListItem, EtfSearchResult, SectorWeighting,
};

/// ETF API endpoints
pub struct Etf {
    client: FmpClient,
}

impl Etf {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get a list of all available ETFs
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let etfs = client.etf().get_etf_list().await?;
    ///     
    ///     for etf in etfs.iter().take(5) {
    ///         println!("{}: {}", etf.symbol, etf.name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_list(&self) -> Result<Vec<EtfListItem>> {
        self.client.get("/api/v3/etf/list").await
    }

    /// Search for ETFs by name or symbol
    ///
    /// # Arguments
    ///
    /// * `query` - Search query (name or symbol fragment)
    /// * `limit` - Optional limit on number of results
    /// * `exchange` - Optional exchange filter (e.g., "NASDAQ", "NYSE")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let results = client.etf().search_etf("vanguard", Some(10), None).await?;
    ///     
    ///     for etf in &results {
    ///         println!("{}: {} ({})", etf.symbol, etf.name,
    ///                  etf.exchange_short_name.as_deref().unwrap_or("N/A"));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn search_etf(
        &self,
        query: &str,
        limit: Option<u32>,
        exchange: Option<&str>,
    ) -> Result<Vec<EtfSearchResult>> {
        let mut url = format!("/api/v3/search/etf?query={}", query);
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }
        if let Some(exchange) = exchange {
            url.push_str(&format!("&exchange={}", exchange));
        }
        self.client.get(&url).await
    }

    /// Get institutional holders of an ETF (who holds this ETF)
    ///
    /// # Arguments
    ///
    /// * `symbol` - ETF symbol (e.g., "SPY")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let holders = client.etf().get_etf_holder("SPY").await?;
    ///     
    ///     println!("Top holders of SPY:");
    ///     for holder in holders.iter().take(10) {
    ///         println!("  {}: {}%", holder.name,
    ///                  holder.weight_percentage.unwrap_or(0.0));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_holder(&self, symbol: &str) -> Result<Vec<EtfHolder>> {
        self.client
            .get(&format!("/api/v3/etf-holder/{}", symbol))
            .await
    }

    /// Get holdings of an ETF (what this ETF holds)
    ///
    /// # Arguments
    ///
    /// * `symbol` - ETF symbol (e.g., "SPY")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let holdings = client.etf().get_etf_holdings("SPY").await?;
    ///     
    ///     println!("Top holdings in SPY:");
    ///     for holding in holdings.iter().take(10) {
    ///         println!("  {}: {:.2}% ({})",
    ///                  holding.asset, holding.weight_percentage, holding.name);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_holdings(&self, symbol: &str) -> Result<Vec<EtfHolding>> {
        self.client
            .get(&format!("/api/v3/etf-holdings/{}", symbol))
            .await
    }

    /// Get sector weighting of an ETF
    ///
    /// # Arguments
    ///
    /// * `symbol` - ETF symbol (e.g., "SPY")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let sectors = client.etf().get_etf_sector_weighting("SPY").await?;
    ///     
    ///     println!("Sector allocation for SPY:");
    ///     for sector in &sectors {
    ///         println!("  {}: {}%", sector.sector, sector.weight_percentage);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_sector_weighting(&self, symbol: &str) -> Result<Vec<SectorWeighting>> {
        self.client
            .get(&format!("/api/v3/etf-sector-weightings/{}", symbol))
            .await
    }

    /// Get country weighting of an ETF
    ///
    /// # Arguments
    ///
    /// * `symbol` - ETF symbol (e.g., "SPY")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let countries = client.etf().get_etf_country_weighting("SPY").await?;
    ///     
    ///     println!("Country allocation for SPY:");
    ///     for country in &countries {
    ///         println!("  {}: {}%", country.country, country.weight_percentage);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_country_weighting(&self, symbol: &str) -> Result<Vec<CountryWeighting>> {
        self.client
            .get(&format!("/api/v3/etf-country-weightings/{}", symbol))
            .await
    }

    /// Get detailed information about an ETF
    ///
    /// # Arguments
    ///
    /// * `symbol` - ETF symbol (e.g., "SPY")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = FmpClient::new()?;
    ///     let info = client.etf().get_etf_info("SPY").await?;
    ///     
    ///     if let Some(etf) = info.first() {
    ///         println!("ETF: {} ({})", etf.company_name, etf.symbol);
    ///         println!("AUM: ${:.2}B", etf.aum / 1_000_000_000.0);
    ///         println!("Expense Ratio: {:.2}%", etf.expense_ratio);
    ///         println!("Holdings: {}", etf.holdings_count);
    ///         println!("Inception: {}", etf.inception_date);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_etf_info(&self, symbol: &str) -> Result<Vec<EtfInfo>> {
        self.client
            .get(&format!("/api/v4/etf-info?symbol={}", symbol))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let _ = Etf::new(client);
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_list() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_list().await;
        assert!(result.is_ok());
        let etfs = result.unwrap();
        assert!(!etfs.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_etf() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().search_etf("vanguard", Some(10), None).await;
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(!results.is_empty());
        assert!(results.len() <= 10);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_holder() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_holder("SPY").await;
        assert!(result.is_ok());
        let holders = result.unwrap();
        assert!(!holders.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_holdings() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_holdings("SPY").await;
        assert!(result.is_ok());
        let holdings = result.unwrap();
        assert!(!holdings.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_sector_weighting() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_sector_weighting("SPY").await;
        assert!(result.is_ok());
        let sectors = result.unwrap();
        assert!(!sectors.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_country_weighting() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_country_weighting("SPY").await;
        assert!(result.is_ok());
        let countries = result.unwrap();
        assert!(!countries.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_info() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_info("SPY").await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_etf_with_exchange() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().search_etf("sp", Some(5), Some("NYSE")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_etf_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().search_etf("tech", None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_holder_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_holder("INVALID_ETF_XYZ123").await;
        // Should either return empty vec or error
        if let Ok(holders) = result {
            assert!(holders.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_holdings_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().get_etf_holdings("INVALID_ETF_XYZ123").await;
        // Should either return empty vec or error
        if let Ok(holdings) = result {
            assert!(holdings.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_etf_info_multiple_etfs() {
        let client = FmpClient::new().unwrap();
        // Test with a well-known ETF
        let result = client.etf().get_etf_info("QQQ").await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.is_empty());
        if let Some(etf) = info.first() {
            assert_eq!(etf.symbol, "QQQ");
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_etf_special_characters() {
        let client = FmpClient::new().unwrap();
        let result = client.etf().search_etf("S&P", Some(5), None).await;
        assert!(result.is_ok());
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.etf().get_etf_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.etf().get_etf_holder("").await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_empty_search_query() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.etf().search_etf("", Some(10), None).await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
