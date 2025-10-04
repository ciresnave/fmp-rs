//! Comprehensive example of market performance analysis
//!
//! Demonstrates using all market performance endpoints to:
//! - Analyze current sector performance
//! - Review historical sector trends
//! - Check market hours and status
//! - Identify top gainers, losers, and most active stocks
//! - Monitor pre/post-market trading activity

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the FMP client
    let client = FmpClient::new()?;

    println!("=== Market Performance Analysis ===\n");

    // 1. Current Sector Performance
    println!("📊 Current Sector Performance:");
    match client.market_performance().get_sector_performance().await {
        Ok(sectors) => {
            // Sort by performance
            let mut sectors = sectors;
            sectors.sort_by(|a, b| {
                let a_pct = a
                    .changes_percentage
                    .as_ref()
                    .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                    .unwrap_or(0.0);
                let b_pct = b
                    .changes_percentage
                    .as_ref()
                    .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                    .unwrap_or(0.0);
                b_pct.partial_cmp(&a_pct).unwrap()
            });

            for sector in sectors {
                let pct = sector.changes_percentage.unwrap_or_default();
                let icon = if pct.starts_with('-') { "🔴" } else { "🟢" };
                println!("  {} {}: {}", icon, sector.sector, pct);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 2. Historical Sector Trends
    println!("\n📈 Historical Sector Performance (Last 5 Days):");
    match client
        .market_performance()
        .get_sectors_historical(Some(5))
        .await
    {
        Ok(history) => {
            for record in &history {
                println!("\n  Date: {}", record.date);
                println!(
                    "    Technology: {:+.2}%",
                    record.technology_changes_percentage.unwrap_or(0.0)
                );
                println!(
                    "    Healthcare: {:+.2}%",
                    record.healthcare_changes_percentage.unwrap_or(0.0)
                );
                println!(
                    "    Financials: {:+.2}%",
                    record.financial_services_changes_percentage.unwrap_or(0.0)
                );
                println!(
                    "    Energy: {:+.2}%",
                    record.energy_changes_percentage.unwrap_or(0.0)
                );
            }

            // Calculate average performance
            if !history.is_empty() {
                let avg_tech: f64 = history
                    .iter()
                    .filter_map(|r| r.technology_changes_percentage)
                    .sum::<f64>()
                    / history.len() as f64;
                let avg_health: f64 = history
                    .iter()
                    .filter_map(|r| r.healthcare_changes_percentage)
                    .sum::<f64>()
                    / history.len() as f64;

                println!("\n  5-Day Averages:");
                println!("    Technology: {:+.2}%", avg_tech);
                println!("    Healthcare: {:+.2}%", avg_health);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 3. Market Hours and Status
    println!("\n🕐 Market Hours:");
    match client.market_performance().get_market_hours().await {
        Ok(markets) => {
            for market in markets.iter().take(3) {
                if let Some(name) = &market.stock_exchange_name {
                    println!("\n  Exchange: {}", name);
                    println!(
                        "    Is Open: {}",
                        if market.is_the_stock_market_open.unwrap_or(false) {
                            "✅ Yes"
                        } else {
                            "❌ No"
                        }
                    );

                    if let Some(hours) = &market.stock_market_hours {
                        if let (Some(open), Some(close)) = (&hours.open_time, &hours.close_time) {
                            println!("    Hours: {} - {}", open, close);
                        }
                    }

                    if market.is_the_stock_market_holiday.unwrap_or(false) {
                        println!("    Status: 🏖️  Holiday");
                    }

                    if !market.stock_market_holidays.is_empty() {
                        println!("    Upcoming Holidays:");
                        for holiday in market.stock_market_holidays.iter().take(3) {
                            if let (Some(name), Some(date)) = (&holiday.holiday, &holiday.date) {
                                println!("      - {}: {}", date, name);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 4. Top Gainers
    println!("\n🚀 Top 10 Gainers:");
    match client.market_performance().get_gainers().await {
        Ok(gainers) => {
            for (i, stock) in gainers.iter().take(10).enumerate() {
                println!(
                    "  {}. {} ({}): +{:.2}% at ${:.2} | Vol: {}M",
                    i + 1,
                    stock.name.as_deref().unwrap_or("N/A"),
                    stock.symbol,
                    stock.changes_percentage.unwrap_or(0.0),
                    stock.price.unwrap_or(0.0),
                    stock.volume.unwrap_or(0) / 1_000_000
                );
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 5. Top Losers
    println!("\n📉 Top 10 Losers:");
    match client.market_performance().get_losers().await {
        Ok(losers) => {
            for (i, stock) in losers.iter().take(10).enumerate() {
                println!(
                    "  {}. {} ({}): {:.2}% at ${:.2} | Vol: {}M",
                    i + 1,
                    stock.name.as_deref().unwrap_or("N/A"),
                    stock.symbol,
                    stock.changes_percentage.unwrap_or(0.0),
                    stock.price.unwrap_or(0.0),
                    stock.volume.unwrap_or(0) / 1_000_000
                );
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 6. Most Active Stocks
    println!("\n🔥 Most Active Stocks (by Volume):");
    match client.market_performance().get_most_active().await {
        Ok(active) => {
            for (i, stock) in active.iter().take(10).enumerate() {
                println!(
                    "  {}. {} ({}): {}M shares at ${:.2} ({:+.2}%)",
                    i + 1,
                    stock.name.as_deref().unwrap_or("N/A"),
                    stock.symbol,
                    stock.volume.unwrap_or(0) / 1_000_000,
                    stock.price.unwrap_or(0.0),
                    stock.changes_percentage.unwrap_or(0.0)
                );
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 7. Pre/Post Market Activity
    println!("\n🌅 Pre-Market & After-Hours Trading (Top 10):");
    match client.market_performance().get_pre_post_market().await {
        Ok(extended) => {
            for (i, stock) in extended.iter().take(10).enumerate() {
                println!("\n  {}. {}", i + 1, stock.symbol);

                if let (Some(pre_price), Some(pre_change)) =
                    (stock.pre_market_price, stock.pre_market_change_percentage)
                {
                    let icon = if pre_change >= 0.0 { "🟢" } else { "🔴" };
                    println!(
                        "     Pre-Market: {} ${:.2} ({:+.2}%)",
                        icon, pre_price, pre_change
                    );
                }

                if let (Some(after_price), Some(after_change)) =
                    (stock.after_hours_price, stock.after_hours_change_percentage)
                {
                    let icon = if after_change >= 0.0 { "🟢" } else { "🔴" };
                    println!(
                        "     After-Hours: {} ${:.2} ({:+.2}%)",
                        icon, after_price, after_change
                    );
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 8. Market Summary
    println!("\n📊 Market Summary:");

    // Combine insights
    if let (Ok(gainers), Ok(losers)) = (
        client.market_performance().get_gainers().await,
        client.market_performance().get_losers().await,
    ) {
        let gainer_count = gainers.len();
        let loser_count = losers.len();

        println!("  Advancing: {} stocks", gainer_count);
        println!("  Declining: {} stocks", loser_count);

        if gainer_count > loser_count {
            println!("  Market Sentiment: 🟢 Bullish");
        } else if loser_count > gainer_count {
            println!("  Market Sentiment: 🔴 Bearish");
        } else {
            println!("  Market Sentiment: ⚪ Neutral");
        }
    }

    println!("\n=== Analysis Complete ===");

    Ok(())
}
