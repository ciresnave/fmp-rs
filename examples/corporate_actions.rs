//! Corporate Actions Example
//!
//! Demonstrates how to track earnings calendars, IPOs, dividends,
//! stock splits, and economic events.
//!
//! Run with: cargo run --example corporate_actions

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("=== Corporate Actions Dashboard ===\n");

    // 1. Get upcoming earnings calendar
    println!("📅 Upcoming Earnings Calendar:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_earnings_calendar(None, None)
        .await
    {
        Ok(calendar) => {
            println!("Next 10 Earnings Reports:\n");
            for earning in calendar.iter().take(10) {
                println!(
                    "{} - {} ({})",
                    earning.date.as_deref().unwrap_or("N/A"),
                    earning.symbol,
                    earning.time.as_deref().unwrap_or("N/A")
                );

                if let (Some(eps_est), Some(revenue_est)) =
                    (earning.eps_estimated, earning.revenue_estimated)
                {
                    println!(
                        "  EPS Estimate: ${:.2} | Revenue Estimate: ${:.2}M",
                        eps_est,
                        revenue_est / 1_000_000.0
                    );
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching earnings calendar: {}", e),
    }

    // 2. Analyze historical earnings trends
    println!("\n📈 Historical Earnings Analysis for AAPL:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_historical_earnings("AAPL", Some(8))
        .await
    {
        Ok(history) => {
            println!("Last 8 Quarters:\n");
            for earning in history {
                let eps_beat = match (earning.eps, earning.eps_estimated) {
                    (Some(actual), Some(est)) if actual > est => "✓ Beat",
                    (Some(actual), Some(est)) if actual < est => "✗ Miss",
                    _ => "—",
                };

                let revenue_beat = match (earning.revenue, earning.revenue_estimated) {
                    (Some(actual), Some(est)) if actual > est => "✓ Beat",
                    (Some(actual), Some(est)) if actual < est => "✗ Miss",
                    _ => "—",
                };

                println!("{}", earning.date.as_deref().unwrap_or("N/A"));
                if let (Some(eps), Some(eps_est)) = (earning.eps, earning.eps_estimated) {
                    println!("  EPS: ${:.2} vs ${:.2} est. {}", eps, eps_est, eps_beat);
                }
                if let (Some(rev), Some(rev_est)) = (earning.revenue, earning.revenue_estimated) {
                    println!(
                        "  Revenue: ${:.2}B vs ${:.2}B est. {}",
                        rev / 1_000_000_000.0,
                        rev_est / 1_000_000_000.0,
                        revenue_beat
                    );
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching historical earnings: {}", e),
    }

    // 3. Track upcoming IPOs
    println!("\n🎯 Upcoming IPO Calendar:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_ipo_calendar(None, None)
        .await
    {
        Ok(ipos) => {
            if ipos.is_empty() {
                println!("No upcoming IPOs scheduled.");
            } else {
                for ipo in ipos.iter().take(10) {
                    println!(
                        "{} - {} ({})",
                        ipo.date.as_deref().unwrap_or("N/A"),
                        ipo.company.as_deref().unwrap_or("N/A"),
                        ipo.symbol
                    );
                    if let (Some(shares), Some(price)) = (ipo.number_of_shares, ipo.price.as_ref())
                    {
                        println!("  {} shares at {}", shares, price);
                        if let Some(exchange) = &ipo.exchange {
                            println!("  Exchange: {}", exchange);
                        }
                    }
                    println!();
                }
            }
        }
        Err(e) => eprintln!("Error fetching IPO calendar: {}", e),
    }

    // 4. Monitor stock splits
    println!("\n✂️ Stock Split Calendar:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_stock_split_calendar(None, None)
        .await
    {
        Ok(splits) => {
            if splits.is_empty() {
                println!("No recent or upcoming stock splits.");
            } else {
                for split in splits.iter().take(10) {
                    println!(
                        "{} - {} → {}",
                        split.date.as_deref().unwrap_or("N/A"),
                        split.symbol,
                        split.split.as_deref().unwrap_or("N/A")
                    );
                    if let Some(label) = &split.label {
                        println!("  {}", label);
                    }
                    println!();
                }
            }
        }
        Err(e) => eprintln!("Error fetching stock splits: {}", e),
    }

    // 5. Track dividend payments
    println!("\n💰 Dividend Calendar:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_dividend_calendar(None, None)
        .await
    {
        Ok(dividends) => {
            println!("Upcoming Dividend Payments:\n");

            // Group by symbol and sum dividends
            let mut dividend_map = std::collections::HashMap::new();
            for div in &dividends {
                let entry = dividend_map.entry(div.symbol.clone()).or_insert((0.0, 0));
                if let Some(amount) = div.dividend {
                    entry.0 += amount;
                    entry.1 += 1;
                }
            }

            // Show top dividend payers
            let mut sorted: Vec<_> = dividend_map.iter().collect();
            sorted.sort_by(|a, b| b.1.0.partial_cmp(&a.1.0).unwrap());

            for (symbol, (total, count)) in sorted.iter().take(10) {
                println!("{}: ${:.2} ({} payments)", symbol, total, count);
            }

            println!("\nRecent Dividends:");
            for div in dividends.iter().take(10) {
                println!(
                    "{} - {}: ${:.2} per share",
                    div.date.as_deref().unwrap_or("N/A"),
                    div.symbol,
                    div.dividend.unwrap_or(0.0)
                );
                if let Some(payment_date) = &div.payment_date {
                    println!("  Payment Date: {}", payment_date);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching dividend calendar: {}", e),
    }

    // 6. Monitor economic calendar
    println!("\n🌍 Economic Calendar:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_economic_calendar(None, None)
        .await
    {
        Ok(events) => {
            println!("Upcoming Economic Events:\n");

            // Group by country
            let mut by_country: std::collections::HashMap<String, Vec<_>> =
                std::collections::HashMap::new();

            for event in &events {
                if let Some(country) = &event.country {
                    by_country
                        .entry(country.clone())
                        .or_insert_with(Vec::new)
                        .push(event);
                }
            }

            for (country, country_events) in by_country.iter().take(5) {
                println!("📍 {}:", country);
                for event in country_events.iter().take(3) {
                    println!(
                        "  {} - {}",
                        event.date.as_deref().unwrap_or("N/A"),
                        event.event.as_deref().unwrap_or("N/A")
                    );
                    if let Some(impact) = &event.impact {
                        let emoji = match impact.as_str() {
                            "High" => "🔴",
                            "Medium" => "🟡",
                            "Low" => "🟢",
                            _ => "⚪",
                        };
                        println!("    {} Impact: {}", emoji, impact);
                    }
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching economic calendar: {}", e),
    }

    // 7. Check earnings confirmations
    println!("\n✅ Earnings Confirmations:");
    println!("{}", "=".repeat(60));
    match client
        .corporate_actions()
        .get_earnings_confirmations(None, None)
        .await
    {
        Ok(confirmations) => {
            println!("Recently Confirmed Earnings Dates:\n");
            for conf in confirmations.iter().take(10) {
                println!(
                    "{} - {} ({})",
                    conf.publication_date.as_deref().unwrap_or("N/A"),
                    conf.symbol,
                    conf.when.as_deref().unwrap_or("N/A")
                );
                if let Some(title) = &conf.title {
                    println!("  {}", title);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching earnings confirmations: {}", e),
    }

    println!("\n=== Dashboard Complete ===");
    Ok(())
}
