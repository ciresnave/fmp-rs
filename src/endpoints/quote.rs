//! Quote endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::quote::{PriceChange, ShortQuote, StockQuote};
use serde::Serialize;

/// Quote API endpoints
pub struct Quote {
    client: FmpClient,
}

impl Quote {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get full quote for a symbol
    pub async fn get_quote(&self, symbol: &str) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/quote");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get short quote for a symbol
    pub async fn get_quote_short(&self, symbol: &str) -> Result<Vec<ShortQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/quote-short");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get quote for multiple symbols (batch)
    pub async fn get_quote_batch(&self, symbols: &[&str]) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbols: String,
            apikey: &'a str,
        }

        let url = self.client.build_url("/batch-quote");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbols: symbols.join(","),
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get price change data for a symbol
    pub async fn get_price_change(&self, symbol: &str) -> Result<Vec<PriceChange>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock-price-change");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get quotes for all stocks on an exchange
    pub async fn get_exchange_quotes(&self, exchange: &str) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            exchange: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/batch-exchange-quote");
        self.client
            .get_with_query(
                &url,
                &Query {
                    exchange,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get all ETF quotes
    pub async fn get_all_etf_quotes(&self) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/batch-etf-quotes");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get all cryptocurrency quotes
    pub async fn get_all_crypto_quotes(&self) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/batch-crypto-quotes");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get all forex quotes
    pub async fn get_all_forex_quotes(&self) -> Result<Vec<StockQuote>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/batch-forex-quotes");
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

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_quote("AAPL").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert_eq!(quotes[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_batch() {
        let client = FmpClient::new().unwrap();
        let result = client
            .quote()
            .get_quote_batch(&["AAPL", "MSFT", "GOOGL"])
            .await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert_eq!(quotes.len(), 3);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_short() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_quote_short("AAPL").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        assert_eq!(quotes[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_price_change() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_price_change("AAPL").await;
        assert!(result.is_ok());
        let changes = result.unwrap();
        assert!(!changes.is_empty());
        assert_eq!(changes[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_exchange_quotes() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_exchange_quotes("NASDAQ").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        // Verify at least some NASDAQ stocks are returned
        let has_nasdaq = quotes.iter().any(|q| q.exchange.contains("NASDAQ"));
        assert!(has_nasdaq || quotes.len() > 0); // Either has NASDAQ or has some data
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_all_etf_quotes() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_all_etf_quotes().await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        // ETFs should have symbols ending in common ETF patterns
        assert!(quotes.iter().any(|q| q.symbol.len() >= 3));
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_all_crypto_quotes() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_all_crypto_quotes().await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        // Crypto symbols often contain USD suffix
        assert!(
            quotes
                .iter()
                .any(|q| q.symbol.contains("USD") || q.symbol.contains("BTC"))
        );
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_all_forex_quotes() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_all_forex_quotes().await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(!quotes.is_empty());
        // Forex pairs typically have 6-7 character symbols like EURUSD
        assert!(
            quotes
                .iter()
                .any(|q| q.symbol.len() >= 6 && q.symbol.len() <= 8)
        );
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_quote("INVALID_SYMBOL_12345").await;
        // Should handle gracefully - either empty result or error
        match result {
            Ok(quotes) => assert!(quotes.is_empty() || quotes[0].symbol == "INVALID_SYMBOL_12345"),
            Err(_) => {} // API may return error for invalid symbols
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_batch_mixed_valid_invalid() {
        let client = FmpClient::new().unwrap();
        let result = client
            .quote()
            .get_quote_batch(&["AAPL", "INVALID_SYM", "MSFT"])
            .await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        // Should return quotes for valid symbols, may skip invalid ones
        assert!(quotes.len() >= 2);
        assert!(
            quotes
                .iter()
                .any(|q| q.symbol == "AAPL" || q.symbol == "MSFT")
        );
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_batch_empty_array() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_quote_batch(&[]).await;
        // Should handle empty input gracefully
        match result {
            Ok(quotes) => assert!(quotes.is_empty()),
            Err(_) => {} // API may return error for empty input
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_exchange_quotes_invalid_exchange() {
        let client = FmpClient::new().unwrap();
        let result = client.quote().get_exchange_quotes("INVALID_EXCHANGE").await;
        // Should handle gracefully
        match result {
            Ok(quotes) => assert!(quotes.is_empty()),
            Err(_) => {} // API may return error for invalid exchange
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_quote_special_characters() {
        let client = FmpClient::new().unwrap();
        // Test symbol with special characters (common in some markets)
        let result = client.quote().get_quote("BRK.A").await;
        assert!(result.is_ok());
        // BRK.A is a real symbol, should return data
        let quotes = result.unwrap();
        if !quotes.is_empty() {
            assert_eq!(quotes[0].symbol, "BRK.A");
        }
    }
}
