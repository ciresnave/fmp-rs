//! Commodities Markets Dashboard
//!
//! This example demonstrates how to use the FMP API to track commodity markets,
//! including gold, silver, crude oil, and natural gas prices with historical trends.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("         📊 COMMODITIES MARKETS DASHBOARD 📊");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Available Commodities Overview
    println!("📋 AVAILABLE COMMODITIES");
    println!("─────────────────────────────────────────────────────────");
    let commodities = client.commodities().get_commodity_list().await?;
    println!("Total available commodities: {}\n", commodities.len());

    // Show major commodities
    let major = ["GCUSD", "SIUSD", "CLUSD", "NGUSD"];
    let major_names = ["Gold", "Silver", "Crude Oil", "Natural Gas"];
    for (symbol, name) in major.iter().zip(major_names.iter()) {
        if let Some(c) = commodities.iter().find(|c| c.symbol == *symbol) {
            println!("  • {} ({})", name, c.symbol);
        }
    }
    println!();

    // 2. Real-Time Commodity Quotes
    println!("💰 CURRENT COMMODITY PRICES");
    println!("─────────────────────────────────────────────────────────");

    // Gold
    let gold = client.commodities().get_commodity_quote("GCUSD").await?;
    if let Some(g) = gold.first() {
        let price = g.price.unwrap_or(0.0);
        let change_pct = g.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  🥇 Gold (GCUSD)");
        println!("     Price: ${:.2}/oz", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (g.year_low, g.year_high) {
            println!("     52-Week Range: ${:.2} - ${:.2}", low, high);
        }
    }
    println!();

    // Silver
    let silver = client.commodities().get_commodity_quote("SIUSD").await?;
    if let Some(s) = silver.first() {
        let price = s.price.unwrap_or(0.0);
        let change_pct = s.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  🥈 Silver (SIUSD)");
        println!("     Price: ${:.2}/oz", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (s.year_low, s.year_high) {
            println!("     52-Week Range: ${:.2} - ${:.2}", low, high);
        }
    }
    println!();

    // Crude Oil
    let oil = client.commodities().get_commodity_quote("CLUSD").await?;
    if let Some(o) = oil.first() {
        let price = o.price.unwrap_or(0.0);
        let change_pct = o.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  🛢️  Crude Oil (CLUSD)");
        println!("     Price: ${:.2}/barrel", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (o.year_low, o.year_high) {
            println!("     52-Week Range: ${:.2} - ${:.2}", low, high);
        }
    }
    println!();

    // Natural Gas
    let gas = client.commodities().get_commodity_quote("NGUSD").await?;
    if let Some(ng) = gas.first() {
        let price = ng.price.unwrap_or(0.0);
        let change_pct = ng.changes_percentage.unwrap_or(0.0);
        let indicator = if change_pct > 0.0 {
            "🟢 ↗"
        } else {
            "🔴 ↘"
        };
        println!("  🔥 Natural Gas (NGUSD)");
        println!("     Price: ${:.2}/MMBtu", price);
        println!("     Change: {:+.2}% {}", change_pct, indicator);
        if let (Some(low), Some(high)) = (ng.year_low, ng.year_high) {
            println!("     52-Week Range: ${:.2} - ${:.2}", low, high);
        }
    }
    println!();

    // 3. Gold 30-Day Historical Trend
    println!("📈 GOLD 30-DAY PRICE TREND");
    println!("─────────────────────────────────────────────────────────");
    let gold_history = client
        .commodities()
        .get_commodity_historical("GCUSD", None, None)
        .await?;

    if gold_history.len() >= 30 {
        let recent_30 = &gold_history[..30];
        let oldest = recent_30.last().unwrap();
        let newest = recent_30.first().unwrap();
        let change = ((newest.close - oldest.close) / oldest.close) * 100.0;
        let trend = if change > 0.0 {
            "📈 Uptrend"
        } else {
            "📉 Downtrend"
        };

        println!("  30 Days Ago: ${:.2}/oz ({})", oldest.close, oldest.date);
        println!("  Today: ${:.2}/oz ({})", newest.close, newest.date);
        println!("  30-Day Change: {:+.2}% {}", change, trend);
        println!();

        // Show last 5 days
        println!("  Last 5 Trading Days:");
        for day in recent_30.iter().take(5) {
            let day_change = ((day.close - day.open) / day.open) * 100.0;
            let bar = if day_change > 0.0 {
                "█".repeat((day_change.abs() * 2.0).min(20.0) as usize)
            } else {
                "▓".repeat((day_change.abs() * 2.0).min(20.0) as usize)
            };
            println!(
                "    {}: ${:.2} {:+.2}% {}",
                day.date, day.close, day_change, bar
            );
        }
    }
    println!();

    // 4. Crude Oil Intraday Volatility
    println!("⚡ CRUDE OIL INTRADAY ACTIVITY (15-min intervals)");
    println!("─────────────────────────────────────────────────────────");
    let oil_intraday = client
        .commodities()
        .get_commodity_intraday("CLUSD", "15min")
        .await?;

    if !oil_intraday.is_empty() {
        println!(
            "  Latest {} data points (showing last 10)",
            oil_intraday.len()
        );
        println!();

        for tick in oil_intraday.iter().take(10) {
            let tick_change = ((tick.close - tick.open) / tick.open) * 100.0;
            let indicator = if tick_change > 0.0 { "🟢" } else { "🔴" };
            println!(
                "    {} | ${:.2} {:+.3}% {} | Vol: {:.0}",
                tick.date, tick.close, tick_change, indicator, tick.volume
            );
        }

        // Calculate intraday volatility
        if oil_intraday.len() >= 2 {
            let mut price_changes = Vec::new();
            for i in 0..(oil_intraday.len().min(20) - 1) {
                let pct_change = ((oil_intraday[i].close - oil_intraday[i + 1].close)
                    / oil_intraday[i + 1].close)
                    .abs()
                    * 100.0;
                price_changes.push(pct_change);
            }
            let avg_volatility: f64 =
                price_changes.iter().sum::<f64>() / price_changes.len() as f64;
            println!();
            println!("  Average 15-min volatility: {:.3}%", avg_volatility);
            let volatility_level = if avg_volatility > 0.5 {
                "High volatility - Active trading"
            } else if avg_volatility > 0.2 {
                "Moderate volatility - Normal trading"
            } else {
                "Low volatility - Quiet trading"
            };
            println!("  Status: {}", volatility_level);
        }
    }
    println!();

    // 5. Gold/Silver Ratio Analysis
    println!("⚖️  GOLD/SILVER RATIO ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    if let (Some(g), Some(s)) = (gold.first(), silver.first()) {
        if let (Some(gold_price), Some(silver_price)) = (g.price, s.price) {
            let ratio = gold_price / silver_price;
            println!("  Current Ratio: {:.2}:1", ratio);
            println!("  (How many ounces of silver to equal 1 oz gold)");
            println!();

            let analysis = if ratio > 80.0 {
                "  📊 High ratio - Silver historically undervalued vs gold"
            } else if ratio < 60.0 {
                "  📊 Low ratio - Gold historically undervalued vs silver"
            } else {
                "  📊 Normal range - Precious metals in typical relationship"
            };
            println!("{}", analysis);
            println!();
            println!("  Historical Context:");
            println!("    • Long-term average: ~60-70:1");
            println!("    • Current: {:.2}:1", ratio);
        }
    }
    println!();

    // 6. Market Insights
    println!("💡 COMMODITY MARKET INSIGHTS");
    println!("─────────────────────────────────────────────────────────");
    println!("  🥇 Precious Metals (Gold/Silver):");
    println!("     • Hedge against inflation and currency devaluation");
    println!("     • Safe haven during economic uncertainty");
    println!("     • Influenced by USD strength and real yields");
    println!();
    println!("  🛢️  Energy Commodities (Oil/Gas):");
    println!("     • Driven by global supply/demand dynamics");
    println!("     • OPEC+ production decisions impact prices");
    println!("     • Geopolitical events cause volatility");
    println!();
    println!("  📊 Trading Considerations:");
    println!("     • Commodities trade 23 hours/day (Sunday-Friday)");
    println!("     • Futures contracts available for all major commodities");
    println!("     • ETFs provide easier access (GLD, SLV, USO, UNG)");
    println!("     • Consider contango/backwardation in futures curves");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("Note: Commodity prices are influenced by supply/demand,");
    println!("geopolitics, currency movements, and global economic growth.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
