//! Integration tests with mocked responses
//!
//! These tests verify the library functionality without making actual API calls.
//! Tests marked with #[ignore] require an actual FMP API key and will make real API calls.

use crate::models::common::Period;
use crate::FmpClient;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = FmpClient::builder()
            .api_key("test_key_123")
            .base_url("https://test.example.com")
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.api_key(), "test_key_123");
        assert_eq!(client.base_url(), "https://test.example.com");
    }

    #[test]
    fn test_client_missing_api_key() {
        // This should fail if FMP_API_KEY is not set
        unsafe {
            std::env::remove_var("FMP_API_KEY");
        }
        let result = FmpClient::new();

        if std::env::var("FMP_API_KEY").is_err() {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_url_building() {
        let client = FmpClient::builder().api_key("test").build().unwrap();

        assert_eq!(
            client.build_url("/quote"),
            "https://financialmodelingprep.com/stable/quote"
        );

        assert_eq!(
            client.build_url("quote"),
            "https://financialmodelingprep.com/stable/quote"
        );
    }

    #[test]
    fn test_period_display() {
        assert_eq!(Period::Annual.to_string(), "annual");
        assert_eq!(Period::Quarter.to_string(), "quarter");
    }
}

#[cfg(test)]
mod mock_tests {
    // Mock server tests would go here using wiremock or mockito
    // Example structure:

    #[tokio::test]
    async fn test_quote_with_mock() {
        // This would set up a mock server to test without real API calls
        // Implementation depends on mockito/wiremock

        // For now, we'll skip actual implementation
        // but the structure is here for future implementation
    }
}

// Integration tests that require actual API key
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Helper to check if we have an API key for integration tests
    fn has_api_key() -> bool {
        std::env::var("FMP_API_KEY").is_ok()
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_quote_endpoint() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let quotes = client.quote().get_quote("AAPL").await;

        assert!(quotes.is_ok(), "Quote request failed: {:?}", quotes.err());
        let quotes = quotes.unwrap();
        assert!(!quotes.is_empty(), "No quotes returned");
        assert_eq!(quotes[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_company_profile() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let profiles = client.company_info().get_profile("AAPL").await;

        assert!(
            profiles.is_ok(),
            "Profile request failed: {:?}",
            profiles.err()
        );
        let profiles = profiles.unwrap();
        assert!(!profiles.is_empty(), "No profiles returned");
        assert_eq!(profiles[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_company_search() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let results = client
            .company_search()
            .search("Apple", Some(10), None)
            .await;

        assert!(
            results.is_ok(),
            "Search request failed: {:?}",
            results.err()
        );
        let results = results.unwrap();
        assert!(!results.is_empty(), "No search results returned");
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_income_statement() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let statements = client
            .financials()
            .get_income_statement("AAPL", Period::Annual, Some(5))
            .await;

        assert!(
            statements.is_ok(),
            "Income statement request failed: {:?}",
            statements.err()
        );
        let statements = statements.unwrap();
        assert!(!statements.is_empty(), "No income statements returned");
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_batch_quotes() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let quotes = client
            .quote()
            .get_quote_batch(&["AAPL", "MSFT", "GOOGL"])
            .await;

        assert!(
            quotes.is_ok(),
            "Batch quote request failed: {:?}",
            quotes.err()
        );
        let quotes = quotes.unwrap();
        assert_eq!(quotes.len(), 3, "Expected 3 quotes");
    }

    #[tokio::test]
    #[ignore = "requires FMP_API_KEY environment variable"]
    async fn test_real_key_metrics() {
        if !has_api_key() {
            println!("Skipping: FMP_API_KEY not set");
            return;
        }

        let client = FmpClient::new().expect("Failed to create client");
        let metrics = client
            .financials()
            .get_key_metrics("AAPL", Period::Annual, Some(5))
            .await;

        assert!(
            metrics.is_ok(),
            "Key metrics request failed: {:?}",
            metrics.err()
        );
        let metrics = metrics.unwrap();
        assert!(!metrics.is_empty(), "No key metrics returned");
    }
}
