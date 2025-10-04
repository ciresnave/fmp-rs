# 📈 FMP-RS: Production-Grade Rust Client for Financial Modeling Prep

[![Crates.io](https://img.shields.io/crates/v/fmp-rs.svg)](https://crates.io/crates/fmp-rs)
[![Documentation](https://docs.rs/fmp-rs/badge.svg)](https://docs.rs/fmp-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/ciresnave/fmp-rs/workflows/CI/badge.svg)](https://github.com/ciresnave/fmp-rs/actions)

A **comprehensive, production-ready Rust wrapper** for the [Financial Modeling Prep (FMP) API](https://financialmodelingprep.com/). Designed for high-performance financial applications with enterprise-grade reliability features.

## 🚀 Features

### 📊 Comprehensive API Coverage
- **186 API endpoints** across all FMP categories
- **Complete data models** with full type safety
- **Real-time and historical** financial data
- **Stocks, ETFs, Forex, Crypto, and Commodities**

### 🏗️ Production-Grade Architecture
- **⚡ Intelligent Caching** - Smart TTL strategies per endpoint
- **🛡️ Advanced Retry Logic** - Exponential backoff with jitter  
- **📊 Rate Limiting** - Token bucket algorithm (300 req/sec default)
- **🔗 Connection Pooling** - Optimized HTTP/2 connections
- **📈 Performance Monitoring** - Built-in metrics and health checks
- **🧠 Memory Optimization** - Chunked bulk data processing

### � Developer Experience
- **Zero Configuration** - Sensible defaults, full customization
- **Async/Await Native** - Built on Tokio for maximum performance
- **Type Safety** - Full Rust type system leverage
- **Comprehensive Examples** - Real-world usage patterns
- **Rich Error Handling** - Detailed error context and recovery

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fmp-rs = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

Set your FMP API key as an environment variable:

```bash
export FMP_API_KEY="your_api_key_here"
```

Or in PowerShell:

```powershell
$env:FMP_API_KEY = "your_api_key_here"
```

Then use the client:

```rust
use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client (reads FMP_API_KEY from environment)
    let client = FmpClient::new()?;
    
    // Get a stock quote
    let quotes = client.quote().get_quote("AAPL").await?;
    println!("Apple stock price: ${}", quotes[0].price);
    
    // Get company profile
    let profiles = client.company_info().get_profile("AAPL").await?;
    println!("Company: {}", profiles[0].company_name);
    
    // Get financial statements
    let income_statements = client
        .financials()
        .get_income_statement("AAPL", Period::Annual, Some(5))
        .await?;
    
    for statement in income_statements {
        println!("Year {}: Revenue = ${}", statement.calendar_year, statement.revenue);
    }
    
    Ok(())
}
```

## Usage Examples

### Company Search

```rust
// Search by name
let results = client.company_search()
    .search("Apple", Some(10), None)
    .await?;

// Search by ticker
let results = client.company_search()
    .search_symbol("AAPL", None, None)
    .await?;

// Search by CIK
let results = client.company_search()
    .search_cik("0000320193", None)
    .await?;
```

### Stock Quotes

```rust
// Single quote
let quote = client.quote().get_quote("AAPL").await?;

// Batch quotes
let quotes = client.quote()
    .get_quote_batch(&["AAPL", "MSFT", "GOOGL"])
    .await?;

// Exchange quotes
let nyse_quotes = client.quote()
    .get_exchange_quotes("NYSE")
    .await?;

// Price change data
let changes = client.quote()
    .get_price_change("AAPL")
    .await?;
```

### Financial Statements

```rust
use fmp_rs::models::common::Period;

// Income statement
let income = client.financials()
    .get_income_statement("AAPL", Period::Annual, Some(5))
    .await?;

// Balance sheet
let balance = client.financials()
    .get_balance_sheet("AAPL", Period::Quarter, Some(8))
    .await?;

// Cash flow statement
let cash_flow = client.financials()
    .get_cash_flow_statement("AAPL", Period::Annual, None)
    .await?;

// Key metrics
let metrics = client.financials()
    .get_key_metrics("AAPL", Period::Annual, Some(5))
    .await?;

// Financial ratios
let ratios = client.financials()
    .get_ratios("AAPL", Period::Annual, Some(5))
    .await?;
```

### Historical Prices

```rust
use fmp_rs::models::common::Timeframe;

// Daily historical prices
let daily = client.charts()
    .get_historical_prices("AAPL", Some("2023-01-01"), Some("2023-12-31"))
    .await?;

// Intraday prices
let intraday_1min = client.charts()
    .get_1min_prices("AAPL", None, None)
    .await?;

let intraday_5min = client.charts()
    .get_5min_prices("AAPL", None, None)
    .await?;

// Custom timeframe
let custom = client.charts()
    .get_intraday_prices("AAPL", Timeframe::OneHour, None, None)
    .await?;
```

### Company Information

```rust
// Full company profile
let profile = client.company_info()
    .get_profile("AAPL")
    .await?;

// Key executives
let executives = client.company_info()
    .get_executives("AAPL")
    .await?;

// Market cap
let market_cap = client.company_info()
    .get_market_cap("AAPL")
    .await?;

// Share float
let float = client.company_info()
    .get_share_float("AAPL")
    .await?;

// Stock peers
let peers = client.company_info()
    .get_peers("AAPL")
    .await?;
```

### Custom Configuration

```rust
use std::time::Duration;

let client = FmpClient::builder()
    .api_key("your_api_key")
    .base_url("https://custom.api.endpoint")  // For testing
    .timeout(Duration::from_secs(60))
    .build()?;
```

## Testing

The library is designed to be testable without making actual API requests.

### Running Tests

```bash
# Run unit tests (no API key required)
cargo test

# Run integration tests (requires FMP_API_KEY)
cargo test -- --ignored

# Run specific test module
cargo test company_search
```

### Writing Tests with Mocks

The library is structured to allow easy mocking. You can override the base URL to point to a mock server:

```rust
#[cfg(test)]
mod tests {
    use fmp_rs::FmpClient;

    #[tokio::test]
    async fn test_with_mock_server() {
        // Set up your mock server (e.g., with wiremock or mockito)
        let mock_server = setup_mock_server();
        
        let client = FmpClient::builder()
            .api_key("test_key")
            .base_url(&mock_server.url())
            .build()
            .unwrap();
        
        // Test your code without hitting the real API
        let result = client.quote().get_quote("AAPL").await;
        assert!(result.is_ok());
    }
}
```

## API Coverage

This library provides comprehensive coverage of the FMP API:

### ✅ Implemented
- Company Search (search, CIK, CUSIP, ISIN)
- Stock Quotes (single, batch, exchange)
- Company Information (profile, executives, market cap, peers)
- Financial Statements (income, balance sheet, cash flow)
- Financial Ratios and Key Metrics
- Historical Price Charts (daily and intraday)

### 🚧 Coming Soon
- News and Press Releases
- Analyst Estimates and Ratings
- SEC Filings
- Insider Trading
- ETF and Mutual Fund Data
- Market Performance Indicators
- Technical Indicators
- Economics Data
- Institutional Ownership (Form 13F)
- And more...

## Error Handling

The library uses a custom `Error` type that covers common failure scenarios:

```rust
use fmp_rs::{FmpClient, Error};

match client.quote().get_quote("INVALID").await {
    Ok(quotes) => println!("Got quotes: {:?}", quotes),
    Err(Error::NotFound(msg)) => println!("Symbol not found: {}", msg),
    Err(Error::RateLimitExceeded) => println!("Rate limit hit, try again later"),
    Err(Error::Api { status, message }) => println!("API error {}: {}", status, message),
    Err(e) => println!("Other error: {}", e),
}
```

## Contributing

Contributions are welcome! The API is extensive, and help implementing additional endpoints would be greatly appreciated.

### Adding New Endpoints

1. Add the response model to `src/models/`
2. Implement the endpoint in the appropriate file under `src/endpoints/`
3. Add tests in the endpoint module
4. Update the README

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This library is not officially associated with or endorsed by Financial Modeling Prep. Use of the FMP API is subject to their terms of service and API usage limits.

## Links

- [FMP API Documentation](https://site.financialmodelingprep.com/developer/docs)
- [Crates.io](https://crates.io/crates/fmp-rs)
- [Documentation](https://docs.rs/fmp-rs)
