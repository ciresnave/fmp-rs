//! Crypto & Forex Markets Example
//!
//! Demonstrates how to track cryptocurrency and forex markets,
//! including real-time quotes, historical data, and intraday prices.
//!
//! Run with: cargo run --example crypto_forex

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("=== Crypto & Forex Markets Dashboard ===\n");

    // 1. Cryptocurrency overview
    println!("💰 Cryptocurrency Market Overview:");
    println!("{}", "=".repeat(60));

    // Get list of available cryptos
    match client.crypto().get_crypto_list().await {
        Ok(cryptos) => {
            println!("Available Cryptocurrencies: {}\n", cryptos.len());
            println!("Sample Cryptocurrencies:");
            for crypto in cryptos.iter().take(10) {
                println!(
                    "  {}: {}",
                    crypto.symbol,
                    crypto.name.as_deref().unwrap_or("N/A")
                );
            }
        }
        Err(e) => eprintln!("Error fetching crypto list: {}", e),
    }

    // 2. Major cryptocurrency quotes
    println!("\n\n📊 Major Cryptocurrency Prices:");
    println!("{}", "=".repeat(60));

    let major_cryptos = vec![
        ("BTCUSD", "Bitcoin"),
        ("ETHUSD", "Ethereum"),
        ("ADAUSD", "Cardano"),
        ("SOLUSD", "Solana"),
    ];

    for (symbol, name) in major_cryptos {
        match client.crypto().get_crypto_quote(symbol).await {
            Ok(quotes) => {
                if let Some(quote) = quotes.first() {
                    let change_indicator = if quote.changes_percentage.unwrap_or(0.0) >= 0.0 {
                        "📈"
                    } else {
                        "📉"
                    };

                    println!("\n{} ({}):", name, symbol);
                    println!("  Price: ${:.2}", quote.price.unwrap_or(0.0));
                    println!(
                        "  24h Change: {} {:+.2}%",
                        change_indicator,
                        quote.changes_percentage.unwrap_or(0.0)
                    );

                    if let Some(market_cap) = quote.market_cap {
                        println!("  Market Cap: ${:.2}B", market_cap / 1_000_000_000.0);
                    }

                    if let (Some(low), Some(high)) = (quote.day_low, quote.day_high) {
                        println!("  24h Range: ${:.2} - ${:.2}", low, high);
                    }

                    if let Some(volume) = quote.volume {
                        println!("  24h Volume: ${:.2}M", volume / 1_000_000.0);
                    }
                }
            }
            Err(e) => eprintln!("Error fetching {}: {}", name, e),
        }
    }

    // 3. Bitcoin historical trend
    println!("\n\n📈 Bitcoin Historical Price (Last 7 Days):");
    println!("{}", "=".repeat(60));

    match client
        .crypto()
        .get_crypto_historical("BTCUSD", None, None)
        .await
    {
        Ok(history) => {
            for day in history.iter().take(7) {
                let change = ((day.close - day.open) / day.open) * 100.0;
                let indicator = if change >= 0.0 { "🟢" } else { "🔴" };

                println!(
                    "{} {} - Close: ${:.2} ({:+.2}%) | Vol: ${:.0}M",
                    indicator,
                    day.date,
                    day.close,
                    change,
                    day.volume / 1_000_000.0
                );
            }

            // Calculate 7-day trend
            if history.len() >= 7 {
                let current = history[0].close;
                let week_ago = history[6].close;
                let week_change = ((current - week_ago) / week_ago) * 100.0;
                let trend = if week_change > 0.0 {
                    "Bullish 🐂"
                } else {
                    "Bearish 🐻"
                };
                println!("\n7-Day Trend: {:+.2}% ({})", week_change, trend);
            }
        }
        Err(e) => eprintln!("Error fetching Bitcoin history: {}", e),
    }

    // 4. Ethereum intraday analysis
    println!("\n\n⏱️  Ethereum Intraday (5-Minute Intervals):");
    println!("{}", "=".repeat(60));

    match client.crypto().get_crypto_intraday("ETHUSD", "5min").await {
        Ok(intraday) => {
            println!("Last 10 Price Updates:\n");
            for tick in intraday.iter().take(10) {
                let change = ((tick.close - tick.open) / tick.open) * 100.0;
                let indicator = if change >= 0.0 { "↑" } else { "↓" };

                println!(
                    "{} {} ${:.2} ({:+.2}%)",
                    indicator, tick.date, tick.close, change
                );
            }
        }
        Err(e) => eprintln!("Error fetching Ethereum intraday: {}", e),
    }

    // 5. Forex market overview
    println!("\n\n\n💱 Foreign Exchange Market:");
    println!("{}", "=".repeat(60));

    // Get list of available forex pairs
    match client.forex().get_forex_list().await {
        Ok(pairs) => {
            println!("Available Forex Pairs: {}\n", pairs.len());
            println!("Sample Pairs:");
            for pair in pairs.iter().take(10) {
                println!(
                    "  {}: {}",
                    pair.symbol,
                    pair.name.as_deref().unwrap_or("N/A")
                );
            }
        }
        Err(e) => eprintln!("Error fetching forex list: {}", e),
    }

    // 6. Major forex pair quotes
    println!("\n\n📊 Major Forex Pairs:");
    println!("{}", "=".repeat(60));

    let major_pairs = vec![
        ("EURUSD", "Euro / US Dollar"),
        ("GBPUSD", "British Pound / US Dollar"),
        ("USDJPY", "US Dollar / Japanese Yen"),
        ("USDCHF", "US Dollar / Swiss Franc"),
    ];

    for (symbol, name) in major_pairs {
        match client.forex().get_forex_quote(symbol).await {
            Ok(quotes) => {
                if let Some(quote) = quotes.first() {
                    let change_indicator = if quote.changes_percentage.unwrap_or(0.0) >= 0.0 {
                        "📈"
                    } else {
                        "📉"
                    };

                    println!("\n{}:", name);
                    println!("  Rate: {:.5}", quote.price.unwrap_or(0.0));
                    println!(
                        "  Change: {} {:+.2}%",
                        change_indicator,
                        quote.changes_percentage.unwrap_or(0.0)
                    );

                    if let (Some(low), Some(high)) = (quote.day_low, quote.day_high) {
                        println!("  Day Range: {:.5} - {:.5}", low, high);
                    }
                }
            }
            Err(e) => eprintln!("Error fetching {}: {}", name, e),
        }
    }

    // 7. EUR/USD historical trend
    println!("\n\n📈 EUR/USD Historical Rates (Last 7 Days):");
    println!("{}", "=".repeat(60));

    match client
        .forex()
        .get_forex_historical("EURUSD", None, None)
        .await
    {
        Ok(history) => {
            for day in history.iter().take(7) {
                let change = ((day.close - day.open) / day.open) * 100.0;
                let indicator = if change >= 0.0 { "🟢" } else { "🔴" };

                println!(
                    "{} {} - Rate: {:.5} ({:+.2}%)",
                    indicator, day.date, day.close, change
                );
            }

            // Calculate 7-day trend
            if history.len() >= 7 {
                let current = history[0].close;
                let week_ago = history[6].close;
                let week_change = ((current - week_ago) / week_ago) * 100.0;
                let strength = if week_change > 0.0 {
                    "EUR Strengthening"
                } else {
                    "USD Strengthening"
                };
                println!("\n7-Day Trend: {:+.2}% ({})", week_change, strength);
            }
        }
        Err(e) => eprintln!("Error fetching EUR/USD history: {}", e),
    }

    // 8. GBP/USD intraday analysis
    println!("\n\n⏱️  GBP/USD Intraday (15-Minute Intervals):");
    println!("{}", "=".repeat(60));

    match client.forex().get_forex_intraday("GBPUSD", "15min").await {
        Ok(intraday) => {
            println!("Last 10 Rate Updates:\n");
            for tick in intraday.iter().take(10) {
                let change = ((tick.close - tick.open) / tick.open) * 100.0;
                let indicator = if change >= 0.0 { "↑" } else { "↓" };

                println!(
                    "{} {} {:.5} ({:+.3}%)",
                    indicator, tick.date, tick.close, change
                );
            }
        }
        Err(e) => eprintln!("Error fetching GBP/USD intraday: {}", e),
    }

    // 9. Market correlation insight
    println!("\n\n🔗 Market Insights:");
    println!("{}", "=".repeat(60));
    println!("Crypto markets operate 24/7, providing constant liquidity.");
    println!("Forex markets trade 24/5, closed on weekends.");
    println!("\nKey Correlations:");
    println!("  • Bitcoin often moves inversely to USD strength");
    println!("  • Risk-on sentiment typically supports both crypto and EUR");
    println!("  • Safe-haven flows strengthen USD, JPY, and CHF");

    println!("\n\n✓ Dashboard Complete");

    Ok(())
}
