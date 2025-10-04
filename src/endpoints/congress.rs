//! Congressional and insider trading endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::congress::{
        CongressionalSummary, CongressionalTrade, InsiderOwnership, InsiderRoster, InsiderStats,
        InsiderTrade,
    },
};
use serde::Serialize;

/// Congressional and Insider Trading API endpoints
pub struct Congress {
    client: FmpClient,
}

impl Congress {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get congressional trading data (House and Senate)
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional, if None returns all congressional trades)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    ///
    /// // Get all congressional trades
    /// let all_trades = congress.get_congressional_trades(None).await?;
    ///
    /// // Get congressional trades for a specific stock
    /// let aapl_trades = congress.get_congressional_trades(Some("AAPL")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_congressional_trades(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<CongressionalTrade>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<&'a str>,
        }

        let url = self.client.build_url("/v4/senate-trading");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                    symbol,
                },
            )
            .await
    }

    /// Get House of Representatives trading data
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    /// let house_trades = congress.get_house_trades(Some("TSLA")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_house_trades(&self, symbol: Option<&str>) -> Result<Vec<CongressionalTrade>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<&'a str>,
        }

        let url = self.client.build_url("/v4/senate-trading-rss");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                    symbol,
                },
            )
            .await
    }

    /// Get insider trading transactions for a company
    ///
    /// # Arguments
    /// * `symbol` - Company stock symbol
    /// * `page` - Page number (optional, defaults to 0)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    /// let insider_trades = congress.get_insider_trades("AAPL", Some(0)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_trades(
        &self,
        symbol: &str,
        page: Option<i32>,
    ) -> Result<Vec<InsiderTrade>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }

        let url = self.client.build_url("/v4/insider-trading");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                    page,
                },
            )
            .await
    }

    /// Get insider trading statistics for a company
    ///
    /// # Arguments
    /// * `symbol` - Company stock symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    /// let insider_stats = congress.get_insider_stats("MSFT").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_stats(&self, symbol: &str) -> Result<Vec<InsiderStats>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url("/v4/insider-trading-transaction-type");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get insider roster (list of insiders for a company)
    ///
    /// # Arguments
    /// * `symbol` - Company stock symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    /// let insiders = congress.get_insider_roster("GOOGL").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_roster(&self, symbol: &str) -> Result<Vec<InsiderRoster>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/mapper-cik-company");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get congressional trading summary by representative
    ///
    /// # Arguments
    /// * `representative` - Name of representative (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    ///
    /// // Get all congressional summaries
    /// let all_summaries = congress.get_congressional_summary(None).await?;
    ///
    /// // Get specific representative
    /// let pelosi_summary = congress.get_congressional_summary(Some("Nancy Pelosi")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_congressional_summary(
        &self,
        representative: Option<&str>,
    ) -> Result<Vec<CongressionalSummary>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            representative: Option<&'a str>,
        }

        let url = self.client.build_url("/v4/senate-disclosure");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                    representative,
                },
            )
            .await
    }

    /// Get insider ownership summary for a company
    ///
    /// # Arguments
    /// * `symbol` - Company stock symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let congress = client.congress();
    /// let ownership = congress.get_insider_ownership("NVDA").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_ownership(&self, symbol: &str) -> Result<Vec<InsiderOwnership>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/insider-trading-statistics");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_client() -> FmpClient {
        FmpClient::builder().api_key("test_key").build().unwrap()
    }

    fn get_test_congress() -> Congress {
        Congress::new(create_test_client())
    }

    #[test]
    fn test_new() {
        let client = create_test_client();
        let _congress = Congress::new(client);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_congressional_trades() {
        let congress = get_test_congress();
        let result = congress.get_congressional_trades(None).await;
        assert!(result.is_ok());

        let trades = result.unwrap();
        if !trades.is_empty() {
            let trade = &trades[0];
            assert!(trade.representative.is_some() || trade.ticker.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_congressional_trades_by_symbol() {
        let congress = get_test_congress();
        let result = congress.get_congressional_trades(Some("AAPL")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_house_trades() {
        let congress = get_test_congress();
        let result = congress.get_house_trades(Some("TSLA")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_trades() {
        let congress = get_test_congress();
        let result = congress.get_insider_trades("AAPL", Some(0)).await;
        assert!(result.is_ok());

        let trades = result.unwrap();
        if !trades.is_empty() {
            let trade = &trades[0];
            assert_eq!(trade.symbol.as_ref().unwrap(), "AAPL");
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_trades_pagination() {
        let congress = get_test_congress();
        let page0 = congress.get_insider_trades("MSFT", Some(0)).await;
        let page1 = congress.get_insider_trades("MSFT", Some(1)).await;

        assert!(page0.is_ok());
        assert!(page1.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_stats() {
        let congress = get_test_congress();
        let result = congress.get_insider_stats("GOOGL").await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        if !stats.is_empty() {
            let stat = &stats[0];
            assert!(stat.symbol.is_some() || stat.company_name.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_roster() {
        let congress = get_test_congress();
        let result = congress.get_insider_roster("NVDA").await;
        assert!(result.is_ok());

        let roster = result.unwrap();
        if !roster.is_empty() {
            let insider = &roster[0];
            assert!(insider.name.is_some() || insider.title.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_congressional_summary() {
        let congress = get_test_congress();
        let result = congress.get_congressional_summary(None).await;
        assert!(result.is_ok());

        let summaries = result.unwrap();
        if !summaries.is_empty() {
            let summary = &summaries[0];
            assert!(summary.representative.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_congressional_summary_specific() {
        let congress = get_test_congress();
        let result = congress
            .get_congressional_summary(Some("Nancy Pelosi"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_ownership() {
        let congress = get_test_congress();
        let result = congress.get_insider_ownership("AMZN").await;
        assert!(result.is_ok());

        let ownership = result.unwrap();
        if !ownership.is_empty() {
            let data = &ownership[0];
            assert!(data.symbol.is_some() || data.company_name.is_some());
        }
    }

    // Model serialization tests
    #[test]
    fn test_congressional_trade_serialization() {
        let trade = CongressionalTrade {
            representative: Some("John Doe".to_string()),
            transaction_date: Some("2024-01-15".to_string()),
            ticker: Some("AAPL".to_string()),
            company: Some("Apple Inc".to_string()),
            transaction: Some("Purchase".to_string()),
            amount: Some("$1,001 - $15,000".to_string()),
            party: Some("Democrat".to_string()),
            state: Some("CA".to_string()),
            chamber: Some("House".to_string()),
            asset_type: Some("Stock".to_string()),
            filing_date: Some("2024-01-30".to_string()),
            disclosure_period: Some("2024-Q1".to_string()),
            transaction_id: Some("T123456".to_string()),
            owner: Some("Self".to_string()),
        };

        let json = serde_json::to_string(&trade).unwrap();
        let deserialized: CongressionalTrade = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.representative, trade.representative);
        assert_eq!(deserialized.ticker, trade.ticker);
    }

    #[test]
    fn test_insider_trade_serialization() {
        let trade = InsiderTrade {
            symbol: Some("MSFT".to_string()),
            company_name: Some("Microsoft Corporation".to_string()),
            filing_date: Some("2024-01-15".to_string()),
            transaction_date: Some("2024-01-10".to_string()),
            reporting_name: Some("Satya Nadella".to_string()),
            title: Some("Chief Executive Officer".to_string()),
            transaction_type: Some("P".to_string()),
            securities_transacted: Some(1000.0),
            price: Some(375.50),
            transaction_value: Some(375500.0),
            securities_owned: Some(250000.0),
            form_type: Some("4".to_string()),
            cik: Some("0000789019".to_string()),
            accession_number: Some("0001234567890123456789".to_string()),
            is_director: Some(true),
            is_officer: Some(true),
            is_ten_percent_owner: Some(false),
            ownership_type: Some("D".to_string()),
        };

        let json = serde_json::to_string(&trade).unwrap();
        let deserialized: InsiderTrade = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, trade.symbol);
        assert_eq!(deserialized.reporting_name, trade.reporting_name);
    }

    // Additional edge case tests
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_stats() {
        let congress = get_test_congress();
        let result = congress.get_insider_stats("AAPL").await;
        assert!(result.is_ok());
        let stats = result.unwrap();
        if !stats.is_empty() {
            assert_eq!(stats[0].symbol.as_ref().unwrap(), "AAPL");
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_roster() {
        let congress = get_test_congress();
        let result = congress.get_insider_roster("AAPL").await;
        assert!(result.is_ok());
        let roster = result.unwrap();
        if !roster.is_empty() {
            assert!(roster[0].name.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_congressional_summary() {
        let congress = get_test_congress();
        let result = congress.get_congressional_summary(Some("TSLA")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_insider_ownership() {
        let congress = get_test_congress();
        let result = congress.get_insider_ownership("AAPL").await;
        assert!(result.is_ok());
        let ownership = result.unwrap();
        if !ownership.is_empty() {
            assert!(ownership[0].reporting_name.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_invalid_symbol_edge_cases() {
        let congress = get_test_congress();

        // Test with invalid symbols
        let result1 = congress.get_congressional_trades(Some("INVALID_XYZ")).await;
        let result2 = congress.get_insider_trades("INVALID_XYZ", Some(1)).await;
        let result3 = congress.get_insider_stats("INVALID_XYZ").await;

        // All should handle gracefully - either empty results or errors
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        if let Ok(trades) = result1 {
            assert!(trades.is_empty());
        }
        if let Ok(insider_trades) = result2 {
            assert!(insider_trades.is_empty());
        }
        if let Ok(stats) = result3 {
            assert!(stats.is_empty());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_pagination_edge_cases() {
        let congress = get_test_congress();

        // Test with large page number that should return empty results
        let result = congress.get_insider_trades("AAPL", Some(99999)).await;
        assert!(result.is_ok());
        let trades = result.unwrap();
        assert!(trades.is_empty() || trades.len() <= 100); // Should not exceed reasonable limits
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_congressional_trades_no_symbol() {
        let congress = get_test_congress();
        let result = congress.get_congressional_trades(None).await;
        assert!(result.is_ok());
        // Should return general congressional trades
        let trades = result.unwrap();
        if !trades.is_empty() {
            assert!(trades[0].representative.is_some() || trades[0].ticker.is_some());
        }
    }
}
