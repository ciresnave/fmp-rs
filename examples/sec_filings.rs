//! SEC Filings Example
//!
//! Demonstrates how to analyze SEC filings, Form 13F institutional holdings,
//! Form 4 insider transactions, and CIK lookups.
//!
//! Run with: cargo run --example sec_filings

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("=== SEC Filings Analysis ===\n");

    // 1. Get recent SEC filings for a company
    println!("📄 Recent SEC Filings for AAPL:");
    println!("{}", "=".repeat(60));
    match client.sec_filings().get_sec_filings("AAPL", Some(5)).await {
        Ok(filings) => {
            for filing in filings {
                println!(
                    "{} - {} ({})",
                    filing.filing_date.as_deref().unwrap_or("N/A"),
                    filing.filing_type.as_deref().unwrap_or("N/A"),
                    filing.symbol
                );
                if let Some(url) = filing.link {
                    println!("  📎 {}", url);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching SEC filings: {}", e),
    }

    // 2. Get RSS feed for recent filings
    println!("\n📡 Recent SEC Filings RSS Feed (Latest 10):");
    println!("{}", "=".repeat(60));
    match client.sec_filings().get_sec_rss_feed(Some(10)).await {
        Ok(rss_items) => {
            for item in rss_items.iter().take(10) {
                println!(
                    "{}: {} - {}",
                    item.date.as_deref().unwrap_or("N/A"),
                    item.cik.as_deref().unwrap_or("N/A"),
                    item.title.as_deref().unwrap_or("Untitled")
                );
            }
        }
        Err(e) => eprintln!("Error fetching RSS feed: {}", e),
    }

    // 3. Analyze Form 13F institutional holdings
    println!("\n🏦 Form 13F Institutional Holdings for Berkshire Hathaway:");
    println!("{}", "=".repeat(60));
    match client
        .sec_filings()
        .get_form_13f("BRK-B", Some("2024-03-31"))
        .await
    {
        Ok(holdings) => {
            let total_value: f64 = holdings.iter().filter_map(|h| h.market_value).sum();

            println!(
                "Total Portfolio Value: ${:.2}B\n",
                total_value / 1_000_000_000.0
            );
            println!("Top 10 Holdings:");
            for (i, holding) in holdings.iter().take(10).enumerate() {
                println!(
                    "{}. {} ({})",
                    i + 1,
                    holding.name_of_issuer.as_deref().unwrap_or("N/A"),
                    holding.cusip.as_deref().unwrap_or("N/A")
                );
                println!(
                    "   Shares: {:>15}  Value: ${:>12.2}M",
                    holding.shares.unwrap_or(0).to_string(),
                    holding.market_value.unwrap_or(0.0) / 1_000_000.0
                );
            }
        }
        Err(e) => eprintln!("Error fetching Form 13F: {}", e),
    }

    // 4. Check Form 4 insider transactions
    println!("\n👤 Form 4 Insider Transactions for AAPL:");
    println!("{}", "=".repeat(60));
    match client.sec_filings().get_form_4("AAPL", Some(10)).await {
        Ok(transactions) => {
            for txn in transactions {
                let transaction_type = txn.transaction_type.as_deref().unwrap_or("Unknown");

                println!(
                    "{} - {}",
                    txn.transaction_date.as_deref().unwrap_or("N/A"),
                    txn.reporting_name.as_deref().unwrap_or("N/A")
                );
                println!(
                    "  {} {} shares at ${:.2}",
                    transaction_type,
                    txn.securities_transacted.unwrap_or(0.0),
                    txn.price.unwrap_or(0.0)
                );
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching Form 4: {}", e),
    }

    // 5. Get insider ownership summary
    println!("\n📊 Insider Ownership for TSLA:");
    println!("{}", "=".repeat(60));
    match client
        .sec_filings()
        .get_insider_ownership("TSLA", Some(10))
        .await
    {
        Ok(ownership) => {
            for insider in ownership.iter().take(10) {
                println!(
                    "{} - {}",
                    insider.reporting_name.as_deref().unwrap_or("N/A"),
                    insider.transaction_type.as_deref().unwrap_or("N/A")
                );
                if let Some(shares) = insider.securities_owned {
                    println!("  Securities Owned: {}", shares);
                }
                if let Some(transacted) = insider.securities_transacted {
                    println!("  Securities Transacted: {}", transacted);
                    if let Some(price) = insider.price {
                        println!("  Price: ${:.2}", price);
                    }
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching insider ownership: {}", e),
    }

    // 6. Search for company by CIK
    println!("\n🔍 CIK Search Example:");
    println!("{}", "=".repeat(60));
    match client.sec_filings().search_cik("Apple Inc").await {
        Ok(results) => {
            for result in results.iter().take(5) {
                println!(
                    "{} - CIK: {}",
                    result.name.as_deref().unwrap_or("N/A"),
                    &result.cik
                );
            }
        }
        Err(e) => eprintln!("Error searching CIK: {}", e),
    }

    // 7. Get company name by CIK
    println!("\n📛 Company Name Lookup by CIK:");
    println!("{}", "=".repeat(60));
    match client
        .sec_filings()
        .get_company_name_by_cik("0000320193")
        .await
    {
        Ok(companies) => {
            for company in companies {
                println!("CIK: {} - {}", &company.cik, &company.name);
            }
        }
        Err(e) => eprintln!("Error fetching company name: {}", e),
    }

    // 8. Map CUSIP to symbol
    println!("\n🔗 CUSIP to Symbol Mapping:");
    println!("{}", "=".repeat(60));
    match client.sec_filings().get_cusip_mapper("037833100").await {
        Ok(mappings) => {
            for mapping in mappings {
                println!(
                    "CIK: {} - {} (Company)",
                    &mapping.cik,
                    mapping.name.as_deref().unwrap_or("N/A")
                );
            }
        }
        Err(e) => eprintln!("Error mapping CUSIP: {}", e),
    }

    println!("\n=== Analysis Complete ===");
    Ok(())
}
