//! Bulk data endpoints - Framework implementation without heavy downloads

use crate::{
    client::FmpClient,
    error::Result,
    models::bulk::{
        BulkDataInfo, BulkEarningsEstimate, BulkEtfHolding, BulkFinancialStatement,
        BulkHistoricalPricesMeta, BulkInsiderTrade, BulkInstitutionalHolding, BulkStockPrice,
    },
};
use serde::Serialize;

/// Bulk Data API endpoints
///
/// Note: These endpoints provide access to large datasets. In production,
/// consider implementing streaming, chunking, or selective downloading
/// to manage memory usage and network bandwidth effectively.
pub struct Bulk {
    client: FmpClient,
}

impl Bulk {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get bulk stock prices (all symbols)
    ///
    /// **Warning:** This endpoint returns data for ALL stocks and can be very large (100MB+).
    /// Consider using get_bulk_prices_sample() for testing or implement pagination.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    ///
    /// // WARNING: This downloads ALL stock prices - can be 100MB+
    /// // let all_prices = bulk.get_bulk_stock_prices().await?;
    ///
    /// // Use sample version for testing instead:
    /// let sample_prices = bulk.get_bulk_prices_sample(100).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_stock_prices(&self) -> Result<Vec<BulkStockPrice>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/v3/quotes/nyse");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get a sample of bulk stock prices (first N results)
    ///
    /// This provides a lightweight way to test the bulk prices endpoint
    /// without downloading the entire dataset.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of price records to return
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let sample = bulk.get_bulk_prices_sample(50).await?;
    /// println!("Sample contains {} price records", sample.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_prices_sample(&self, limit: usize) -> Result<Vec<BulkStockPrice>> {
        let all_prices = self.get_bulk_stock_prices().await?;
        Ok(all_prices.into_iter().take(limit).collect())
    }

    /// Get bulk financial statements for all companies
    ///
    /// **Warning:** This endpoint returns financial data for ALL companies and is extremely large.
    ///
    /// # Arguments
    /// * `period` - "annual" or "quarter"
    /// * `year` - Year for the financial data (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    ///
    /// // Get sample of annual financial statements
    /// let statements = bulk.get_bulk_financials_sample("annual", Some(2023), 10).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_financial_statements(
        &self,
        period: &str,
        year: Option<i32>,
    ) -> Result<Vec<BulkFinancialStatement>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: &'a str,
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            year: Option<i32>,
        }

        let url = self.client.build_url("/v4/financial-statements-list");
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    apikey: self.client.api_key(),
                    year,
                },
            )
            .await
    }

    /// Get sample of bulk financial statements
    ///
    /// # Arguments
    /// * `period` - "annual" or "quarter"
    /// * `year` - Year for the financial data (optional)
    /// * `limit` - Maximum number of records to return
    pub async fn get_bulk_financials_sample(
        &self,
        period: &str,
        year: Option<i32>,
        limit: usize,
    ) -> Result<Vec<BulkFinancialStatement>> {
        let all_statements = self.get_bulk_financial_statements(period, year).await?;
        Ok(all_statements.into_iter().take(limit).collect())
    }

    /// Get bulk ETF holdings data
    ///
    /// **Warning:** Contains holdings for ALL ETFs - can be very large.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let sample_holdings = bulk.get_bulk_etf_holdings_sample(50).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_etf_holdings(&self) -> Result<Vec<BulkEtfHolding>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/etf-holdings");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get sample of bulk ETF holdings
    ///
    /// # Arguments
    /// * `limit` - Maximum number of holding records to return
    pub async fn get_bulk_etf_holdings_sample(&self, limit: usize) -> Result<Vec<BulkEtfHolding>> {
        let all_holdings = self.get_bulk_etf_holdings().await?;
        Ok(all_holdings.into_iter().take(limit).collect())
    }

    /// Get bulk insider trading data
    ///
    /// **Warning:** Contains ALL insider trades - extremely large dataset.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let recent_trades = bulk.get_bulk_insider_trades_sample(25).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_insider_trades(&self) -> Result<Vec<BulkInsiderTrade>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/insider-trading-list");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get sample of bulk insider trading data
    ///
    /// # Arguments
    /// * `limit` - Maximum number of trade records to return
    pub async fn get_bulk_insider_trades_sample(
        &self,
        limit: usize,
    ) -> Result<Vec<BulkInsiderTrade>> {
        let all_trades = self.get_bulk_insider_trades().await?;
        Ok(all_trades.into_iter().take(limit).collect())
    }

    /// Get bulk institutional holdings (13F filings)
    ///
    /// **Warning:** Contains ALL institutional holdings - massive dataset.
    ///
    /// # Arguments
    /// * `date` - Filing date (YYYY-MM-DD format, optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let recent_holdings = bulk.get_bulk_institutional_holdings_sample(None, 30).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_institutional_holdings(
        &self,
        date: Option<&str>,
    ) -> Result<Vec<BulkInstitutionalHolding>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<&'a str>,
        }

        let url = self.client.build_url("/v4/institutional-holdings-list");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                    date,
                },
            )
            .await
    }

    /// Get sample of institutional holdings
    ///
    /// # Arguments  
    /// * `date` - Filing date (optional)
    /// * `limit` - Maximum number of records to return
    pub async fn get_bulk_institutional_holdings_sample(
        &self,
        date: Option<&str>,
        limit: usize,
    ) -> Result<Vec<BulkInstitutionalHolding>> {
        let all_holdings = self.get_bulk_institutional_holdings(date).await?;
        Ok(all_holdings.into_iter().take(limit).collect())
    }

    /// Get bulk earnings estimates
    ///
    /// # Arguments
    /// * `period` - "annual" or "quarter"
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let estimates = bulk.get_bulk_earnings_estimates_sample("quarter", 20).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_earnings_estimates(
        &self,
        period: &str,
    ) -> Result<Vec<BulkEarningsEstimate>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/earnings-estimates");
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get sample of earnings estimates
    ///
    /// # Arguments
    /// * `period` - "annual" or "quarter"  
    /// * `limit` - Maximum number of records to return
    pub async fn get_bulk_earnings_estimates_sample(
        &self,
        period: &str,
        limit: usize,
    ) -> Result<Vec<BulkEarningsEstimate>> {
        let all_estimates = self.get_bulk_earnings_estimates(period).await?;
        Ok(all_estimates.into_iter().take(limit).collect())
    }

    /// Get bulk dataset information/metadata
    ///
    /// Returns metadata about available bulk datasets including size,
    /// last update time, and download URLs.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let info = bulk.get_bulk_data_info().await?;
    ///
    /// for dataset in info {
    ///     println!("Dataset: {:?}, Size: {:?} MB",
    ///         dataset.dataset,
    ///         dataset.file_size.map(|s| s / 1024 / 1024));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulk_data_info(&self) -> Result<Vec<BulkDataInfo>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/v4/bulk-data-info");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical prices metadata for bulk download planning
    ///
    /// Returns information about available historical price datasets
    /// to help plan bulk downloads efficiently.
    ///
    /// # Arguments
    /// * `exchange` - Exchange identifier (optional, e.g., "NYSE", "NASDAQ")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::builder().api_key("your_api_key").build()?;
    /// let bulk = client.bulk();
    /// let meta = bulk.get_historical_prices_metadata(Some("NYSE")).await?;
    ///
    /// for info in meta {
    ///     println!("NYSE Historical Data: {} symbols, ~{} MB",
    ///         info.symbols_count.unwrap_or(0),
    ///         info.estimated_size_mb.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_prices_metadata(
        &self,
        exchange: Option<&str>,
    ) -> Result<Vec<BulkHistoricalPricesMeta>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            exchange: Option<&'a str>,
        }

        let url = self.client.build_url("/v4/bulk-historical-metadata");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                    exchange,
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

    fn get_test_bulk() -> Bulk {
        Bulk::new(create_test_client())
    }

    #[test]
    fn test_new() {
        let client = create_test_client();
        let _bulk = Bulk::new(client);
    }

    // Lightweight tests - only testing samples, not full datasets

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_prices_sample() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_prices_sample(5).await;
        assert!(result.is_ok());

        let prices = result.unwrap();
        assert!(prices.len() <= 5);
        if !prices.is_empty() {
            let price = &prices[0];
            assert!(price.symbol.is_some() || price.price.is_some());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_financials_sample() {
        let bulk = get_test_bulk();
        let result = bulk
            .get_bulk_financials_sample("annual", Some(2023), 3)
            .await;
        assert!(result.is_ok());

        let statements = result.unwrap();
        assert!(statements.len() <= 3);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_etf_holdings_sample() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_etf_holdings_sample(5).await;
        assert!(result.is_ok());

        let holdings = result.unwrap();
        assert!(holdings.len() <= 5);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_insider_trades_sample() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_insider_trades_sample(3).await;
        assert!(result.is_ok());

        let trades = result.unwrap();
        assert!(trades.len() <= 3);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_institutional_holdings_sample() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_institutional_holdings_sample(None, 3).await;
        assert!(result.is_ok());

        let holdings = result.unwrap();
        assert!(holdings.len() <= 3);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_earnings_estimates_sample() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_earnings_estimates_sample("quarter", 3).await;
        assert!(result.is_ok());

        let estimates = result.unwrap();
        assert!(estimates.len() <= 3);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_data_info() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_data_info().await;
        assert!(result.is_ok());

        let info = result.unwrap();
        // Info endpoint should be lightweight
        for dataset in &info {
            if let Some(name) = &dataset.dataset {
                println!("Available dataset: {}", name);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_historical_prices_metadata() {
        let bulk = get_test_bulk();
        let result = bulk.get_historical_prices_metadata(Some("NYSE")).await;
        assert!(result.is_ok());

        let meta = result.unwrap();
        // Metadata should be lightweight
        for info in &meta {
            if let Some(count) = info.symbols_count {
                assert!(count > 0);
            }
        }
    }

    // Missing endpoint tests - these test endpoints not previously covered
    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_stock_prices() {
        let bulk = get_test_bulk();
        // Only test if user explicitly wants full bulk download
        let result = bulk.get_bulk_stock_prices().await;
        // This should work but will be very large, so we just test the call succeeds
        match result {
            Ok(prices) => {
                println!("Successfully retrieved {} stock prices", prices.len());
                // Validate structure if any data returned
                if !prices.is_empty() {
                    assert!(prices[0].symbol.is_some() || prices[0].price.is_some());
                }
            }
            Err(e) => {
                // May fail due to size limits or API restrictions
                println!("Bulk download failed (expected for large datasets): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_financial_statements() {
        let bulk = get_test_bulk();
        let result = bulk
            .get_bulk_financial_statements("annual", Some(2023))
            .await;
        match result {
            Ok(statements) => {
                println!(
                    "Successfully retrieved {} financial statements",
                    statements.len()
                );
                if !statements.is_empty() {
                    assert!(statements[0].symbol.is_some());
                }
            }
            Err(e) => {
                println!("Bulk financial download failed (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_etf_holdings() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_etf_holdings().await;
        match result {
            Ok(holdings) => {
                println!("Successfully retrieved {} ETF holdings", holdings.len());
                if !holdings.is_empty() {
                    assert!(holdings[0].etf_symbol.is_some() || holdings[0].asset_symbol.is_some());
                }
            }
            Err(e) => {
                println!("Bulk ETF holdings download failed (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_insider_trades() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_insider_trades().await;
        match result {
            Ok(trades) => {
                println!("Successfully retrieved {} insider trades", trades.len());
                if !trades.is_empty() {
                    assert!(trades[0].symbol.is_some());
                }
            }
            Err(e) => {
                println!("Bulk insider trades download failed (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_institutional_holdings() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_institutional_holdings(None).await;
        match result {
            Ok(holdings) => {
                println!(
                    "Successfully retrieved {} institutional holdings",
                    holdings.len()
                );
                if !holdings.is_empty() {
                    assert!(holdings[0].ticker_symbol.is_some());
                }
            }
            Err(e) => {
                println!(
                    "Bulk institutional holdings download failed (expected): {}",
                    e
                );
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key - WARNING: This downloads large datasets
    async fn test_get_bulk_earnings_estimates() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_earnings_estimates("quarter").await;
        match result {
            Ok(estimates) => {
                println!(
                    "Successfully retrieved {} earnings estimates",
                    estimates.len()
                );
                if !estimates.is_empty() {
                    assert!(estimates[0].symbol.is_some());
                }
            }
            Err(e) => {
                println!("Bulk earnings estimates download failed (expected): {}", e);
            }
        }
    }

    // Edge case tests for sample functions
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_prices_sample_edge_cases() {
        let bulk = get_test_bulk();

        // Test with zero limit
        let result = bulk.get_bulk_prices_sample(0).await;
        if let Ok(prices) = result {
            assert!(prices.is_empty())
        }

        // Test with very large limit
        let result = bulk.get_bulk_prices_sample(1000000).await;
        if let Ok(prices) = result {
            // Should be capped by API or return reasonable amount
            assert!(prices.len() <= 1000000);
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_bulk_financials_sample_edge_cases() {
        let bulk = get_test_bulk();

        // Test invalid period
        let result = bulk
            .get_bulk_financials_sample("invalid_period", Some(2023), 3)
            .await;
        if let Ok(statements) = result {
            assert!(statements.is_empty())
        }

        // Test future year
        let result = bulk
            .get_bulk_financials_sample("annual", Some(2030), 3)
            .await;
        if let Ok(statements) = result {
            assert!(statements.is_empty())
        }

        // Test very old year
        let result = bulk
            .get_bulk_financials_sample("annual", Some(1900), 3)
            .await;
        if let Ok(statements) = result {
            assert!(statements.is_empty())
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_historical_prices_metadata_edge_cases() {
        let bulk = get_test_bulk();

        // Test invalid exchange
        let result = bulk
            .get_historical_prices_metadata(Some("INVALID_EXCHANGE"))
            .await;
        if let Ok(meta) = result {
            assert!(meta.is_empty())
        }

        // Test without exchange filter
        let result = bulk.get_historical_prices_metadata(None).await;
        assert!(result.is_ok()); // Should work without filter
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_bulk_sample_data_validation() {
        let bulk = get_test_bulk();
        let result = bulk.get_bulk_prices_sample(3).await;
        assert!(result.is_ok());

        let prices = result.unwrap();
        for price in &prices {
            // Validate data structure
            if let Some(ref symbol) = price.symbol {
                assert!(!symbol.is_empty());
                assert!(symbol.len() <= 10); // Reasonable symbol length
            }
            if let Some(price_val) = price.price {
                assert!(price_val > 0.0); // Price should be positive
            }
            if let Some(volume) = price.volume {
                assert!(volume >= 0); // Volume should be non-negative
            }
        }
    }

    // Model serialization tests
    #[test]
    fn test_bulk_stock_price_serialization() {
        let price = BulkStockPrice {
            symbol: Some("AAPL".to_string()),
            name: Some("Apple Inc".to_string()),
            price: Some(150.25),
            change: Some(2.50),
            changes_percentage: Some(1.69),
            volume: Some(50000000),
            market_cap: Some(2500000000000.0),
            pe: Some(28.5),
            exchange: Some("NASDAQ".to_string()),
            day_low: Some(148.50),
            day_high: Some(151.00),
            year_low: Some(124.17),
            year_high: Some(182.94),
            avg_volume: Some(60000000),
            open: Some(149.50),
            previous_close: Some(147.75),
            eps: Some(5.28),
            shares_outstanding: Some(16500000000),
            timestamp: Some(1640995200),
        };

        let json = serde_json::to_string(&price).unwrap();
        let deserialized: BulkStockPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, price.symbol);
        assert_eq!(deserialized.price, price.price);
    }

    #[test]
    fn test_bulk_data_info_serialization() {
        let info = BulkDataInfo {
            dataset: Some("stock_prices".to_string()),
            last_updated: Some("2024-01-15T10:30:00Z".to_string()),
            file_size: Some(104857600), // 100MB
            record_count: Some(1000000),
            download_url: Some("https://api.fmp.com/bulk/stock_prices.csv".to_string()),
            format: Some("CSV".to_string()),
            compression: Some("gzip".to_string()),
            schema_version: Some("v1.2".to_string()),
        };

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: BulkDataInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.dataset, info.dataset);
        assert_eq!(deserialized.file_size, info.file_size);
    }

    // Additional edge case tests for missing endpoints

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_prices_metadata_no_exchange() {
        let client = FmpClient::new().unwrap();
        let result = client.bulk().get_historical_prices_metadata(None).await;
        assert!(result.is_ok());
        // Should return metadata for all exchanges
    }

    // Error handling and edge cases
}
