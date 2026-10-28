//! Insider trading endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::institutional::InsiderTrade;

/// Insider Trading API endpoints
pub struct InsiderTrades {
    client: FmpClient,
}

impl InsiderTrades {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Search insider trades for a symbol (paid plans only: HTTP 402 on the free tier,
    /// returned as [`crate::Error::PlanRestricted`]; the response shape is assumed to
    /// match [`Self::get_latest_insider_trades`])
    ///
    /// ```no_run
    /// use fmp_rs::FmpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let trades = client.insider_trades().get_insider_trading("AAPL", Some(10)).await?;
    /// for t in &trades {
    ///     println!("{} {} {}", t.transaction_date, t.reporting_name, t.transaction_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_trading(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<InsiderTrade>> {
        let limit = limit.map(|l| l.to_string());
        let mut params = vec![("symbol", symbol)];
        if let Some(l) = &limit {
            params.push(("limit", l));
        }
        self.client
            .get_stable("/insider-trading/search", &params)
            .await
    }

    /// Get the latest insider trades across all companies (works on the free tier)
    pub async fn get_latest_insider_trades(&self, limit: Option<u32>) -> Result<Vec<InsiderTrade>> {
        let limit = limit.map(|l| l.to_string());
        let mut params = Vec::new();
        if let Some(l) = &limit {
            params.push(("limit", l.as_str()));
        }
        self.client
            .get_stable("/insider-trading/latest", &params)
            .await
    }
}
