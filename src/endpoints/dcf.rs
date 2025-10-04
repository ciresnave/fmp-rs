//! DCF (Discounted Cash Flow) and valuation endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::valuation::{AdvancedDcf, Dcf as DcfModel, EnterpriseValue, HistoricalDcf, LeveredDcf},
};
use serde::Serialize;

/// DCF and Valuation API endpoints
pub struct Dcf {
    client: FmpClient,
}

impl Dcf {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get current DCF valuation for a symbol
    ///
    /// Returns the discounted cash flow (DCF) valuation which estimates
    /// the intrinsic value of a stock based on its future cash flows.
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
    /// let dcf = client.dcf().get_dcf("AAPL").await?;
    /// for valuation in dcf {
    ///     println!("Symbol: {}, DCF: ${:.2}, Stock Price: ${:.2}",
    ///         valuation.symbol,
    ///         valuation.dcf.unwrap_or(0.0),
    ///         valuation.stock_price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dcf(&self, symbol: &str) -> Result<Vec<DcfModel>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/discounted-cash-flow/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical DCF valuations for a symbol
    ///
    /// Returns DCF valuations over time, allowing comparison of
    /// historical intrinsic values vs. actual stock prices.
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
    /// let history = client.dcf().get_historical_dcf("AAPL").await?;
    /// for record in history.iter().take(5) {
    ///     println!("{}: DCF ${:.2} vs Stock ${:.2}",
    ///         record.date,
    ///         record.dcf.unwrap_or(0.0),
    ///         record.stock_price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_dcf(&self, symbol: &str) -> Result<Vec<HistoricalDcf>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!(
            "/historical-discounted-cash-flow-statement/{}",
            symbol
        ));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get advanced DCF with detailed assumptions
    ///
    /// Returns DCF valuation with detailed modeling assumptions including
    /// revenue growth, margins, tax rates, WACC, and projected cash flows.
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
    /// let dcf = client.dcf().get_advanced_dcf("AAPL").await?;
    /// for valuation in dcf {
    ///     println!("DCF: ${:.2}", valuation.dcf.unwrap_or(0.0));
    ///     println!("Revenue Growth: {:.2}%", valuation.revenue_growth.unwrap_or(0.0) * 100.0);
    ///     println!("WACC: {:.2}%", valuation.wacc.unwrap_or(0.0) * 100.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_advanced_dcf(&self, symbol: &str) -> Result<Vec<AdvancedDcf>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/advanced_discounted_cash_flow/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get levered DCF (equity value after debt)
    ///
    /// Returns DCF valuation accounting for debt, providing equity value
    /// rather than enterprise value.
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
    /// let dcf = client.dcf().get_levered_dcf("AAPL").await?;
    /// for valuation in dcf {
    ///     println!("Levered DCF: ${:.2}", valuation.levered_dcf.unwrap_or(0.0));
    ///     println!("Enterprise Value: ${:.2}B", valuation.enterprise_value.unwrap_or(0.0) / 1_000_000_000.0);
    ///     println!("Total Debt: ${:.2}B", valuation.total_debt.unwrap_or(0.0) / 1_000_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_levered_dcf(&self, symbol: &str) -> Result<Vec<LeveredDcf>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/levered_discounted_cash_flow/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company enterprise value
    ///
    /// Returns the enterprise value calculation, which represents the total
    /// value of a company (market cap + debt - cash).
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
    /// let ev = client.dcf().get_enterprise_value("AAPL").await?;
    /// for value in ev {
    ///     println!("Market Cap: ${:.2}B", value.market_capitalization.unwrap_or(0.0) / 1_000_000_000.0);
    ///     println!("+ Debt: ${:.2}B", value.add_total_debt.unwrap_or(0.0) / 1_000_000_000.0);
    ///     println!("- Cash: ${:.2}B", value.minus_cash_and_cash_equivalents.unwrap_or(0.0) / 1_000_000_000.0);
    ///     println!("= Enterprise Value: ${:.2}B", value.enterprise_value.unwrap_or(0.0) / 1_000_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_enterprise_value(&self, symbol: &str) -> Result<Vec<EnterpriseValue>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/enterprise-values/{}", symbol));
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
    async fn test_get_dcf() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_dcf("AAPL").await;
        assert!(result.is_ok());
        let dcf = result.unwrap();
        assert!(!dcf.is_empty());
        assert_eq!(dcf[0].symbol, "AAPL");
        assert!(dcf[0].dcf.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_dcf() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_historical_dcf("AAPL").await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
        assert_eq!(history[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_advanced_dcf() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_advanced_dcf("AAPL").await;
        assert!(result.is_ok());
        let dcf = result.unwrap();
        assert!(!dcf.is_empty());
        assert!(dcf[0].wacc.is_some());
        assert!(dcf[0].revenue_growth.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_levered_dcf() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_levered_dcf("AAPL").await;
        assert!(result.is_ok());
        let dcf = result.unwrap();
        assert!(!dcf.is_empty());
        assert!(dcf[0].levered_dcf.is_some());
        assert!(dcf[0].enterprise_value.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_enterprise_value() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_enterprise_value("AAPL").await;
        assert!(result.is_ok());
        let ev = result.unwrap();
        assert!(!ev.is_empty());
        assert!(ev[0].enterprise_value.is_some());
        assert!(ev[0].market_capitalization.is_some());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_dcf_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_dcf("INVALID_XYZ123").await;
        // Should succeed but return empty for invalid symbol
        assert!(result.is_ok());
        if let Ok(dcf) = result {
            assert!(dcf.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_dcf_multiple_records() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_historical_dcf("MSFT").await;
        assert!(result.is_ok());
        let history = result.unwrap();
        // Should have multiple historical records
        assert!(history.len() > 1);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_enterprise_value_calculations() {
        let client = FmpClient::new().unwrap();
        let result = client.dcf().get_enterprise_value("GOOGL").await;
        assert!(result.is_ok());
        let ev = result.unwrap();
        assert!(!ev.is_empty());
        // Verify enterprise value components are present
        let first = &ev[0];
        assert!(first.market_capitalization.is_some());
        assert!(first.add_total_debt.is_some());
        assert!(first.minus_cash_and_cash_equivalents.is_some());
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.dcf().get_dcf("AAPL").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.dcf().get_dcf("").await;
        // Should fail or return empty
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
