//! ETF endpoints
//!
//! Every ETF endpoint is paid-plan only: on the free tier FMP answers HTTP 402, which
//! this crate returns as [`crate::Error::PlanRestricted`] (live-checked 2026-10-28; the
//! routes exist, a wrong route would be a 404). No response has been recorded yet, so
//! rows are returned as raw JSON rather than a typed model that could be wrong.

use crate::client::FmpClient;
use crate::error::Result;
use serde_json::Value;

/// ETF API endpoints
pub struct Etf {
    client: FmpClient,
}

impl Etf {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get a list of all available ETFs (paid plans only)
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let etfs = client.etf().get_etf_list().await?;
    /// println!("{} ETFs", etfs.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_etf_list(&self) -> Result<Vec<Value>> {
        self.client.get_stable("/etf-list", &[]).await
    }

    /// Get what an ETF holds (paid plans only)
    pub async fn get_etf_holdings(&self, symbol: &str) -> Result<Vec<Value>> {
        self.client
            .get_stable("/etf/holdings", &[("symbol", symbol)])
            .await
    }

    /// Get an ETF's sector weightings (paid plans only)
    pub async fn get_etf_sector_weighting(&self, symbol: &str) -> Result<Vec<Value>> {
        self.client
            .get_stable("/etf/sector-weightings", &[("symbol", symbol)])
            .await
    }

    /// Get an ETF's country weightings (paid plans only)
    pub async fn get_etf_country_weighting(&self, symbol: &str) -> Result<Vec<Value>> {
        self.client
            .get_stable("/etf/country-weightings", &[("symbol", symbol)])
            .await
    }

    /// Get general information about an ETF (paid plans only)
    pub async fn get_etf_info(&self, symbol: &str) -> Result<Vec<Value>> {
        self.client
            .get_stable("/etf/info", &[("symbol", symbol)])
            .await
    }
}
