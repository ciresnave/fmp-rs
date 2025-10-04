//! Comprehensive ETF Analysis Example
//!
//! Demonstrates ETF-related functionality:
//! - Searching and listing ETFs
//! - Getting ETF holdings (what the ETF holds)
//! - Getting ETF holders (who holds the ETF)
//! - Analyzing sector and country allocation
//! - Retrieving detailed ETF information
//!
//! To run: cargo run --example etf_analysis
//!
//! Make sure FMP_API_KEY environment variable is set

use fmp_rs::FmpClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = FmpClient::new()?;
    let symbol = "SPY"; // S&P 500 ETF

    println!("🔍 Comprehensive ETF Analysis for {}\n", symbol);
    println!("{}", "=".repeat(80));

    // 1. Search for ETFs
    println!("\n📋 ETF SEARCH");
    println!("{}", "-".repeat(80));

    let search_results = client.etf().search_etf("vanguard", Some(5), None).await?;
    println!("\nTop 5 Vanguard ETFs:");
    for etf in &search_results {
        println!(
            "  {} - {} ({})",
            etf.symbol,
            etf.name,
            etf.exchange_short_name.as_deref().unwrap_or("N/A")
        );
    }

    // 2. Get detailed ETF information
    println!("\n\n📊 ETF INFORMATION");
    println!("{}", "-".repeat(80));

    let info = client.etf().get_etf_info(symbol).await?;
    if let Some(etf) = info.first() {
        println!("\n{} - {}", etf.symbol, etf.company_name);
        println!("  Inception Date: {}", etf.inception_date);
        println!("  Domicile: {}", etf.domicile);
        println!("  ETF Company: {}", etf.etf_company);
        println!("  AUM: ${:.2}B", etf.aum / 1_000_000_000.0);
        println!("  NAV: ${:.2} {}", etf.nav, etf.nav_currency);
        println!("  Expense Ratio: {:.2}%", etf.expense_ratio);
        println!("  Holdings Count: {}", etf.holdings_count);
        println!("  Avg Volume: {}M shares", etf.avg_volume / 1_000_000);
        println!("  Website: {}", etf.website);
        println!("\nDescription:");
        println!("  {}", etf.description);
    }

    // 3. Get ETF holdings (what the ETF holds)
    println!("\n\n💼 ETF HOLDINGS (What {} Holds)", symbol);
    println!("{}", "-".repeat(80));

    let holdings = client.etf().get_etf_holdings(symbol).await?;
    println!("\nTop 15 Holdings:");
    for (i, holding) in holdings.iter().take(15).enumerate() {
        println!(
            "  {:2}. {} ({}) - {:.2}% | ${:.2}B",
            i + 1,
            holding.asset,
            holding.name,
            holding.weight_percentage,
            holding.market_value / 1_000_000_000.0
        );
    }

    let total_weight: f64 = holdings.iter().take(15).map(|h| h.weight_percentage).sum();
    println!(
        "\n  Top 15 holdings represent {:.2}% of the fund",
        total_weight
    );

    // 4. Get ETF holders (who holds this ETF)
    println!("\n\n🏛️ INSTITUTIONAL HOLDERS (Who Holds {})", symbol);
    println!("{}", "-".repeat(80));

    let holders = client.etf().get_etf_holder(symbol).await?;
    println!("\nTop 10 Institutional Holders:");
    for (i, holder) in holders.iter().take(10).enumerate() {
        if let Some(weight) = holder.weight_percentage {
            if let Some(shares) = holder.shares {
                println!(
                    "  {:2}. {} - {:.4}% ({} shares)",
                    i + 1,
                    holder.name,
                    weight,
                    shares
                );
            } else {
                println!("  {:2}. {} - {:.4}%", i + 1, holder.name, weight);
            }
        }
    }

    // 5. Sector allocation
    println!("\n\n🏭 SECTOR ALLOCATION");
    println!("{}", "-".repeat(80));

    let sectors = client.etf().get_etf_sector_weighting(symbol).await?;
    println!("\nSector Breakdown:");
    for sector in &sectors {
        let weight: f64 = sector.weight_percentage.parse().unwrap_or(0.0);
        let bar_length = (weight * 0.5) as usize; // Scale for display
        let bar = "█".repeat(bar_length);
        println!(
            "  {:30} {:>6}% {}",
            sector.sector, sector.weight_percentage, bar
        );
    }

    // 6. Geographic allocation
    println!("\n\n🌍 GEOGRAPHIC ALLOCATION");
    println!("{}", "-".repeat(80));

    let countries = client.etf().get_etf_country_weighting(symbol).await?;
    println!("\nCountry Breakdown:");
    for country in &countries {
        let weight: f64 = country.weight_percentage.parse().unwrap_or(0.0);
        let bar_length = (weight * 0.5) as usize; // Scale for display
        let bar = "█".repeat(bar_length);
        println!(
            "  {:30} {:>6}% {}",
            country.country, country.weight_percentage, bar
        );
    }

    // 7. List other popular ETFs
    println!("\n\n📈 OTHER POPULAR ETFS");
    println!("{}", "-".repeat(80));

    let popular_etfs = ["QQQ", "IWM", "DIA", "VTI", "AGG"];
    println!("\nQuick info on other major ETFs:");
    for etf_symbol in &popular_etfs {
        let info = client.etf().get_etf_info(etf_symbol).await?;
        if let Some(etf) = info.first() {
            println!(
                "  {} - {}: AUM ${:.2}B, Expense Ratio {:.2}%",
                etf.symbol,
                etf.company_name,
                etf.aum / 1_000_000_000.0,
                etf.expense_ratio
            );
        }
    }

    // Summary
    println!("\n\n{}", "=".repeat(80));
    println!("✅ ETF Analysis Complete!");
    println!("\nThis analysis covered:");
    println!("  • ETF search and discovery");
    println!("  • Detailed ETF information (AUM, expense ratio, etc.)");
    println!("  • Holdings breakdown (what the ETF owns)");
    println!("  • Institutional ownership (who owns the ETF)");
    println!("  • Sector and geographic allocation");
    println!("  • Comparison with other major ETFs");

    Ok(())
}
