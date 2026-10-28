//! Commitment of Traders (COT) endpoints
//!
//! Paid plans only: on the free tier FMP answers HTTP 402 "Restricted Endpoint"
//! (live-checked 2026-10-28), which this crate returns as [`crate::Error::PlanRestricted`].

use crate::{
    client::FmpClient,
    error::Result,
    models::cot::{CotRecord, CotReportListItem},
};
use serde::Serialize;

/// Commitment of Traders API endpoints
pub struct Cot {
    client: FmpClient,
}

impl Cot {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get the COT report for a contract
    ///
    /// Long, short and spread positions of market participants.
    ///
    /// # Arguments
    /// * `symbol` - Contract symbol (see [`Cot::get_report_list`])
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let rows = client.cot().get_report("KC", None, None).await?;
    /// println!("{} COT rows", rows.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_report(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CotRecord>> {
        self.dated("/commitment-of-traders-report", symbol, from, to)
            .await
    }

    /// Get the COT analysis for a contract
    ///
    /// Tracks long and short positioning over time.
    ///
    /// # Arguments
    /// * `symbol` - Contract symbol (see [`Cot::get_report_list`])
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    pub async fn get_analysis(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CotRecord>> {
        self.dated("/commitment-of-traders-analysis", symbol, from, to)
            .await
    }

    /// List the contracts that have COT reports
    pub async fn get_report_list(&self) -> Result<Vec<CotReportListItem>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/commitment-of-traders-list");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    async fn dated(
        &self,
        path: &str,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CotRecord>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url(path);
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}
