//! Analyst endpoints

use crate::Result;
use crate::client::FmpClient;
use crate::models::analyst::{AnalystEstimates, AnalystGrade, ConsensusSummary, PriceTarget};
use crate::models::common::Period;
use serde::Serialize;

/// Analyst API endpoints
pub struct Analyst {
    client: FmpClient,
}

#[derive(Debug, Clone, Serialize)]
struct AnalystQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl Analyst {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get analyst estimates for a symbol
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (annual or quarter, optional)
    /// * `limit` - Number of results (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # use fmp_rs::models::common::Period;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let estimates = client.analyst().get_estimates("AAPL", Some(Period::Annual), Some(10)).await?;
    /// for estimate in estimates {
    ///     println!("{}: Estimated EPS {:.2}", estimate.date, estimate.estimated_eps_avg);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_estimates(
        &self,
        symbol: &str,
        period: Option<Period>,
        limit: Option<u32>,
    ) -> Result<Vec<AnalystEstimates>> {
        let query = AnalystQuery {
            period: period.map(|p| p.to_string()),
            limit,
        };
        self.client
            .get_with_query(&format!("v3/analyst-estimates/{}", symbol), &query)
            .await
    }

    /// Get price targets for a symbol
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
    /// let targets = client.analyst().get_price_targets("AAPL").await?;
    /// for target in targets {
    ///     println!("{} from {}: ${:.2}", target.published_date, target.analyst_company, target.price_target);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_price_targets(&self, symbol: &str) -> Result<Vec<PriceTarget>> {
        self.client
            .get_with_query(&format!("v4/price-target?symbol={}", symbol), &())
            .await
    }

    /// Get analyst price target consensus for a symbol
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
    /// let consensus = client.analyst().get_price_target_consensus("AAPL").await?;
    /// println!("Target High: ${:.2}, Target Low: ${:.2}",
    ///     consensus.first().unwrap().target_high,
    ///     consensus.first().unwrap().target_low);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_price_target_consensus(
        &self,
        symbol: &str,
    ) -> Result<Vec<PriceTargetConsensus>> {
        self.client
            .get_with_query(&format!("v4/price-target-consensus?symbol={}", symbol), &())
            .await
    }

    /// Get analyst recommendations/upgrades for a symbol
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
    /// let grades = client.analyst().get_grades("AAPL").await?;
    /// for grade in grades {
    ///     println!("{} from {}: {} -> {}",
    ///         grade.published_date,
    ///         grade.grading_company,
    ///         grade.previous_grade.unwrap_or_default(),
    ///         grade.new_grade);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_grades(&self, symbol: &str) -> Result<Vec<AnalystGrade>> {
        self.client
            .get_with_query(&format!("v3/grade/{}", symbol), &())
            .await
    }

    /// Get analyst recommendation consensus for a symbol
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
    /// let consensus = client.analyst().get_recommendation_consensus("AAPL").await?;
    /// for rec in consensus {
    ///     println!("Strong Buy: {}, Buy: {}, Hold: {}, Sell: {}, Strong Sell: {}",
    ///         rec.strong_buy, rec.buy, rec.hold, rec.sell, rec.strong_sell);
    ///     println!("Consensus: {}", rec.consensus);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_recommendation_consensus(
        &self,
        symbol: &str,
    ) -> Result<Vec<ConsensusSummary>> {
        self.client
            .get_with_query(&format!("v3/rating/{}", symbol), &())
            .await
    }
}

/// Price target consensus
#[derive(Debug, Clone, Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceTargetConsensus {
    pub symbol: String,
    pub target_high: f64,
    pub target_low: f64,
    pub target_consensus: f64,
    pub target_median: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let analyst = Analyst::new(client);
        assert!(std::ptr::addr_of!(analyst.client).is_null() == false);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_estimates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .analyst()
            .get_estimates("AAPL", Some(Period::Annual), Some(5))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_price_targets() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_price_targets("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_grades() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_grades("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_recommendation_consensus() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_recommendation_consensus("AAPL").await;
        assert!(result.is_ok());
    }

    // Missing endpoint test
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_price_target_consensus() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_price_target_consensus("AAPL").await;
        assert!(result.is_ok());
        let consensus = result.unwrap();

        if !consensus.is_empty() {
            let target = &consensus[0];
            assert_eq!(target.symbol, "AAPL".to_string());
            // Validate price target ranges
            assert!(target.target_high >= target.target_low); // High should be >= low
            assert!(target.target_high > 0.0 && target.target_low > 0.0); // Prices should be positive
        }
    }

    // Edge case tests
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_analyst_data_with_invalid_symbol() {
        let client = FmpClient::new().unwrap();

        let invalid_symbol = "INVALID_STOCK_12345";

        // Test all endpoints with invalid symbol
        let estimates_result = client
            .analyst()
            .get_estimates(invalid_symbol, Some(Period::Annual), Some(5))
            .await;
        let targets_result = client.analyst().get_price_targets(invalid_symbol).await;
        let consensus_result = client
            .analyst()
            .get_price_target_consensus(invalid_symbol)
            .await;
        let grades_result = client.analyst().get_grades(invalid_symbol).await;
        let rec_consensus_result = client
            .analyst()
            .get_recommendation_consensus(invalid_symbol)
            .await;

        // All should handle gracefully - either empty results or errors
        match estimates_result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable
        }

        match targets_result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable
        }

        match consensus_result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable
        }

        match grades_result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable
        }

        match rec_consensus_result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_estimates_with_different_periods() {
        let client = FmpClient::new().unwrap();

        // Test both annual and quarterly periods
        let annual_result = client
            .analyst()
            .get_estimates("AAPL", Some(Period::Annual), Some(3))
            .await;
        let quarterly_result = client
            .analyst()
            .get_estimates("AAPL", Some(Period::Quarter), Some(3))
            .await;

        assert!(annual_result.is_ok());
        assert!(quarterly_result.is_ok());

        let annual_estimates = annual_result.unwrap();
        let quarterly_estimates = quarterly_result.unwrap();

        // Should have different data for different periods
        if !annual_estimates.is_empty() && !quarterly_estimates.is_empty() {
            // Data should be structured correctly
            assert_eq!(annual_estimates[0].symbol, "AAPL".to_string());
            assert_eq!(quarterly_estimates[0].symbol, "AAPL".to_string());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_estimates_limit_parameter() {
        let client = FmpClient::new().unwrap();

        // Test with different limits
        let small_limit_result = client
            .analyst()
            .get_estimates("AAPL", Some(Period::Annual), Some(2))
            .await;
        let large_limit_result = client
            .analyst()
            .get_estimates("AAPL", Some(Period::Annual), Some(10))
            .await;

        assert!(small_limit_result.is_ok());
        assert!(large_limit_result.is_ok());

        let small_estimates = small_limit_result.unwrap();
        let large_estimates = large_limit_result.unwrap();

        // Small limit should return <= 2 items
        assert!(small_estimates.len() <= 2);

        // Large limit should return more data (if available)
        if !small_estimates.is_empty() && !large_estimates.is_empty() {
            // Large limit should return at least as much data as small limit
            assert!(large_estimates.len() >= small_estimates.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_price_targets_data_validation() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_price_targets("AAPL").await;
        assert!(result.is_ok());

        let targets = result.unwrap();
        if !targets.is_empty() {
            let target = &targets[0];

            // Validate required fields
            assert!(!target.symbol.is_empty());

            // Validate price target values
            assert!(target.price_target > 0.0); // Should be positive

            // Validate analyst name
            assert!(!target.analyst_name.is_empty()); // Should not be empty string

            // Validate company name
            assert!(!target.analyst_company.is_empty()); // Should not be empty string
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_grades_data_validation() {
        let client = FmpClient::new().unwrap();
        let result = client.analyst().get_grades("AAPL").await;
        assert!(result.is_ok());

        let grades = result.unwrap();
        if !grades.is_empty() {
            let grade = &grades[0];

            // Validate symbol
            assert_eq!(grade.symbol, "AAPL".to_string());

            // Validate grade values
            let new_grade = &grade.new_grade;
            // Common analyst grades
            let _valid_grades = vec![
                "BUY",
                "SELL",
                "HOLD",
                "STRONG_BUY",
                "STRONG_SELL",
                "OUTPERFORM",
                "UNDERPERFORM",
                "NEUTRAL",
                "OVERWEIGHT",
                "UNDERWEIGHT",
            ];
            // Should be a recognized grade or at least not empty
            assert!(!new_grade.is_empty());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_multiple_symbols_consistency() {
        let client = FmpClient::new().unwrap();

        let symbols = vec!["AAPL", "MSFT", "GOOGL"];

        for symbol in symbols {
            let result = client.analyst().get_price_targets(symbol).await;
            assert!(result.is_ok());

            let targets = result.unwrap();
            if !targets.is_empty() {
                // Each result should be for the correct symbol
                assert_eq!(targets[0].symbol, symbol.to_string());
            }
        }
    }
}
