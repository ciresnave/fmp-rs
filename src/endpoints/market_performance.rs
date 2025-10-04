//! Market performance and market overview endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::market_performance::{
        MarketHours, PrePostMarket, SectorHistorical, SectorPerformance, StockMover,
    },
};
use serde::Serialize;

/// Market Performance API endpoints
pub struct MarketPerformance {
    client: FmpClient,
}

impl MarketPerformance {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get current sector performance
    ///
    /// Returns the current day's performance for all market sectors.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let sectors = client.market_performance().get_sector_performance().await?;
    /// for sector in sectors {
    ///     println!("{}: {}", sector.sector, sector.changes_percentage.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sector_performance(&self) -> Result<Vec<SectorPerformance>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/sector-performance");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical sector performance
    ///
    /// Returns historical performance data for all sectors over time.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of records to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let history = client.market_performance().get_sectors_historical(Some(30)).await?;
    /// for record in history.iter().take(5) {
    ///     println!("{}: Tech {:.2}%, Healthcare {:.2}%",
    ///         record.date,
    ///         record.technology_changes_percentage.unwrap_or(0.0),
    ///         record.healthcare_changes_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sectors_historical(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<SectorHistorical>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/historical-sectors-performance");
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

    /// Get market hours for exchanges
    ///
    /// Returns trading hours, holidays, and current market status.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let hours = client.market_performance().get_market_hours().await?;
    /// for market in hours {
    ///     if let Some(name) = &market.stock_exchange_name {
    ///         println!("Exchange: {}", name);
    ///         println!("Is Open: {}", market.is_the_stock_market_open.unwrap_or(false));
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_hours(&self) -> Result<Vec<MarketHours>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/market-hours");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get stock market gainers
    ///
    /// Returns the top gaining stocks for the current trading day.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let gainers = client.market_performance().get_gainers().await?;
    /// for stock in gainers.iter().take(10) {
    ///     println!("{} ({}): +{:.2}% at ${:.2}",
    ///         stock.name.as_deref().unwrap_or(""),
    ///         stock.symbol,
    ///         stock.changes_percentage.unwrap_or(0.0),
    ///         stock.price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_gainers(&self) -> Result<Vec<StockMover>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock_market/gainers");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get stock market losers
    ///
    /// Returns the top losing stocks for the current trading day.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let losers = client.market_performance().get_losers().await?;
    /// for stock in losers.iter().take(10) {
    ///     println!("{} ({}): {:.2}% at ${:.2}",
    ///         stock.name.as_deref().unwrap_or(""),
    ///         stock.symbol,
    ///         stock.changes_percentage.unwrap_or(0.0),
    ///         stock.price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_losers(&self) -> Result<Vec<StockMover>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock_market/losers");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get most active stocks
    ///
    /// Returns the most actively traded stocks by volume.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let active = client.market_performance().get_most_active().await?;
    /// for stock in active.iter().take(10) {
    ///     println!("{} ({}): {} shares at ${:.2}",
    ///         stock.name.as_deref().unwrap_or(""),
    ///         stock.symbol,
    ///         stock.volume.unwrap_or(0),
    ///         stock.price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_most_active(&self) -> Result<Vec<StockMover>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock_market/actives");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get pre-market and after-hours trading data
    ///
    /// Returns pre-market and after-hours price information for active stocks.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let extended = client.market_performance().get_pre_post_market().await?;
    /// for stock in extended.iter().take(10) {
    ///     println!("{}: Pre ${:.2} ({:+.2}%), After ${:.2} ({:+.2}%)",
    ///         stock.symbol,
    ///         stock.pre_market_price.unwrap_or(0.0),
    ///         stock.pre_market_change_percentage.unwrap_or(0.0),
    ///         stock.after_hours_price.unwrap_or(0.0),
    ///         stock.after_hours_change_percentage.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pre_post_market(&self) -> Result<Vec<PrePostMarket>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/pre-post-market");
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
    async fn test_get_sector_performance() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_sector_performance().await;
        assert!(result.is_ok());
        let sectors = result.unwrap();
        assert!(!sectors.is_empty());
        // Should have major sectors
        assert!(sectors.iter().any(|s| s.sector.contains("Technology")));
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sectors_historical() {
        let client = FmpClient::new().unwrap();
        let result = client
            .market_performance()
            .get_sectors_historical(Some(10))
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
        assert!(history.len() <= 10);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_market_hours() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_market_hours().await;
        assert!(result.is_ok());
        let hours = result.unwrap();
        assert!(!hours.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_gainers() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_gainers().await;
        assert!(result.is_ok());
        let gainers = result.unwrap();
        assert!(!gainers.is_empty());
        // Gainers should have positive change
        if let Some(first) = gainers.first() {
            assert!(first.changes_percentage.unwrap_or(0.0) > 0.0);
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_losers() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_losers().await;
        assert!(result.is_ok());
        let losers = result.unwrap();
        assert!(!losers.is_empty());
        // Losers should have negative change
        if let Some(first) = losers.first() {
            assert!(first.changes_percentage.unwrap_or(0.0) < 0.0);
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_most_active() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_most_active().await;
        assert!(result.is_ok());
        let active = result.unwrap();
        assert!(!active.is_empty());
        // Most active should have significant volume
        if let Some(first) = active.first() {
            assert!(first.volume.unwrap_or(0) > 0);
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_pre_post_market() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_pre_post_market().await;
        assert!(result.is_ok());
        let extended = result.unwrap();
        assert!(!extended.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sectors_historical_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .market_performance()
            .get_sectors_historical(None)
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        // Should return more than 10 records without limit
        assert!(history.len() > 10);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sectors_historical_large_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .market_performance()
            .get_sectors_historical(Some(100))
            .await;
        assert!(result.is_ok());
        let history = result.unwrap();
        assert!(!history.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_gainers_have_symbols() {
        let client = FmpClient::new().unwrap();
        let result = client.market_performance().get_gainers().await;
        assert!(result.is_ok());
        let gainers = result.unwrap();
        // All gainers should have symbols
        for gainer in gainers.iter().take(5) {
            assert!(!gainer.symbol.is_empty());
            assert!(gainer.price.is_some());
        }
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.market_performance().get_sector_performance().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_sectors_historical_zero_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .market_performance()
            .get_sectors_historical(Some(0))
            .await;
        // Should succeed but return empty or minimal data
        assert!(result.is_ok());
    }
}
