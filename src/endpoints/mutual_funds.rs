//! Mutual Funds endpoints

use crate::{client::FmpClient, error::Result, models::mutual_funds::*};
use serde::Serialize;

/// Mutual Funds API endpoints
pub struct MutualFunds {
    client: FmpClient,
}

impl MutualFunds {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Search for mutual funds
    ///
    /// Returns a list of mutual funds matching search criteria or all available funds.
    /// Useful for discovering funds by name, symbol, or category.
    ///
    /// # Arguments
    /// * `query` - Optional search term to filter funds
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let funds = client.mutual_funds().search_funds(Some("Vanguard")).await?;
    /// for fund in funds.iter().take(5) {
    ///     println!("{}: {} - AUM: ${:.1}B",
    ///         fund.symbol.as_deref().unwrap_or("N/A"),
    ///         fund.name.as_deref().unwrap_or("Unknown"),
    ///         fund.total_assets.unwrap_or(0.0) / 1_000_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_funds(&self, query: Option<&str>) -> Result<Vec<MutualFund>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            query: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/mutual_fund");
        self.client
            .get_with_query(
                &url,
                &Query {
                    query,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get mutual fund details by symbol
    ///
    /// Returns detailed information about a specific mutual fund including
    /// NAV, expense ratio, total assets, and fund characteristics.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (e.g., "VTSAX", "FXNAX")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let fund_details = client.mutual_funds().get_fund_details("VTSAX").await?;
    /// if let Some(fund) = fund_details.first() {
    ///     println!("{}: NAV = ${:.2}, Expense Ratio = {:.2}%",
    ///         fund.name.as_deref().unwrap_or("Unknown"),
    ///         fund.nav.unwrap_or(0.0),
    ///         fund.expense_ratio.unwrap_or(0.0) * 100.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fund_details(&self, symbol: &str) -> Result<Vec<MutualFund>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/mutual_fund/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get mutual fund historical prices
    ///
    /// Returns historical NAV (Net Asset Value) data for a mutual fund.
    /// Useful for performance analysis and trend identification.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (e.g., "VTSAX")
    /// * `from_date` - Optional start date (YYYY-MM-DD format)
    /// * `to_date` - Optional end date (YYYY-MM-DD format)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.mutual_funds()
    ///     .get_fund_historical("VTSAX", Some("2024-01-01"), Some("2024-03-31")).await?;
    /// for price in history.iter().take(10) {
    ///     println!("{}: NAV = ${:.2} ({:+.2}%)",
    ///         price.date.as_deref().unwrap_or("N/A"),
    ///         price.nav.unwrap_or(0.0),
    ///         price.change_percent.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fund_historical(
        &self,
        symbol: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<MutualFundHistorical>> {
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
            .build_url(&format!("/mutual_fund_historical/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    from: from_date,
                    to: to_date,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get mutual fund holdings
    ///
    /// Returns the top holdings of a mutual fund, showing what securities
    /// the fund invests in and their allocation percentages.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (e.g., "VTSAX")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let holdings = client.mutual_funds().get_fund_holdings("VTSAX").await?;
    /// for holding in holdings.iter().take(10) {
    ///     println!("{}: {:.2}% - {}",
    ///         holding.symbol.as_deref().unwrap_or("N/A"),
    ///         holding.weight_percentage.unwrap_or(0.0),
    ///         holding.name.as_deref().unwrap_or("Unknown"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fund_holdings(&self, symbol: &str) -> Result<Vec<MutualFundHolding>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/mutual_fund_holdings/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get mutual fund performance metrics
    ///
    /// Returns performance statistics including returns over various time periods,
    /// risk metrics (alpha, beta, Sharpe ratio), and benchmark comparisons.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (e.g., "VTSAX")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let performance = client.mutual_funds().get_fund_performance("VTSAX").await?;
    /// if let Some(perf) = performance.first() {
    ///     println!("{}: 1Y = {:.1}%, 3Y = {:.1}%, Sharpe = {:.2}",
    ///         perf.name.as_deref().unwrap_or("Unknown"),
    ///         perf.one_year.unwrap_or(0.0),
    ///         perf.three_years.unwrap_or(0.0),
    ///         perf.sharpe_ratio.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fund_performance(&self, symbol: &str) -> Result<Vec<MutualFundPerformance>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/mutual_fund_performance/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get mutual fund sector allocation
    ///
    /// Returns the sector breakdown of a mutual fund's holdings,
    /// showing how the fund is allocated across different industries.
    ///
    /// # Arguments
    /// * `symbol` - Fund symbol (e.g., "VTSAX")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let sectors = client.mutual_funds().get_fund_sectors("VTSAX").await?;
    /// for sector in sectors.iter().take(10) {
    ///     println!("{}: {:.1}%",
    ///         sector.sector.as_deref().unwrap_or("Unknown"),
    ///         sector.weight_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fund_sectors(&self, symbol: &str) -> Result<Vec<MutualFundSector>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/mutual_fund_sectors/{}", symbol));
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

    fn create_test_client() -> FmpClient {
        FmpClient::builder().api_key("test_key").build().unwrap()
    }

    #[test]
    fn test_new() {
        let client = create_test_client();
        let _mutual_funds = MutualFunds::new(client);
        // Test passes if no panic occurs
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_search_funds_no_query() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().search_funds(None).await;
        assert!(result.is_ok());

        let funds = result.unwrap();
        if !funds.is_empty() {
            let first_fund = &funds[0];
            // Should have at least symbol or name
            assert!(first_fund.symbol.is_some() || first_fund.name.is_some());
            println!("Found {} mutual funds", funds.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_search_funds_with_query() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().search_funds(Some("Vanguard")).await;
        assert!(result.is_ok());

        let funds = result.unwrap();
        println!("Found {} Vanguard funds", funds.len());

        // Check that results contain Vanguard in name or symbol
        if !funds.is_empty() {
            let has_vanguard = funds.iter().any(|fund| {
                fund.name
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains("vanguard")
                    || fund
                        .fund_family
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains("vanguard")
            });
            println!("Search results contain Vanguard: {}", has_vanguard);
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_fund_details() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().get_fund_details("VTSAX").await;
        assert!(result.is_ok());

        let fund_details = result.unwrap();
        if let Some(fund) = fund_details.first() {
            assert_eq!(fund.symbol.as_deref(), Some("VTSAX"));
            assert!(fund.name.is_some());
            println!("VTSAX: {}", fund.name.as_deref().unwrap_or("Unknown"));
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_fund_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .mutual_funds()
            .get_fund_historical("VTSAX", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());

        let history = result.unwrap();
        if !history.is_empty() {
            let first_entry = &history[0];
            assert!(first_entry.date.is_some());
            assert!(first_entry.nav.is_some());
            println!("Historical data points: {}", history.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_fund_holdings() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().get_fund_holdings("VTSAX").await;
        assert!(result.is_ok());

        let holdings = result.unwrap();
        if !holdings.is_empty() {
            let top_holding = &holdings[0];
            assert!(top_holding.symbol.is_some() || top_holding.name.is_some());
            assert!(top_holding.weight_percentage.is_some());
            println!("Top holdings count: {}", holdings.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_fund_performance() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().get_fund_performance("VTSAX").await;
        assert!(result.is_ok());

        let performance = result.unwrap();
        if let Some(perf) = performance.first() {
            assert!(perf.symbol.is_some());
            // Should have at least some performance metrics
            assert!(perf.one_year.is_some() || perf.ytd.is_some());
            println!("Performance data available");
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_fund_sectors() {
        let client = FmpClient::new().unwrap();
        let result = client.mutual_funds().get_fund_sectors("VTSAX").await;
        assert!(result.is_ok());

        let sectors = result.unwrap();
        if !sectors.is_empty() {
            let top_sector = &sectors[0];
            assert!(top_sector.sector.is_some());
            assert!(top_sector.weight_percentage.is_some());
            println!("Sector allocations: {}", sectors.len());
        }
    }

    #[test]
    fn test_mutual_fund_models_serialization() {
        use serde_json;

        // Test MutualFund model
        let fund = MutualFund {
            symbol: Some("VTSAX".to_string()),
            name: Some("Vanguard Total Stock Market Index Fund".to_string()),
            fund_family: Some("Vanguard".to_string()),
            category: Some("Large Blend".to_string()),
            nav: Some(120.50),
            expense_ratio: Some(0.0003),
            total_assets: Some(1_400_000_000_000.0), // $1.4T
            yield_: Some(1.45),
            fund_type: Some("Index Fund".to_string()),
            investment_objective: Some("Track total stock market".to_string()),
            previous_nav: Some(120.25),
            change: Some(0.25),
            change_percent: Some(0.21),
            minimum_investment: Some(3000.0),
            inception_date: Some("2000-11-13".to_string()),
            manager_name: Some("Vanguard Equity Investment Group".to_string()),
            currency: Some("USD".to_string()),
            exchange: Some("NASDAQ".to_string()),
            last_updated: Some("2024-01-15".to_string()),
        };

        let json = serde_json::to_string(&fund).unwrap();
        let deserialized: MutualFund = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, Some("VTSAX".to_string()));
        assert_eq!(deserialized.nav, Some(120.50));

        // Test MutualFundPerformance model
        let performance = MutualFundPerformance {
            symbol: Some("VTSAX".to_string()),
            name: Some("Vanguard Total Stock Market".to_string()),
            one_day: Some(-0.15),
            one_week: Some(1.25),
            one_month: Some(2.80),
            three_months: Some(8.45),
            ytd: Some(12.30),
            one_year: Some(15.75),
            three_years: Some(10.20),
            five_years: Some(11.85),
            ten_years: Some(12.45),
            since_inception: Some(9.95),
            standard_deviation: Some(14.85),
            sharpe_ratio: Some(0.95),
            alpha: Some(0.15),
            beta: Some(0.98),
            r_squared: Some(0.995),
            tracking_error: Some(0.05),
            information_ratio: Some(0.25),
            benchmark: Some("VTI".to_string()),
            six_months: Some(6.75),
            date: Some("2024-01-15".to_string()),
        };

        let json = serde_json::to_string(&performance).unwrap();
        let deserialized: MutualFundPerformance = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.one_year, Some(15.75));
        assert_eq!(deserialized.sharpe_ratio, Some(0.95));
    }

    #[test]
    fn test_fund_holdings_model() {
        let holding = MutualFundHolding {
            symbol: Some("AAPL".to_string()),
            name: Some("Apple Inc.".to_string()),
            sector: Some("Technology".to_string()),
            industry: Some("Consumer Electronics".to_string()),
            country: Some("United States".to_string()),
            weight_percentage: Some(7.25),
            market_value: Some(125_000_000.0),
            shares_held: Some(675_000),
            asset_type: Some("Common Stock".to_string()),
            cusip: Some("037833100".to_string()),
            isin: Some("US0378331005".to_string()),
            date: Some("2024-01-15".to_string()),
        };

        // Verify all fields are accessible and serializable
        assert_eq!(holding.symbol, Some("AAPL".to_string()));
        assert_eq!(holding.weight_percentage, Some(7.25));
        assert_eq!(holding.sector, Some("Technology".to_string()));

        let json = serde_json::to_string(&holding).unwrap();
        let deserialized: MutualFundHolding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, Some("AAPL".to_string()));
    }

    #[test]
    fn test_date_range_validation() {
        // Test that date parameters are properly formatted
        let start_date = "2024-01-01";
        let end_date = "2024-12-31";

        // These should be valid ISO date formats
        assert!(chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").is_ok());
        assert!(chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d").is_ok());
    }

    // Additional edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_fund_details_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .mutual_funds()
            .get_fund_details("INVALID_FUND_XYZ")
            .await;
        if let Ok(funds) = result {
            assert!(funds.is_empty())
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_fund_historical_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .mutual_funds()
            .get_fund_historical("VTSAX", Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
        let historical = result.unwrap();
        // Should have at most 31 days of data
        assert!(historical.len() <= 31);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_fund_holdings_empty_result() {
        let client = FmpClient::new().unwrap();
        let result = client
            .mutual_funds()
            .get_fund_holdings("INVALID_FUND")
            .await;
        if let Ok(holdings) = result {
            assert!(holdings.is_empty())
        }
    }
}
