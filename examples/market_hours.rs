//! Market Hours and Trading Sessions Dashboard
//!
//! This example demonstrates market hours analysis, trading session monitoring,
//! and market holiday tracking across global exchanges.

use chrono::Datelike;
use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("          🕒 MARKET HOURS & TRADING SESSIONS 🕒");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Current Market Status
    println!("🚦 CURRENT MARKET STATUS");
    println!("─────────────────────────────────────────────────────────");

    let market_status = client.market_hours().get_market_hours().await?;
    if !market_status.is_empty() {
        println!("  Real-time Market Status:");
        println!();

        for market in &market_status {
            let market_name = market
                .stock_market
                .as_deref()
                .or(market.exchange.as_deref())
                .unwrap_or("Unknown Market");

            let status = if market.is_market_open.unwrap_or(false) {
                "🟢 OPEN"
            } else {
                "🔴 CLOSED"
            };

            let hours =
                if let (Some(open), Some(close)) = (&market.opening_hour, &market.closing_hour) {
                    format!("({} - {})", open, close)
                } else {
                    "Hours N/A".to_string()
                };

            let timezone = market.timezone.as_deref().unwrap_or("UTC");
            let current_time = market.current_date_time.as_deref().unwrap_or("N/A");

            println!("    📊 {} {}", market_name, status);
            println!("       Trading Hours: {}", hours);
            println!(
                "       Timezone: {} | Current Time: {}",
                timezone, current_time
            );
            println!();
        }
    } else {
        println!("    Market status data currently unavailable");
    }

    // 2. Extended Trading Hours Analysis
    println!("⏰ EXTENDED TRADING HOURS");
    println!("─────────────────────────────────────────────────────────");

    let extended_hours = client.market_hours().get_extended_hours().await?;
    if !extended_hours.is_empty() {
        println!("  Complete Trading Session Schedule:");
        println!();

        for exchange in extended_hours.iter().take(8) {
            let exchange_name = exchange
                .name
                .as_deref()
                .or(exchange.exchange.as_deref())
                .unwrap_or("Unknown Exchange");

            let is_open = exchange.is_open.unwrap_or(false);
            let status_emoji = if is_open { "🟢" } else { "🔴" };

            println!(
                "    {} {} {}",
                status_emoji,
                exchange_name,
                if is_open { "ACTIVE" } else { "CLOSED" }
            );

            // Regular session
            if let (Some(open), Some(close)) = (&exchange.market_open, &exchange.market_close) {
                println!("      📈 Regular Session: {} - {}", open, close);
            }

            // Pre-market
            if let (Some(pre_open), Some(pre_close)) =
                (&exchange.pre_market_open, &exchange.pre_market_close)
            {
                println!("      🌅 Pre-Market:      {} - {}", pre_open, pre_close);
            }

            // After-hours
            if let (Some(after_open), Some(after_close)) =
                (&exchange.after_hours_open, &exchange.after_hours_close)
            {
                println!("      🌙 After-Hours:     {} - {}", after_open, after_close);
            }

            if let Some(timezone) = &exchange.timezone {
                println!("      🌍 Timezone: {}", timezone);
            }

            println!();
        }
    }

    // 3. Global Exchange Hours
    println!("🌍 GLOBAL EXCHANGE HOURS");
    println!("─────────────────────────────────────────────────────────");

    let exchange_hours = client.market_hours().get_exchange_hours().await?;
    if !exchange_hours.is_empty() {
        println!("  Worldwide Trading Hours:");
        println!();

        // Group by region/country for better organization
        let mut exchanges_by_region: std::collections::HashMap<String, Vec<_>> =
            std::collections::HashMap::new();

        for exchange in &exchange_hours {
            let country = exchange.country.as_deref().unwrap_or("Unknown").to_string();
            exchanges_by_region
                .entry(country)
                .or_insert_with(Vec::new)
                .push(exchange);
        }

        // Display by region
        let regions = [
            "United States",
            "United Kingdom",
            "Germany",
            "Japan",
            "Hong Kong",
            "Australia",
        ];

        for region in &regions {
            if let Some(regional_exchanges) = exchanges_by_region.get(*region) {
                println!("    🏴 {}:", region);

                for exchange in regional_exchanges.iter().take(3) {
                    let exchange_name = exchange
                        .exchange_name
                        .as_deref()
                        .or(exchange.exchange.as_deref())
                        .unwrap_or("Unknown");

                    let status = if exchange.is_open.unwrap_or(false) {
                        "🟢 OPEN"
                    } else {
                        "🔴 CLOSED"
                    };

                    let trading_hours = if let (Some(open), Some(close)) =
                        (&exchange.market_open, &exchange.market_close)
                    {
                        format!("{} - {}", open, close)
                    } else {
                        "Hours N/A".to_string()
                    };

                    let timezone = exchange.timezone.as_deref().unwrap_or("UTC");
                    let day = exchange.day_of_week.as_deref().unwrap_or("Unknown");

                    println!(
                        "      {} {} | {} ({}) | {}",
                        exchange_name, status, trading_hours, timezone, day
                    );
                }
                println!();
            }
        }
    }

    // 4. Market Holidays Analysis
    println!("🎄 MARKET HOLIDAYS & CLOSURES");
    println!("─────────────────────────────────────────────────────────");

    // Get holidays for current year
    let current_year = chrono::Utc::now().year();
    let holidays = client
        .market_hours()
        .get_market_holidays(Some(current_year))
        .await?;

    if !holidays.is_empty() {
        println!("  {} Market Holidays & Special Hours:", current_year);
        println!();

        // Group holidays by month for better organization
        let mut holidays_by_month: std::collections::HashMap<String, Vec<_>> =
            std::collections::HashMap::new();

        for holiday in &holidays {
            if let Some(date) = &holiday.date {
                let month =
                    if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                        format!(
                            "{:02} - {}",
                            parsed_date.month(),
                            chrono::Month::try_from(parsed_date.month() as u8)
                                .unwrap_or(chrono::Month::January)
                                .name()
                        )
                    } else {
                        "Unknown".to_string()
                    };
                holidays_by_month
                    .entry(month)
                    .or_insert_with(Vec::new)
                    .push(holiday);
            }
        }

        // Sort and display holidays by month
        let mut sorted_months: Vec<_> = holidays_by_month.keys().collect();
        sorted_months.sort();

        for month in sorted_months.iter().take(12) {
            if let Some(month_holidays) = holidays_by_month.get(*month) {
                println!("    📅 {}:", month);

                for holiday in month_holidays.iter().take(5) {
                    let date = holiday.date.as_deref().unwrap_or("N/A");
                    let name = holiday.holiday.as_deref().unwrap_or("Unknown Holiday");
                    let status = holiday.market_status.as_deref().unwrap_or("closed");
                    let exchange = holiday.exchange.as_deref().unwrap_or("All Markets");

                    let status_emoji = match status {
                        "closed" => "🔒 CLOSED",
                        "early_close" => "⏰ EARLY CLOSE",
                        _ => "ℹ️  SPECIAL",
                    };

                    println!("      {} | {} {} | {}", date, status_emoji, name, exchange);

                    if let Some(early_close) = &holiday.early_close_time {
                        println!("        Early close at: {}", early_close);
                    }
                }
                println!();
            }
        }
    } else {
        println!("    No holiday data available for {}", current_year);
        println!();
    }

    // Get next year's holidays for planning
    let next_year = current_year + 1;
    let next_year_holidays = client
        .market_hours()
        .get_market_holidays(Some(next_year))
        .await?;

    if !next_year_holidays.is_empty() {
        println!("  🔮 Upcoming {} Holidays (Planning Ahead):", next_year);
        println!();

        for holiday in next_year_holidays.iter().take(8) {
            let date = holiday.date.as_deref().unwrap_or("N/A");
            let name = holiday.holiday.as_deref().unwrap_or("Unknown");
            let status = holiday.market_status.as_deref().unwrap_or("closed");

            let icon = match name {
                name if name.contains("New Year") => "🎆",
                name if name.contains("Christmas") => "🎄",
                name if name.contains("Thanksgiving") => "🦃",
                name if name.contains("Independence") => "🇺🇸",
                name if name.contains("Labor") => "👷",
                name if name.contains("Memorial") => "🕊️",
                name if name.contains("Presidents") => "🏛️",
                _ => "📅",
            };

            println!("      {} {} {} ({})", icon, date, name, status);
        }
        println!();
    }

    // 5. Trading Strategy Insights
    println!("💡 TRADING HOURS INSIGHTS");
    println!("─────────────────────────────────────────────────────────");
    println!("  🕘 Regular Hours Trading:");
    println!("     • Higher volume and liquidity");
    println!("     • Better price discovery");
    println!("     • Most institutional activity");
    println!("     • Reduced bid-ask spreads");
    println!();
    println!("  🌅 Pre-Market Trading (4:00 AM - 9:30 AM ET):");
    println!("     • React to overnight news and earnings");
    println!("     • Lower volume, higher volatility");
    println!("     • Gap trading opportunities");
    println!("     • Economic data releases impact");
    println!();
    println!("  🌙 After-Hours Trading (4:00 PM - 8:00 PM ET):");
    println!("     • Earnings announcements reactions");
    println!("     • News-driven price movements");
    println!("     • Lower liquidity, wider spreads");
    println!("     • Limited order types available");
    println!();
    println!("  🎄 Holiday Trading Considerations:");
    println!("     • Plan around market closures");
    println!("     • Early close days have compressed sessions");
    println!("     • Lower volume on holiday-adjacent days");
    println!("     • International markets may remain open");
    println!();
    println!("  🌍 Global Market Coordination:");
    println!("     • Asian markets: 6:00 PM - 3:00 AM ET (next day)");
    println!("     • European markets: 3:00 AM - 11:30 AM ET");
    println!("     • US markets: 9:30 AM - 4:00 PM ET");
    println!("     • Overlap periods offer increased opportunity");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("⚠️  TRADING HOURS DISCLAIMER: Market hours can change due");
    println!("to holidays, technical issues, or market conditions.");
    println!("Always verify current market status before trading.");
    println!("Extended hours trading involves additional risks.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
