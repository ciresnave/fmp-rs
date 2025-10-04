//! Market Indexes Dashboard
//!
//! This example demonstrates tracking major market indexes including S&P 500,
//! Nasdaq, Dow Jones, and their constituent companies.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("          📊 MARKET INDEXES DASHBOARD 📊");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Available Indexes Overview
    println!("📋 AVAILABLE MARKET INDEXES");
    println!("─────────────────────────────────────────────────────────");
    let indexes = client.indexes().get_index_list().await?;
    println!("Total available indexes: {}\n", indexes.len());

    // Show major US indexes
    println!("  Major US Indexes:");
    let major_us = ["^GSPC", "^DJI", "^IXIC", "^RUT"];
    let major_names = ["S&P 500", "Dow Jones", "Nasdaq Composite", "Russell 2000"];
    for (symbol, name) in major_us.iter().zip(major_names.iter()) {
        if indexes.iter().any(|i| i.symbol == *symbol) {
            println!("    • {} ({})", name, symbol);
        }
    }
    println!();

    // 2. Real-Time Index Quotes
    println!("💹 CURRENT MARKET LEVELS");
    println!("─────────────────────────────────────────────────────────");

    // S&P 500
    let sp500 = client.indexes().get_index_quote("^GSPC").await?;
    if let Some(sp) = sp500.first() {
        let price = sp.price.unwrap_or(0.0);
        let change_pct = sp.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  📈 S&P 500 (^GSPC)");
        println!("     Level: {:.2}", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (sp.year_low, sp.year_high) {
            let ytd_range = ((price - low) / (high - low)) * 100.0;
            println!(
                "     52-Week Range: {:.2} - {:.2} (Currently at {:.0}%)",
                low, high, ytd_range
            );
        }
    }
    println!();

    // Nasdaq Composite
    let nasdaq = client.indexes().get_index_quote("^IXIC").await?;
    if let Some(nq) = nasdaq.first() {
        let price = nq.price.unwrap_or(0.0);
        let change_pct = nq.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  💻 Nasdaq Composite (^IXIC)");
        println!("     Level: {:.2}", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (nq.year_low, nq.year_high) {
            let ytd_range = ((price - low) / (high - low)) * 100.0;
            println!(
                "     52-Week Range: {:.2} - {:.2} (Currently at {:.0}%)",
                low, high, ytd_range
            );
        }
    }
    println!();

    // Dow Jones
    let dow = client.indexes().get_index_quote("^DJI").await?;
    if let Some(dj) = dow.first() {
        let price = dj.price.unwrap_or(0.0);
        let change_pct = dj.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  🏭 Dow Jones Industrial (^DJI)");
        println!("     Level: {:.2}", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (dj.year_low, dj.year_high) {
            let ytd_range = ((price - low) / (high - low)) * 100.0;
            println!(
                "     52-Week Range: {:.2} - {:.2} (Currently at {:.0}%)",
                low, high, ytd_range
            );
        }
    }
    println!();

    // Market Breadth Analysis
    println!("📊 MARKET BREADTH ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let sp_change = sp500
        .first()
        .and_then(|s| s.changes_percentage)
        .unwrap_or(0.0);
    let nq_change = nasdaq
        .first()
        .and_then(|n| n.changes_percentage)
        .unwrap_or(0.0);
    let dj_change = dow
        .first()
        .and_then(|d| d.changes_percentage)
        .unwrap_or(0.0);

    let positive_count = [sp_change > 0.0, nq_change > 0.0, dj_change > 0.0]
        .iter()
        .filter(|&&x| x)
        .count();

    let market_sentiment = match positive_count {
        3 => "🟢 Strong bullish - All major indexes up",
        2 => "🟡 Mixed positive - Majority indexes up",
        1 => "🟠 Mixed negative - Majority indexes down",
        _ => "🔴 Strong bearish - All major indexes down",
    };

    println!("  Market Sentiment: {}", market_sentiment);
    println!("  Indexes Positive: {}/3", positive_count);
    println!();

    // 3. S&P 500 30-Day Historical Performance
    println!("📈 S&P 500 30-DAY PERFORMANCE");
    println!("─────────────────────────────────────────────────────────");
    let sp_history = client
        .indexes()
        .get_index_historical("^GSPC", None, None)
        .await?;

    if sp_history.len() >= 30 {
        let recent_30 = &sp_history[..30];
        let oldest = recent_30.last().unwrap();
        let newest = recent_30.first().unwrap();
        let change = ((newest.close - oldest.close) / oldest.close) * 100.0;
        let trend = if change > 0.0 {
            "📈 Uptrend"
        } else {
            "📉 Downtrend"
        };

        println!("  30 Days Ago: {:.2} ({})", oldest.close, oldest.date);
        println!("  Today: {:.2} ({})", newest.close, newest.date);
        println!("  30-Day Return: {:+.2}% {}", change, trend);
        println!();

        // Show last 5 trading days
        println!("  Last 5 Trading Days:");
        for day in recent_30.iter().take(5) {
            let day_change = ((day.close - day.open) / day.open) * 100.0;
            let bar = if day_change > 0.0 {
                "█".repeat((day_change.abs() * 5.0).min(20.0) as usize)
            } else {
                "▓".repeat((day_change.abs() * 5.0).min(20.0) as usize)
            };
            println!(
                "    {}: {:.2} {:+.2}% {}",
                day.date, day.close, day_change, bar
            );
        }

        // Calculate volatility
        let mut daily_changes = Vec::new();
        for i in 0..recent_30.len() - 1 {
            let pct_change =
                ((recent_30[i].close - recent_30[i + 1].close) / recent_30[i + 1].close).abs()
                    * 100.0;
            daily_changes.push(pct_change);
        }
        let avg_volatility = daily_changes.iter().sum::<f64>() / daily_changes.len() as f64;
        println!();
        println!("  Average Daily Volatility: {:.2}%", avg_volatility);
        let vol_assessment = if avg_volatility > 2.0 {
            "High volatility - Turbulent market"
        } else if avg_volatility > 1.0 {
            "Moderate volatility - Normal market"
        } else {
            "Low volatility - Calm market"
        };
        println!("  Assessment: {}", vol_assessment);
    }
    println!();

    // 4. S&P 500 Constituents Analysis
    println!("🏢 S&P 500 CONSTITUENTS ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let sp500_constituents = client.indexes().get_index_constituents("^GSPC").await?;

    println!("  Total Companies: {}", sp500_constituents.len());
    println!();

    // Sector breakdown
    let mut sector_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for company in &sp500_constituents {
        if let Some(sector) = &company.sector {
            *sector_counts.entry(sector.clone()).or_insert(0) += 1;
        }
    }

    let mut sectors: Vec<_> = sector_counts.iter().collect();
    sectors.sort_by(|a, b| b.1.cmp(a.1));

    println!("  Sector Distribution:");
    for (sector, count) in sectors.iter().take(5) {
        let percentage = (**count as f64 / sp500_constituents.len() as f64) * 100.0;
        let bar = "█".repeat((percentage / 2.0) as usize);
        println!(
            "    {:25} {:3} ({:4.1}%) {}",
            sector, count, percentage, bar
        );
    }
    println!();

    // Show some notable companies
    println!("  Notable Companies (first 10 alphabetically):");
    let mut sorted_companies = sp500_constituents.clone();
    sorted_companies.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    for company in sorted_companies.iter().take(10) {
        println!(
            "    {} - {} ({})",
            company.symbol,
            company.name.as_deref().unwrap_or("N/A"),
            company.sector.as_deref().unwrap_or("N/A")
        );
    }
    println!();

    // 5. Dow Jones 30 Analysis
    println!("🏭 DOW JONES 30 INDUSTRIALS");
    println!("─────────────────────────────────────────────────────────");
    let dow_constituents = client.indexes().get_index_constituents("^DJI").await?;

    println!(
        "  The Dow Jones consists of {} blue-chip companies",
        dow_constituents.len()
    );
    println!();
    println!("  Components:");
    for (i, company) in dow_constituents.iter().enumerate() {
        if i < 15 {
            // Show first 15
            println!(
                "    {} - {}",
                company.symbol,
                company.name.as_deref().unwrap_or("N/A")
            );
        }
    }
    if dow_constituents.len() > 15 {
        println!("    ... and {} more", dow_constituents.len() - 15);
    }
    println!();

    // 6. Market Insights
    println!("💡 MARKET INDEX INSIGHTS");
    println!("─────────────────────────────────────────────────────────");
    println!("  📊 S&P 500 (^GSPC):");
    println!("     • Tracks 500 large-cap US companies");
    println!("     • Market-cap weighted index");
    println!("     • Best representation of US stock market");
    println!("     • Often used as benchmark for portfolio performance");
    println!();
    println!("  💻 Nasdaq Composite (^IXIC):");
    println!("     • Tracks all stocks on Nasdaq exchange (~3,000)");
    println!("     • Heavy technology sector weighting");
    println!("     • More volatile than S&P 500");
    println!("     • Includes growth stocks and small-caps");
    println!();
    println!("  🏭 Dow Jones Industrial (^DJI):");
    println!("     • Price-weighted index of 30 blue-chip companies");
    println!("     • Oldest US market index (since 1896)");
    println!("     • Less representative than S&P 500");
    println!("     • Easy to track and widely recognized");
    println!();
    println!("  🔄 Index Correlations:");
    println!("     • S&P 500 and Dow typically move together");
    println!("     • Nasdaq more sensitive to tech sector");
    println!("     • Divergence signals sector rotation");
    println!("     • All three down = broad market weakness");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("Note: Market indexes provide instant market health snapshot.");
    println!("Watch for divergences between indexes for sector insights.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
