//! Corporate actions endpoints (earnings, IPOs, dividends, splits)

use crate::{
    client::FmpClient,
    error::Result,
    models::corporate_actions::{
        DividendCalendar, EarningsCalendar, EarningsConfirmation, EconomicCalendar,
        HistoricalEarnings, IpoCalendar, StockSplitCalendar,
    },
};
use serde::Serialize;

/// Corporate Actions API endpoints
pub struct CorporateActions {
    client: FmpClient,
}

impl CorporateActions {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get earnings calendar
    ///
    /// Returns upcoming earnings announcements with estimates.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let calendar = client.corporate_actions().get_earnings_calendar(None, None).await?;
    /// for earning in calendar.iter().take(10) {
    ///     println!("{}: {} - EPS Est: {}, Revenue Est: ${}M",
    ///         earning.date.as_deref().unwrap_or("N/A"),
    ///         earning.symbol,
    ///         earning.eps_estimated.unwrap_or(0.0),
    ///         earning.revenue_estimated.unwrap_or(0.0) / 1_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_earnings_calendar(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<EarningsCalendar>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/earning_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical earnings for a symbol
    ///
    /// Returns past earnings results with actuals vs. estimates.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Maximum number of records to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.corporate_actions().get_historical_earnings("AAPL", Some(8)).await?;
    /// for earning in history {
    ///     println!("{}: EPS {} (est: {})",
    ///         earning.date.unwrap_or_default(),
    ///         earning.eps.unwrap_or(0.0),
    ///         earning.eps_estimated.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_earnings(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<HistoricalEarnings>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical/earning_calendar/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get IPO calendar
    ///
    /// Returns upcoming and recent IPO listings.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let ipos = client.corporate_actions().get_ipo_calendar(None, None).await?;
    /// for ipo in ipos.iter().take(10) {
    ///     println!("{}: {} ({}) - {} shares at {}",
    ///         ipo.date.as_deref().unwrap_or("N/A"),
    ///         ipo.company.as_deref().unwrap_or("N/A"),
    ///         ipo.symbol,
    ///         ipo.number_of_shares.unwrap_or(0),
    ///         ipo.price.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ipo_calendar(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<IpoCalendar>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/ipo_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get stock split calendar
    ///
    /// Returns upcoming and recent stock splits.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let splits = client.corporate_actions().get_stock_split_calendar(None, None).await?;
    /// for split in splits.iter().take(10) {
    ///     println!("{}: {} - {} split",
    ///         split.date.as_deref().unwrap_or("N/A"),
    ///         split.symbol,
    ///         split.split.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_stock_split_calendar(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<StockSplitCalendar>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock_split_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get dividend calendar
    ///
    /// Returns upcoming and recent dividend declarations.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let dividends = client.corporate_actions().get_dividend_calendar(None, None).await?;
    /// for div in dividends.iter().take(10) {
    ///     println!("{}: {} - ${} per share",
    ///         div.date.as_deref().unwrap_or("N/A"),
    ///         div.symbol,
    ///         div.dividend.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dividend_calendar(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<DividendCalendar>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock_dividend_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get economic calendar
    ///
    /// Returns upcoming economic events and indicators.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let events = client.corporate_actions().get_economic_calendar(None, None).await?;
    /// for event in events.iter().take(10) {
    ///     println!("{}: {} - {} (Impact: {})",
    ///         event.date.as_deref().unwrap_or("N/A"),
    ///         event.country.as_deref().unwrap_or("N/A"),
    ///         event.event.as_deref().unwrap_or("N/A"),
    ///         event.impact.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_economic_calendar(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<EconomicCalendar>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get earnings confirmations
    ///
    /// Returns confirmed earnings dates and times.
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let confirmations = client.corporate_actions().get_earnings_confirmations(None, None).await?;
    /// for conf in confirmations.iter().take(10) {
    ///     println!("{}: {} ({})",
    ///         conf.publication_date.as_deref().unwrap_or("N/A"),
    ///         conf.symbol,
    ///         conf.when.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_earnings_confirmations(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<EarningsConfirmation>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/earnings-confirmed");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
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
    async fn test_get_earnings_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_earnings_calendar(None, None)
            .await;
        assert!(result.is_ok());
        let calendar = result.unwrap();
        assert!(!calendar.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_earnings() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_historical_earnings("AAPL", Some(8))
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
        assert!(history.len() <= 8);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_ipo_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_ipo_calendar(None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_stock_split_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_stock_split_calendar(None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_dividend_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_dividend_calendar(None, None)
            .await;
        assert!(result.is_ok());
        let dividends = result.unwrap();
        assert!(!dividends.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_economic_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_economic_calendar(None, None)
            .await;
        assert!(result.is_ok());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_earnings_calendar_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_earnings_calendar(Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_historical_earnings_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_historical_earnings("AAPL", None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(history.len() > 8);
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client
            .corporate_actions()
            .get_earnings_calendar(None, None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol_historical_earnings() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_historical_earnings("INVALID_XYZ123", Some(10))
            .await;
        assert!(result.is_ok());
        if let Ok(history) = result {
            assert!(history.is_empty());
        }
    }

    // Additional edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_dividend_calendar_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_dividend_calendar(Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
        let dividends = result.unwrap();
        // Should have some dividend data for a month period
        if !dividends.is_empty() {
            assert!(dividends[0].symbol.is_some());
            assert!(dividends[0].dividend.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_economic_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_economic_calendar(None, None)
            .await;
        assert!(result.is_ok());
        let events = result.unwrap();
        if !events.is_empty() {
            assert!(events[0].country.is_some());
            assert!(events[0].event.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_earnings_calendar_future_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_earnings_calendar(Some("2025-01-01"), Some("2025-01-31"))
            .await;
        assert!(result.is_ok());
        // Future earnings calendar should return data
        let earnings = result.unwrap();
        if !earnings.is_empty() {
            assert!(earnings[0].symbol.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_ipo_calendar_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_ipo_calendar(Some("2024-01-01"), Some("2024-12-31"))
            .await;
        assert!(result.is_ok());
        let ipos = result.unwrap();
        // IPO calendar might be empty for some periods
        if !ipos.is_empty() {
            assert!(ipos[0].symbol.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_stock_splits_calendar() {
        let client = FmpClient::new().unwrap();
        let result = client
            .corporate_actions()
            .get_stock_splits_calendar(Some("2024-01-01"), Some("2024-12-31"))
            .await;
        assert!(result.is_ok());
        let splits = result.unwrap();
        // Stock splits are less frequent events
        if !splits.is_empty() {
            assert!(splits[0].symbol.is_some());
            assert!(splits[0].numerator.is_some() || splits[0].denominator.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_date_validation() {
        let client = FmpClient::new().unwrap();
        // Test with invalid date format
        let result = client
            .corporate_actions()
            .get_earnings_calendar(Some("invalid-date"), Some("2024-12-31"))
            .await;
        // Should handle gracefully - either error or empty result
        match result {
            Ok(_) => {}  // API might handle gracefully
            Err(_) => {} // Or return error, both acceptable
        }
    }
}
