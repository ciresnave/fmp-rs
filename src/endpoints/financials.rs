//! Financial statements endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::common::Period;
use crate::models::financials::{
    BalanceSheet, CashFlowStatement, FinancialAsReported, FinancialGrowth, FinancialRatios,
    IncomeStatement, KeyMetrics, RevenueGeographicSegmentation, RevenueProductSegmentation,
};
use serde::Serialize;

/// Financial statements API endpoints
pub struct Financials {
    client: FmpClient,
}

impl Financials {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get income statements
    pub async fn get_income_statement(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<IncomeStatement>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/income-statement");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get balance sheet
    pub async fn get_balance_sheet(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<BalanceSheet>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/balance-sheet-statement");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get cash flow statement
    pub async fn get_cash_flow_statement(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<CashFlowStatement>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/cash-flow-statement");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get financial ratios
    pub async fn get_ratios(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<FinancialRatios>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/ratios");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get key metrics
    pub async fn get_key_metrics(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<KeyMetrics>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/key-metrics");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get key metrics TTM (Trailing Twelve Months)
    pub async fn get_key_metrics_ttm(&self, symbol: &str) -> Result<Vec<KeyMetrics>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/key-metrics-ttm");
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

    /// Get financial ratios TTM (Trailing Twelve Months)
    pub async fn get_ratios_ttm(&self, symbol: &str) -> Result<Vec<FinancialRatios>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/ratios-ttm");
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

    /// Get financial growth metrics
    ///
    /// Returns year-over-year and multi-year growth rates for key financial metrics.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (Annual or Quarter)
    /// * `limit` - Number of results (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, models::common::Period};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let growth = client.financials().get_financial_growth("AAPL", Period::Annual, Some(5)).await?;
    /// for g in growth {
    ///     println!("{}: Revenue growth: {:.2}%, Net income growth: {:.2}%",
    ///         g.date, g.revenue_growth * 100.0, g.net_income_growth * 100.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_financial_growth(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<FinancialGrowth>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/financial-growth");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get financial statement as reported (XBRL data)
    ///
    /// Returns financial statements as reported to the SEC with XBRL tags.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (Annual or Quarter)
    /// * `limit` - Number of results (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, models::common::Period};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let reported = client.financials().get_financial_as_reported("AAPL", Period::Annual, Some(1)).await?;
    /// for statement in reported {
    ///     println!("{}: {} fields reported", statement.date, statement.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_financial_as_reported(
        &self,
        symbol: &str,
        period: Period,
        limit: Option<u32>,
    ) -> Result<Vec<FinancialAsReported>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url("/financial-statement-full-as-reported");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get revenue product segmentation
    ///
    /// Returns revenue breakdown by product or service line.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (Annual or Quarter)
    /// * `structure` - "flat" for simple structure (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, models::common::Period};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let segments = client.financials().get_revenue_product_segmentation("AAPL", Period::Annual, None).await?;
    /// for seg in segments {
    ///     println!("{}: {} product segments", seg.date, seg.segments.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_revenue_product_segmentation(
        &self,
        symbol: &str,
        period: Period,
        structure: Option<&str>,
    ) -> Result<Vec<RevenueProductSegmentation>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            structure: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/revenue-product-segmentation");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    structure,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get revenue geographic segmentation
    ///
    /// Returns revenue breakdown by geographic region.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (Annual or Quarter)
    /// * `structure` - "flat" for simple structure (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, models::common::Period};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let segments = client.financials().get_revenue_geographic_segmentation("AAPL", Period::Annual, None).await?;
    /// for seg in segments {
    ///     println!("{}: {} geographic segments", seg.date, seg.segments.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_revenue_geographic_segmentation(
        &self,
        symbol: &str,
        period: Period,
        structure: Option<&str>,
    ) -> Result<Vec<RevenueGeographicSegmentation>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            structure: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/revenue-geographic-segmentation");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    structure,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get full financial statement as reported (comprehensive XBRL data)
    ///
    /// Returns the complete set of as-reported financial data from SEC filings.
    /// This is more comprehensive than `get_financial_as_reported()`.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period (Annual or Quarter)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::{FmpClient, models::common::Period};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let statements = client.financials().get_financial_full_as_reported("AAPL", Period::Annual).await?;
    /// for stmt in statements.iter().take(1) {
    ///     println!("Date: {}, Period: {}", stmt.date, stmt.period);
    ///     println!("XBRL fields: {}", stmt.data.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_financial_full_as_reported(
        &self,
        symbol: &str,
        period: Period,
    ) -> Result<Vec<FinancialAsReported>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            period: &'a str,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url("/financial-statement-full-as-reported");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    period: &period.to_string(),
                    apikey: self.client.api_key(),
                },
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
        let _ = Financials::new(client);
    }

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_income_statement() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_income_statement("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
        assert!(statements.len() <= 5);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_balance_sheet() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_balance_sheet("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cash_flow_statement() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_cash_flow_statement("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_ratios() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_ratios("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let ratios = result.unwrap();
        assert!(!ratios.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_key_metrics() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_key_metrics("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(!metrics.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_key_metrics_ttm() {
        let client = FmpClient::new().unwrap();
        let result = client.financials().get_key_metrics_ttm("AAPL").await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(!metrics.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_ratios_ttm() {
        let client = FmpClient::new().unwrap();
        let result = client.financials().get_ratios_ttm("AAPL").await;
        assert!(result.is_ok());
        let ratios = result.unwrap();
        assert!(!ratios.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_growth() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_growth("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_ok());
        let growth = result.unwrap();
        assert!(!growth.is_empty());
        assert!(growth.len() <= 5);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_as_reported() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_as_reported("AAPL", Period::Annual, Some(1))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_revenue_product_segmentation() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_revenue_product_segmentation("AAPL", Period::Annual, None)
            .await;
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(!segments.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_revenue_geographic_segmentation() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_revenue_geographic_segmentation("AAPL", Period::Annual, None)
            .await;
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(!segments.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_full_as_reported() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_full_as_reported("AAPL", Period::Annual)
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
        // Full as reported should have comprehensive XBRL data
        assert!(!statements[0].data.is_empty());
    }

    // Test quarterly period
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_income_statement_quarterly() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_income_statement("AAPL", Period::Quarter, Some(4))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
        assert!(statements.len() <= 4);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_growth_quarterly() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_growth("AAPL", Period::Quarter, Some(4))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_full_as_reported_quarterly() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_full_as_reported("AAPL", Period::Quarter)
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_income_statement_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_income_statement("INVALID_SYMBOL_XYZ123", Period::Annual, Some(5))
            .await;
        // Should either return empty vec or error
        if let Ok(statements) = result {
            assert!(statements.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_growth_zero_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_growth("AAPL", Period::Annual, Some(0))
            .await;
        // Should handle gracefully
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_revenue_segmentation_with_structure() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_revenue_product_segmentation("AAPL", Period::Annual, Some("flat"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_revenue_geographic_with_structure() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_revenue_geographic_segmentation("AAPL", Period::Annual, Some("flat"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_balance_sheet_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_balance_sheet("AAPL", Period::Annual, None)
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cash_flow_quarterly() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_cash_flow_statement("AAPL", Period::Quarter, Some(8))
            .await;
        assert!(result.is_ok());
        let statements = result.unwrap();
        assert!(!statements.is_empty());
        assert!(statements.len() <= 8);
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client
            .financials()
            .get_income_statement("AAPL", Period::Annual, Some(5))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client
            .financials()
            .get_income_statement("", Period::Annual, Some(5))
            .await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_company_without_segmentation() {
        let client = FmpClient::new().unwrap();
        // Some companies may not have segmentation data
        let result = client
            .financials()
            .get_revenue_product_segmentation("TSLA", Period::Annual, None)
            .await;
        // Should succeed but may be empty
        if let Ok(_segments) = result {
            // No assertion on emptiness as it depends on company
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_financial_full_as_reported_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .financials()
            .get_financial_full_as_reported("INVALID_SYMBOL_XYZ123", Period::Annual)
            .await;
        // Should succeed but return empty for invalid symbol
        assert!(result.is_ok());
        if let Ok(statements) = result {
            assert!(statements.is_empty());
        }
    }
}
