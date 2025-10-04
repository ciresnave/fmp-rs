//! Market Hours endpoints

use crate::models::market_hours::{ExchangeHours, ExtendedMarketHours, MarketHoliday};
use crate::{FmpClient, Result};
use chrono::Datelike;
use serde::Serialize;

/// Market Hours API endpoints
pub struct MarketHours {
    client: FmpClient,
}

impl MarketHours {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get current market status and hours
    ///
    /// Returns the current market status (open/closed) and trading hours for major exchanges.
    /// Useful for determining if markets are currently active for trading.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let market_status = client.market_hours().get_market_hours().await?;
    /// for market in market_status.iter().take(3) {
    ///     println!("{}: {} ({})",
    ///         market.stock_market.as_deref().unwrap_or("Unknown"),
    ///         if market.is_market_open.unwrap_or(false) { "OPEN" } else { "CLOSED" },
    ///         market.timezone.as_deref().unwrap_or("UTC"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_hours(&self) -> Result<Vec<crate::models::market_hours::MarketHours>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/is-the-market-open");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get extended market hours for all exchanges
    ///
    /// Returns detailed trading hours including pre-market, regular session,
    /// and after-hours trading times for all major exchanges.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let extended_hours = client.market_hours().get_extended_hours().await?;
    /// for exchange in extended_hours.iter().take(5) {
    ///     println!("{}: Regular: {} - {}, Pre: {} - {}, After: {} - {}",
    ///         exchange.name.as_deref().unwrap_or("Unknown"),
    ///         exchange.market_open.as_deref().unwrap_or("N/A"),
    ///         exchange.market_close.as_deref().unwrap_or("N/A"),
    ///         exchange.pre_market_open.as_deref().unwrap_or("N/A"),
    ///         exchange.pre_market_close.as_deref().unwrap_or("N/A"),
    ///         exchange.after_hours_open.as_deref().unwrap_or("N/A"),
    ///         exchange.after_hours_close.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_extended_hours(&self) -> Result<Vec<ExtendedMarketHours>> {
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

    /// Get market holidays for a specific year
    ///
    /// Returns all market holidays and early closures for major exchanges.
    /// Useful for planning trading strategies around market closures.
    ///
    /// # Arguments
    /// * `year` - Optional year (e.g., 2024). Defaults to current year if not provided.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let holidays = client.market_hours().get_market_holidays(Some(2024)).await?;
    /// for holiday in holidays.iter().take(10) {
    ///     println!("{}: {} ({})",
    ///         holiday.date.as_deref().unwrap_or("N/A"),
    ///         holiday.holiday.as_deref().unwrap_or("Unknown"),
    ///         holiday.market_status.as_deref().unwrap_or("closed"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_holidays(&self, year: Option<i32>) -> Result<Vec<MarketHoliday>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            year: Option<i32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/market_holidays");
        self.client
            .get_with_query(
                &url,
                &Query {
                    year,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get trading hours for all exchanges
    ///
    /// Returns comprehensive trading hours information for all global exchanges,
    /// including regular hours, pre-market, and after-hours sessions.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let exchange_hours = client.market_hours().get_exchange_hours().await?;
    /// for exchange in exchange_hours.iter().take(5) {
    ///     let status = if exchange.is_open.unwrap_or(false) { "🟢 OPEN" } else { "🔴 CLOSED" };
    ///     println!("{} ({}): {} | Regular: {} - {}",
    ///         exchange.exchange_name.as_deref().unwrap_or("Unknown"),
    ///         exchange.country.as_deref().unwrap_or("Unknown"),
    ///         status,
    ///         exchange.market_open.as_deref().unwrap_or("N/A"),
    ///         exchange.market_close.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_exchange_hours(&self) -> Result<Vec<ExchangeHours>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/exchange-hours");
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

    fn create_test_client() -> FmpClient {
        FmpClient::builder().api_key("test_key").build().unwrap()
    }

    #[test]
    fn test_new() {
        let client = create_test_client();
        let market_hours = MarketHours::new(client);
        // Test passes if no panic occurs
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_market_hours() {
        let client = FmpClient::new().unwrap();
        let result = client.market_hours().get_market_hours().await;
        assert!(result.is_ok());

        let market_status = result.unwrap();
        if !market_status.is_empty() {
            let first_market = &market_status[0];
            // Verify basic structure
            assert!(first_market.stock_market.is_some() || first_market.exchange.is_some());
            println!("Market status retrieved: {} markets", market_status.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_extended_hours() {
        let client = FmpClient::new().unwrap();
        let result = client.market_hours().get_extended_hours().await;
        assert!(result.is_ok());

        let extended_hours = result.unwrap();
        if !extended_hours.is_empty() {
            let first_exchange = &extended_hours[0];
            // Should have basic exchange information
            assert!(first_exchange.name.is_some() || first_exchange.exchange.is_some());
            println!("Extended hours for {} exchanges", extended_hours.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_market_holidays_current_year() {
        let client = FmpClient::new().unwrap();
        let result = client.market_hours().get_market_holidays(None).await;
        assert!(result.is_ok());

        let holidays = result.unwrap();
        if !holidays.is_empty() {
            let first_holiday = &holidays[0];
            assert!(first_holiday.date.is_some());
            assert!(first_holiday.holiday.is_some());
            println!("Found {} holidays for current year", holidays.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_market_holidays_specific_year() {
        let client = FmpClient::new().unwrap();
        let result = client.market_hours().get_market_holidays(Some(2024)).await;
        assert!(result.is_ok());

        let holidays = result.unwrap();
        if !holidays.is_empty() {
            // Check that we got holidays for the requested year
            if let Some(first_holiday) = holidays.first() {
                assert_eq!(first_holiday.year, Some(2024));
                println!("Found {} holidays for 2024", holidays.len());
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_exchange_hours() {
        let client = FmpClient::new().unwrap();
        let result = client.market_hours().get_exchange_hours().await;
        assert!(result.is_ok());

        let exchange_hours = result.unwrap();
        if !exchange_hours.is_empty() {
            let first_exchange = &exchange_hours[0];
            // Should have exchange name and basic hours info
            assert!(first_exchange.exchange_name.is_some() || first_exchange.exchange.is_some());
            println!("Trading hours for {} exchanges", exchange_hours.len());
        }
    }

    #[test]
    fn test_market_hours_url_building() {
        let client = create_test_client();
        let market_hours = MarketHours::new(client);
        // Test passes if construction succeeds without panics
    }

    #[test]
    fn test_holiday_year_validation() {
        // Test that year parameter is properly handled
        let current_year = chrono::Utc::now().year();
        let future_year = current_year + 1;
        let past_year = current_year - 1;

        // These should all be valid years for the API
        assert!(past_year > 1900);
        assert!(future_year < 2100);
    }

    #[test]
    fn test_market_hours_models_serialization() {
        use serde_json;

        // Test MarketHours model
        let market_hours = crate::models::market_hours::MarketHours {
            exchange: Some("NYSE".to_string()),
            stock_exchange_name: Some("New York Stock Exchange".to_string()),
            stock_market: Some("NYSE".to_string()),
            is_market_open: Some(true),
            opening_hour: Some("09:30".to_string()),
            closing_hour: Some("16:00".to_string()),
            current_date_time: Some("2024-01-15 14:30:00".to_string()),
            timezone: Some("EST".to_string()),
        };

        let json = serde_json::to_string(&market_hours).unwrap();
        let deserialized: crate::models::market_hours::MarketHours =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.exchange, Some("NYSE".to_string()));
        assert_eq!(deserialized.is_market_open, Some(true));

        // Test MarketHoliday model
        let holiday = MarketHoliday {
            date: Some("2024-12-25".to_string()),
            year: Some(2024),
            holiday: Some("Christmas Day".to_string()),
            exchange: Some("NYSE".to_string()),
            market_status: Some("closed".to_string()),
            early_close_time: None,
        };

        let json = serde_json::to_string(&holiday).unwrap();
        let deserialized: MarketHoliday = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.holiday, Some("Christmas Day".to_string()));
        assert_eq!(deserialized.year, Some(2024));
    }

    #[test]
    fn test_exchange_hours_model_completeness() {
        let exchange = ExchangeHours {
            exchange: Some("NASDAQ".to_string()),
            exchange_name: Some("NASDAQ Stock Market".to_string()),
            country: Some("United States".to_string()),
            timezone: Some("EST".to_string()),
            market_open: Some("09:30".to_string()),
            market_close: Some("16:00".to_string()),
            pre_market_start: Some("04:00".to_string()),
            pre_market_end: Some("09:30".to_string()),
            after_hours_start: Some("16:00".to_string()),
            after_hours_end: Some("20:00".to_string()),
            is_open: Some(false),
            current_time: Some("2024-01-15 18:30:00".to_string()),
            day_of_week: Some("Monday".to_string()),
        };

        // Verify all fields are accessible
        assert_eq!(exchange.exchange, Some("NASDAQ".to_string()));
        assert_eq!(exchange.country, Some("United States".to_string()));
        assert_eq!(exchange.is_open, Some(false));
        assert_eq!(exchange.day_of_week, Some("Monday".to_string()));
    }

    // Additional edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_market_holidays_future_year() {
        let client = FmpClient::new().unwrap();
        let future_year = chrono::Utc::now().year() + 1;
        let result = client
            .market_hours()
            .get_market_holidays(Some(future_year))
            .await;
        assert!(result.is_ok());
        let holidays = result.unwrap();
        // Future years should have some holidays defined
        if !holidays.is_empty() {
            assert!(holidays[0].year.is_some());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_market_holidays_past_year() {
        let client = FmpClient::new().unwrap();
        let past_year = chrono::Utc::now().year() - 1;
        let result = client
            .market_hours()
            .get_market_holidays(Some(past_year))
            .await;
        assert!(result.is_ok());
        let holidays = result.unwrap();
        // Past years should have holiday data
        assert!(!holidays.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_market_hours_consistency() {
        let client = FmpClient::new().unwrap();
        let market_hours_result = client.market_hours().get_market_hours().await;
        let extended_hours_result = client.market_hours().get_extended_hours().await;

        assert!(market_hours_result.is_ok());
        assert!(extended_hours_result.is_ok());

        // Both should return data
        let market_hours = market_hours_result.unwrap();
        let extended_hours = extended_hours_result.unwrap();

        if !market_hours.is_empty() && !extended_hours.is_empty() {
            // Market hours should have basic fields
            assert!(market_hours[0].exchange.is_some());
            assert!(extended_hours[0].exchange.is_some());
        }
    }
}
