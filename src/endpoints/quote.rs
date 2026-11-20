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
    use serde_json::{json, Value};
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn client(server: &MockServer) -> FmpClient {
        FmpClient::builder()
            .api_key("test_key")
            .base_url(server.uri())
            .build()
            .unwrap()
    }

    /// SYNTHETIC body (not a recording): same field set as tests/fixtures/quote_aapl.live.json.
    fn full_quote(symbol: &str, exchange: &str) -> Value {
        json!({
            "symbol": symbol, "name": symbol, "price": 100.0,
            "changePercentage": 1.5, "change": 1.5, "volume": 1000,
            "dayLow": 99.0, "dayHigh": 101.0, "yearHigh": 120.0, "yearLow": 80.0,
            "marketCap": 1000000, "priceAvg50": 98.0, "priceAvg200": 95.0,
            "exchange": exchange, "open": 99.5, "previousClose": 98.5,
            "timestamp": 1793213533
        })
    }

    async fn serve(server: &MockServer, p: &str, body: Value) {
        Mock::given(method("GET"))
            .and(path(p))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn test_get_quote() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/quote"))
            .and(query_param("symbol", "AAPL"))
            .and(query_param("apikey", "test_key"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([full_quote("AAPL", "NASDAQ")])),
            )
            .expect(1)
            .mount(&server)
            .await;
        let quotes = client(&server).await.quote().get_quote("AAPL").await.unwrap();
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0].symbol, "AAPL");
    }

    #[tokio::test]
    async fn test_get_quote_batch() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/batch-quote"))
            .and(query_param("symbols", "AAPL,MSFT,GOOGL"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                full_quote("AAPL", "NASDAQ"),
                full_quote("MSFT", "NASDAQ"),
                full_quote("GOOGL", "NASDAQ")
            ])))
            .expect(1)
            .mount(&server)
            .await;
        let quotes = client(&server)
            .await
            .quote()
            .get_quote_batch(&["AAPL", "MSFT", "GOOGL"])
            .await
            .unwrap();
        assert_eq!(quotes.len(), 3);
    }

    #[tokio::test]
    async fn test_get_quote_short() {
        let server = MockServer::start().await;
        serve(
            &server,
            "/quote-short",
            json!([{"symbol": "AAPL", "price": 344.35, "volume": 15515541}]),
        )
        .await;
        let quotes = client(&server)
            .await
            .quote()
            .get_quote_short("AAPL")
            .await
            .unwrap();
        assert_eq!(quotes[0].symbol, "AAPL");
        assert_eq!(quotes[0].volume, 15515541);
    }

    #[tokio::test]
    async fn test_get_price_change() {
        let server = MockServer::start().await;
        serve(
            &server,
            "/stock-price-change",
            json!([{"symbol": "AAPL", "1D": 1.5, "5D": 2.0, "ytd": 10.0, "1Y": 20.0, "max": 5000.0}]),
        )
        .await;
        let changes = client(&server)
            .await
            .quote()
            .get_price_change("AAPL")
            .await
            .unwrap();
        assert_eq!(changes[0].symbol, "AAPL");
        assert_eq!(changes[0].one_day, Some(1.5));
        assert_eq!(changes[0].year_to_date, Some(10.0));
    }

    #[tokio::test]
    async fn test_get_exchange_quotes() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/batch-exchange-quote"))
            .and(query_param("exchange", "NASDAQ"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([full_quote("AAPL", "NASDAQ")])),
            )
            .expect(1)
            .mount(&server)
            .await;
        let quotes = client(&server)
            .await
            .quote()
            .get_exchange_quotes("NASDAQ")
            .await
            .unwrap();
        assert!(quotes.iter().all(|q| q.exchange.contains("NASDAQ")));
    }

    #[tokio::test]
    async fn test_get_all_etf_quotes() {
        let server = MockServer::start().await;
        serve(&server, "/batch-etf-quotes", json!([full_quote("SPY", "NYSE")])).await;
        let quotes = client(&server).await.quote().get_all_etf_quotes().await.unwrap();
        assert_eq!(quotes[0].symbol, "SPY");
    }

    #[tokio::test]
    async fn test_get_all_crypto_quotes() {
        let server = MockServer::start().await;
        serve(&server, "/batch-crypto-quotes", json!([full_quote("BTCUSD", "CRYPTO")])).await;
        let quotes = client(&server)
            .await
            .quote()
            .get_all_crypto_quotes()
            .await
            .unwrap();
        assert_eq!(quotes[0].symbol, "BTCUSD");
    }

    #[tokio::test]
    async fn test_get_all_forex_quotes() {
        let server = MockServer::start().await;
        serve(&server, "/batch-forex-quotes", json!([full_quote("EURUSD", "FOREX")])).await;
        let quotes = client(&server)
            .await
            .quote()
            .get_all_forex_quotes()
            .await
            .unwrap();
        assert_eq!(quotes[0].symbol, "EURUSD");
    }

    #[tokio::test]
    async fn test_get_quote_invalid_symbol_returns_empty() {
        let server = MockServer::start().await;
        serve(&server, "/quote", json!([])).await;
        let quotes = client(&server)
            .await
            .quote()
            .get_quote("INVALID_SYMBOL_12345")
            .await
            .unwrap();
        assert!(quotes.is_empty());
    }

    #[tokio::test]
    async fn test_get_quote_batch_mixed_valid_invalid() {
        let server = MockServer::start().await;
        serve(
            &server,
            "/batch-quote",
            json!([full_quote("AAPL", "NASDAQ"), full_quote("MSFT", "NASDAQ")]),
        )
        .await;
        let quotes = client(&server)
            .await
            .quote()
            .get_quote_batch(&["AAPL", "INVALID_SYM", "MSFT"])
            .await
            .unwrap();
        assert_eq!(quotes.len(), 2);
    }

    #[tokio::test]
    async fn test_get_quote_batch_empty_array() {
        let server = MockServer::start().await;
        serve(&server, "/batch-quote", json!([])).await;
        let quotes = client(&server).await.quote().get_quote_batch(&[]).await.unwrap();
        assert!(quotes.is_empty());
    }

    #[tokio::test]
    async fn test_get_exchange_quotes_invalid_exchange() {
        let server = MockServer::start().await;
        serve(&server, "/batch-exchange-quote", json!([])).await;
        let quotes = client(&server)
            .await
            .quote()
            .get_exchange_quotes("INVALID_EXCHANGE")
            .await
            .unwrap();
        assert!(quotes.is_empty());
    }

    #[tokio::test]
    async fn test_get_quote_special_characters() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/quote"))
            .and(query_param("symbol", "BRK.A"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([full_quote("BRK.A", "NYSE")])),
            )
            .expect(1)
            .mount(&server)
            .await;
        let quotes = client(&server).await.quote().get_quote("BRK.A").await.unwrap();
        assert_eq!(quotes[0].symbol, "BRK.A");
    }
}
