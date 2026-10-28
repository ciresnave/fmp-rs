//! Institutional holdings endpoints
//!
//! The 13F endpoints are paid-plan only: on the free tier FMP answers HTTP 402, which
//! this crate returns as [`crate::Error::PlanRestricted`] (live-checked 2026-10-28; the
//! routes exist, a wrong route would be a 404). No response has been recorded yet, so
//! rows are returned as raw JSON rather than a typed model that could be wrong.
//! `year` and `quarter` select the 13F filing period.

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::institutional::CikSearchResult;
use serde_json::Value;

/// Institutional API endpoints
pub struct Institutional {
    client: FmpClient,
}

impl Institutional {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get the institutions holding a symbol for a filing period (paid plans only)
    pub async fn get_institutional_holders(
        &self,
        symbol: &str,
        year: u16,
        quarter: u8,
    ) -> Result<Vec<Value>> {
        let (year, quarter) = (year.to_string(), quarter.to_string());
        self.client
            .get_stable(
                "/institutional-ownership/extract-analytics/holder",
                &[("symbol", symbol), ("year", &year), ("quarter", &quarter)],
            )
            .await
    }

    /// Get an institution's portfolio holdings for a filing period (paid plans only)
    pub async fn get_portfolio_holdings(
        &self,
        cik: &str,
        year: u16,
        quarter: u8,
    ) -> Result<Vec<Value>> {
        let (year, quarter) = (year.to_string(), quarter.to_string());
        self.client
            .get_stable(
                "/institutional-ownership/extract",
                &[("cik", cik), ("year", &year), ("quarter", &quarter)],
            )
            .await
    }

    /// Get an institution's portfolio performance summary (paid plans only)
    pub async fn get_holder_performance_summary(&self, cik: &str) -> Result<Vec<Value>> {
        self.client
            .get_stable(
                "/institutional-ownership/holder-performance-summary",
                &[("cik", cik)],
            )
            .await
    }

    /// Get a symbol's aggregate institutional positions for a filing period (paid plans only)
    pub async fn get_symbol_positions_summary(
        &self,
        symbol: &str,
        year: u16,
        quarter: u8,
    ) -> Result<Vec<Value>> {
        let (year, quarter) = (year.to_string(), quarter.to_string());
        self.client
            .get_stable(
                "/institutional-ownership/symbol-positions-summary",
                &[("symbol", symbol), ("year", &year), ("quarter", &quarter)],
            )
            .await
    }

    /// Get the latest 13F filings (paid plans only)
    pub async fn get_latest_institutional_filings(&self) -> Result<Vec<Value>> {
        self.client
            .get_stable("/institutional-ownership/latest", &[])
            .await
    }

    /// Get the full CIK list (paid plans only)
    pub async fn get_cik_list(&self) -> Result<Vec<Value>> {
        self.client.get_stable("/cik-list", &[]).await
    }

    /// Look up the listed companies for a CIK (works on the free tier)
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// for c in client.institutional().search_cik("0001067983").await? {
    ///     println!("{}: {}", c.symbol, c.company_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_cik(&self, cik: &str) -> Result<Vec<CikSearchResult>> {
        self.client.get_stable("/search-cik", &[("cik", cik)]).await
    }
}
