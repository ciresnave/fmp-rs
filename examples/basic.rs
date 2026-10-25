//! Basic usage example
//!
//! To run: cargo run --example basic
//!
//! Make sure FMP_API_KEY environment variable is set

use fmp_rs::{models::common::Period, FmpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = FmpClient::new()?;

    println!("=== FMP API Basic Example ===\n");

    // 1. Get stock quote
    println!("1. Getting Apple stock quote...");
    let quotes = client.quote().get_quote("AAPL").await?;
    if let Some(quote) = quotes.first() {
        println!("   Symbol: {}", quote.symbol);
        println!("   Price: ${:.2}", quote.price);
        println!("   Change: {:.2}%", quote.changes_percentage);
        println!("   Volume: {}", quote.volume);
    }

    // 2. Get company profile
    println!("\n2. Getting company profile...");
    let profiles = client.company_info().get_profile("AAPL").await?;
    if let Some(profile) = profiles.first() {
        println!("   Company: {}", profile.company_name);
        println!("   CEO: {}", profile.ceo);
        println!("   Industry: {}", profile.industry);
        println!("   Sector: {}", profile.sector);
        println!("   Market Cap: ${}", profile.mkt_cap);
        println!("   Website: {}", profile.website);
    }

    // 3. Get income statements
    println!("\n3. Getting last 3 annual income statements...");
    let statements = client
        .financials()
        .get_income_statement("AAPL", Period::Annual, Some(3))
        .await?;

    for statement in statements {
        println!(
            "   Year {}: Revenue ${}, Net Income ${}",
            statement.calendar_year, statement.revenue, statement.net_income
        );
    }

    // 4. Search for companies
    println!("\n4. Searching for tech companies...");
    let results = client
        .company_search()
        .search("Microsoft", Some(5), None)
        .await?;

    for result in results {
        println!(
            "   {} - {} ({})",
            result.symbol, result.name, result.exchange_short_name
        );
    }

    // 5. Get batch quotes
    println!("\n5. Getting batch quotes for tech giants...");
    let batch = client
        .quote()
        .get_quote_batch(&["AAPL", "MSFT", "GOOGL", "AMZN", "META"])
        .await?;

    for quote in batch {
        println!(
            "   {}: ${:.2} ({:+.2}%)",
            quote.symbol, quote.price, quote.changes_percentage
        );
    }

    println!("\n✅ All examples completed successfully!");

    Ok(())
}
