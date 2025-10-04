//! Congressional and Insider Trading Analysis Dashboard
//!
//! This example demonstrates comprehensive analysis capabilities for:
//! - Congressional stock trading transparency
//! - Corporate insider trading patterns
//! - Political trading impact analysis
//! - Investment sentiment indicators

use fmp_rs::{FmpClient, error::Result};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let client = FmpClient::builder().api_key("your_api_key_here").build()?;

    println!("🏛️  CONGRESSIONAL & INSIDER TRADING ANALYSIS DASHBOARD");
    println!("═══════════════════════════════════════════════════════");

    // Analyze congressional trading patterns
    analyze_congressional_trading(&client).await?;

    println!();

    // Analyze insider trading for major stocks
    analyze_insider_trading(&client).await?;

    println!();

    // Compare congressional vs insider sentiment
    analyze_political_vs_corporate_sentiment(&client).await?;

    Ok(())
}

/// Analyze congressional trading patterns and transparency
async fn analyze_congressional_trading(client: &FmpClient) -> Result<()> {
    println!("📊 CONGRESSIONAL TRADING ANALYSIS");
    println!("──────────────────────────────────");

    // Get all congressional trades
    let congress = client.congress();
    let trades = congress.get_congressional_trades(None).await?;
    println!("📈 Found {} congressional trades", trades.len());

    if !trades.is_empty() {
        // Analyze party trading patterns
        let mut party_stats: HashMap<String, i32> = HashMap::new();
        let mut state_stats: HashMap<String, i32> = HashMap::new();
        let mut stock_popularity: HashMap<String, i32> = HashMap::new();

        for trade in &trades[..20.min(trades.len())] {
            if let Some(party) = &trade.party {
                *party_stats.entry(party.clone()).or_insert(0) += 1;
            }

            if let Some(state) = &trade.state {
                *state_stats.entry(state.clone()).or_insert(0) += 1;
            }

            if let Some(ticker) = &trade.ticker {
                *stock_popularity.entry(ticker.clone()).or_insert(0) += 1;
            }

            // Display individual trade details
            let rep_name = trade.representative.as_deref().unwrap_or("Unknown");
            let stock = trade.ticker.as_deref().unwrap_or("N/A");
            let action = trade.transaction.as_deref().unwrap_or("N/A");
            let amount = trade.amount.as_deref().unwrap_or("N/A");
            let party = trade.party.as_deref().unwrap_or("N/A");
            let date = trade.transaction_date.as_deref().unwrap_or("N/A");

            println!(
                "  💼 {} ({}) {} {} on {} - {}",
                rep_name, party, action, stock, date, amount
            );
        }

        println!("\n🏛️ PARTY BREAKDOWN:");
        for (party, count) in party_stats {
            println!("  {} {} trades", party, count);
        }

        println!("\n📍 TOP TRADING STATES:");
        let mut sorted_states: Vec<_> = state_stats.iter().collect();
        sorted_states.sort_by(|a, b| b.1.cmp(a.1));
        for (state, count) in sorted_states.iter().take(5) {
            println!("  {} {} trades", state, count);
        }

        println!("\n🎯 MOST POPULAR STOCKS:");
        let mut sorted_stocks: Vec<_> = stock_popularity.iter().collect();
        sorted_stocks.sort_by(|a, b| b.1.cmp(a.1));
        for (stock, count) in sorted_stocks.iter().take(5) {
            println!("  {} {} trades", stock, count);
        }
    }

    // Get congressional summary for transparency analysis
    let summaries = congress.get_congressional_summary(None).await?;
    if !summaries.is_empty() {
        println!("\n📋 DISCLOSURE SUMMARY:");

        for summary in summaries.iter().take(5) {
            let rep = summary.representative.as_deref().unwrap_or("Unknown");
            let trades = summary.total_trades.unwrap_or(0);
            let value = summary.total_value.as_deref().unwrap_or("N/A");
            let party = summary.party.as_deref().unwrap_or("N/A");

            println!(
                "  🏛️ {} ({}) - {} trades, Total: {}",
                rep, party, trades, value
            );
        }
    }

    println!("\n💡 CONGRESSIONAL TRADING INSIGHTS:");
    println!("  • Congressional trading provides transparency into political stock positions");
    println!("  • Disclosure requirements help track potential conflicts of interest");
    println!("  • Popular stocks among politicians may indicate policy impact expectations");
    println!(
        "  • Party differences in trading patterns can reveal ideological investment preferences"
    );

    Ok(())
}

/// Analyze insider trading patterns for major companies
async fn analyze_insider_trading(client: &FmpClient) -> Result<()> {
    println!("🏢 INSIDER TRADING ANALYSIS");
    println!("───────────────────────────");

    let stocks = vec!["AAPL", "MSFT", "GOOGL", "TSLA", "NVDA"];

    for symbol in &stocks {
        println!("\n📊 {} INSIDER ACTIVITY:", symbol);

        // Get insider trades
        let congress = client.congress();
        let trades = congress.get_insider_trades(symbol, Some(0)).await?;

        if !trades.is_empty() {
            let mut buy_count = 0;
            let mut sell_count = 0;
            let mut total_buy_value = 0.0;
            let mut total_sell_value = 0.0;

            for trade in &trades[..10.min(trades.len())] {
                let insider = trade.reporting_name.as_deref().unwrap_or("Unknown Insider");
                let title = trade.title.as_deref().unwrap_or("N/A");
                let transaction_type = trade.transaction_type.as_deref().unwrap_or("N/A");
                let shares = trade.securities_transacted.unwrap_or(0.0);
                let price = trade.price.unwrap_or(0.0);
                let value = trade.transaction_value.unwrap_or(0.0);
                let date = trade.transaction_date.as_deref().unwrap_or("N/A");

                let action = match transaction_type {
                    "P" => {
                        buy_count += 1;
                        total_buy_value += value;
                        "PURCHASE"
                    }
                    "S" => {
                        sell_count += 1;
                        total_sell_value += value;
                        "SALE"
                    }
                    "A" => "AWARD",
                    "D" => "DISPOSITION",
                    _ => "OTHER",
                };

                let value_display = if value > 0.0 {
                    format!("${:.0}", value)
                } else {
                    format!("{:.0} shares @ ${:.2}", shares, price)
                };

                println!("  👤 {} ({})", insider, title);
                println!("     {} {} on {} - {}", action, symbol, date, value_display);
            }

            println!("\n  📈 INSIDER SENTIMENT ANALYSIS:");
            println!("     Purchases: {} (${:.0})", buy_count, total_buy_value);
            println!("     Sales: {} (${:.0})", sell_count, total_sell_value);

            let net_sentiment = total_buy_value - total_sell_value;
            let sentiment_indicator = if net_sentiment > 0.0 {
                "🟢 BULLISH"
            } else if net_sentiment < 0.0 {
                "🔴 BEARISH"
            } else {
                "🟡 NEUTRAL"
            };

            println!(
                "     Net Activity: ${:.0} - {}",
                net_sentiment, sentiment_indicator
            );
        }

        // Get insider statistics for deeper analysis
        let stats = congress.get_insider_stats(symbol).await?;
        if !stats.is_empty() {
            for stat in stats.iter().take(1) {
                if let (Some(buys), Some(sells)) = (stat.total_purchases, stat.total_sales) {
                    let buy_sell_ratio = if sells > 0 {
                        buys as f64 / sells as f64
                    } else {
                        buys as f64
                    };
                    println!("     Buy/Sell Ratio: {:.2}", buy_sell_ratio);
                }

                if let Some(sentiment) = &stat.insider_sentiment {
                    println!("     Overall Sentiment: {}", sentiment);
                }
            }
        }

        // Get insider ownership for context
        let ownership = congress.get_insider_ownership(symbol).await?;
        if !ownership.is_empty() {
            for owner in ownership.iter().take(1) {
                if let Some(ownership_pct) = owner.insider_ownership_percent {
                    println!("     Insider Ownership: {:.2}%", ownership_pct);
                }

                if let (Some(buying), Some(selling)) =
                    (owner.recent_insider_buying, owner.recent_insider_selling)
                {
                    println!("     Recent Activity: {} buys, {} sells", buying, selling);
                }
            }
        }
    }

    println!("\n💡 INSIDER TRADING INSIGHTS:");
    println!("  • Insider buying often indicates confidence in company prospects");
    println!("  • Heavy insider selling may signal overvaluation or personal liquidity needs");
    println!("  • CEO/CFO transactions carry more weight than routine option exercises");
    println!("  • Insider ownership percentage shows management alignment with shareholders");

    Ok(())
}

/// Compare political vs corporate sentiment for investment analysis
async fn analyze_political_vs_corporate_sentiment(client: &FmpClient) -> Result<()> {
    println!("🎯 POLITICAL vs CORPORATE SENTIMENT ANALYSIS");
    println!("─────────────────────────────────────────────");

    let target_stocks = vec!["AAPL", "MSFT", "TSLA"];

    for symbol in &target_stocks {
        println!("\n📊 {} CROSS-ANALYSIS:", symbol);

        // Get congressional activity for this stock
        let congress = client.congress();
        let congress_trades = congress.get_congressional_trades(Some(symbol)).await?;
        let mut congress_buys = 0;
        let mut congress_sells = 0;

        for trade in &congress_trades {
            match trade.transaction.as_deref() {
                Some("Purchase") | Some("Buy") => congress_buys += 1,
                Some("Sale") | Some("Sell") => congress_sells += 1,
                _ => {}
            }
        }

        // Get insider activity for comparison
        let insider_trades = congress.get_insider_trades(symbol, Some(0)).await?;
        let mut insider_buys = 0;
        let mut insider_sells = 0;

        for trade in &insider_trades {
            match trade.transaction_type.as_deref() {
                Some("P") => insider_buys += 1,
                Some("S") => insider_sells += 1,
                _ => {}
            }
        }

        println!("  🏛️ CONGRESSIONAL ACTIVITY:");
        println!(
            "     Purchases: {}, Sales: {}",
            congress_buys, congress_sells
        );

        let congress_sentiment = if congress_buys > congress_sells {
            "🟢 BULLISH"
        } else if congress_sells > congress_buys {
            "🔴 BEARISH"
        } else {
            "🟡 NEUTRAL"
        };

        println!("     Congressional Sentiment: {}", congress_sentiment);

        println!("  🏢 INSIDER ACTIVITY:");
        println!("     Purchases: {}, Sales: {}", insider_buys, insider_sells);

        let insider_sentiment = if insider_buys > insider_sells {
            "🟢 BULLISH"
        } else if insider_sells > insider_buys {
            "🔴 BEARISH"
        } else {
            "🟡 NEUTRAL"
        };

        println!("     Insider Sentiment: {}", insider_sentiment);

        // Alignment analysis
        let alignment = match (
            congress_sentiment.contains("BULLISH"),
            insider_sentiment.contains("BULLISH"),
        ) {
            (true, true) => "🟢 STRONG BULLISH ALIGNMENT",
            (false, false) => "🔴 BEARISH ALIGNMENT",
            _ => "⚠️ MIXED SIGNALS",
        };

        println!("  🎯 SENTIMENT ALIGNMENT: {}", alignment);
    }

    println!("\n💡 STRATEGIC INVESTMENT INSIGHTS:");
    println!(
        "  • Aligned bullish sentiment (political + corporate) may indicate strong opportunities"
    );
    println!("  • Mixed signals warrant deeper fundamental analysis");
    println!("  • Political trading ahead of policy announcements can provide early indicators");
    println!("  • Insider selling with political buying might suggest different time horizons");
    println!("  • Consider regulatory environment when politicians are active in sector stocks");

    Ok(())
}
