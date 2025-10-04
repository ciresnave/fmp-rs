//! Economics Dashboard Example
//!
//! Demonstrates how to track treasury rates, GDP, inflation, unemployment,
//! and other key economic indicators.
//!
//! Run with: cargo run --example economics

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("=== Economic Indicators Dashboard ===\n");

    // 1. Treasury rates - yield curve analysis
    println!("📊 US Treasury Yield Curve:");
    println!("{}", "=".repeat(60));
    match client.economics().get_treasury_rates(None, None).await {
        Ok(rates) => {
            if let Some(latest) = rates.first() {
                println!("Latest Rates ({})", latest.date);
                println!("  3-Month:  {:.2}%", latest.month_3.unwrap_or(0.0));
                println!("  2-Year:   {:.2}%", latest.year_2.unwrap_or(0.0));
                println!("  10-Year:  {:.2}%", latest.year_10.unwrap_or(0.0));
                println!("  30-Year:  {:.2}%", latest.year_30.unwrap_or(0.0));

                // Calculate yield curve spread (10Y - 2Y)
                if let (Some(ten_year), Some(two_year)) = (latest.year_10, latest.year_2) {
                    let spread = ten_year - two_year;
                    let curve_shape = if spread > 0.5 {
                        "Steep (Normal)"
                    } else if spread > 0.0 {
                        "Positive (Normal)"
                    } else if spread < -0.25 {
                        "⚠️ Inverted (Recession Signal)"
                    } else {
                        "Flat"
                    };
                    println!("\n  10Y-2Y Spread: {:.2}% ({})", spread, curve_shape);
                }
            }

            // Show historical trend
            println!("\n📈 10-Year Treasury Trend (Last 5 Periods):");
            for rate in rates.iter().take(5) {
                if let Some(ten_year) = rate.year_10 {
                    println!("  {}: {:.2}%", rate.date, ten_year);
                }
            }
        }
        Err(e) => eprintln!("Error fetching treasury rates: {}", e),
    }

    // 2. GDP analysis
    println!("\n\n💰 GDP (Gross Domestic Product):");
    println!("{}", "=".repeat(60));
    match client.economics().get_gdp().await {
        Ok(gdp_data) => {
            if let Some(latest) = gdp_data.first() {
                println!(
                    "Latest GDP: ${:.2} Trillion ({})",
                    latest.value.unwrap_or(0.0) / 1_000_000_000_000.0,
                    latest.date
                );
            }

            // Calculate GDP growth
            if gdp_data.len() >= 2 {
                if let (Some(current), Some(previous)) = (gdp_data[0].value, gdp_data[1].value) {
                    let growth_rate = ((current - previous) / previous) * 100.0;
                    let indicator = if growth_rate > 0.0 { "📈" } else { "📉" };
                    println!(
                        "{} GDP Growth: {:+.2}% from previous period",
                        indicator, growth_rate
                    );
                }
            }

            println!("\nHistorical GDP (Last 5 Periods):");
            for record in gdp_data.iter().take(5) {
                println!(
                    "  {}: ${:.2}T",
                    record.date,
                    record.value.unwrap_or(0.0) / 1_000_000_000_000.0
                );
            }
        }
        Err(e) => eprintln!("Error fetching GDP: {}", e),
    }

    // 3. Real GDP (inflation-adjusted)
    println!("\n\n📐 Real GDP (Inflation-Adjusted):");
    println!("{}", "=".repeat(60));
    match client.economics().get_real_gdp().await {
        Ok(real_gdp) => {
            if let Some(latest) = real_gdp.first() {
                println!(
                    "Latest Real GDP: ${:.2} Trillion ({})",
                    latest.value.unwrap_or(0.0) / 1_000_000_000_000.0,
                    latest.date
                );
            }
        }
        Err(e) => eprintln!("Error fetching Real GDP: {}", e),
    }

    // 4. GDP per capita
    println!("\n\n👥 GDP Per Capita:");
    println!("{}", "=".repeat(60));
    match client.economics().get_gdp_per_capita().await {
        Ok(gdp_pc) => {
            if let Some(latest) = gdp_pc.first() {
                println!(
                    "Latest GDP Per Capita: ${:.2} ({})",
                    latest.value.unwrap_or(0.0),
                    latest.date
                );
            }
        }
        Err(e) => eprintln!("Error fetching GDP per capita: {}", e),
    }

    // 5. Inflation analysis (CPI)
    println!("\n\n📊 Consumer Price Index (CPI):");
    println!("{}", "=".repeat(60));
    match client.economics().get_cpi().await {
        Ok(cpi_data) => {
            if let Some(latest) = cpi_data.first() {
                println!(
                    "Latest CPI: {:.2} ({})",
                    latest.value.unwrap_or(0.0),
                    latest.date
                );
            }

            // Calculate year-over-year inflation
            if cpi_data.len() >= 13 {
                if let (Some(current), Some(year_ago)) = (cpi_data[0].value, cpi_data[12].value) {
                    let yoy_inflation = ((current - year_ago) / year_ago) * 100.0;
                    let trend = if yoy_inflation > 3.0 {
                        "⚠️ High"
                    } else if yoy_inflation > 2.0 {
                        "Moderate"
                    } else {
                        "✓ Low"
                    };
                    println!(
                        "Year-over-Year Inflation: {:.2}% ({})",
                        yoy_inflation, trend
                    );
                }
            }

            println!("\nRecent CPI Values:");
            for record in cpi_data.iter().take(6) {
                println!("  {}: {:.2}", record.date, record.value.unwrap_or(0.0));
            }
        }
        Err(e) => eprintln!("Error fetching CPI: {}", e),
    }

    // 6. Inflation rate
    println!("\n\n🔥 Inflation Rate:");
    println!("{}", "=".repeat(60));
    match client.economics().get_inflation_rate().await {
        Ok(inflation) => {
            if let Some(latest) = inflation.first() {
                let rate = latest.value.unwrap_or(0.0);
                let indicator = if rate > 3.0 {
                    "🔴"
                } else if rate > 2.0 {
                    "🟡"
                } else {
                    "🟢"
                };
                println!(
                    "{} Current Inflation: {:.2}% ({})",
                    indicator, rate, latest.date
                );
            }

            println!("\nInflation Trend (Last 12 Months):");
            for record in inflation.iter().take(12) {
                let value = record.value.unwrap_or(0.0);
                let bar = "█".repeat((value * 2.0) as usize);
                println!("  {}: {:>5.2}% {}", record.date, value, bar);
            }
        }
        Err(e) => eprintln!("Error fetching inflation rate: {}", e),
    }

    // 7. Unemployment rate
    println!("\n\n👷 Unemployment Rate:");
    println!("{}", "=".repeat(60));
    match client.economics().get_unemployment_rate().await {
        Ok(unemployment) => {
            if let Some(latest) = unemployment.first() {
                let rate = latest.value.unwrap_or(0.0);
                let health = if rate < 4.0 {
                    "✓ Strong"
                } else if rate < 6.0 {
                    "Moderate"
                } else {
                    "⚠️ Weak"
                };
                println!(
                    "Current Unemployment: {:.1}% ({}) - {}",
                    rate, latest.date, health
                );
            }

            println!("\nUnemployment Trend (Last 12 Months):");
            for record in unemployment.iter().take(12) {
                println!("  {}: {:.1}%", record.date, record.value.unwrap_or(0.0));
            }
        }
        Err(e) => eprintln!("Error fetching unemployment rate: {}", e),
    }

    // 8. Federal funds rate
    println!("\n\n🏦 Federal Funds Rate:");
    println!("{}", "=".repeat(60));
    match client.economics().get_federal_funds_rate().await {
        Ok(fed_rate) => {
            if let Some(latest) = fed_rate.first() {
                println!(
                    "Current Fed Rate: {:.2}% ({})",
                    latest.value.unwrap_or(0.0),
                    latest.date
                );
            }

            // Show rate changes
            println!("\nFed Rate History (Last 10 Changes):");
            for record in fed_rate.iter().take(10) {
                println!("  {}: {:.2}%", record.date, record.value.unwrap_or(0.0));
            }

            // Analyze monetary policy stance
            if let Some(current_rate) = fed_rate.first().and_then(|r| r.value) {
                let stance = if current_rate > 4.0 {
                    "Restrictive (Fighting Inflation)"
                } else if current_rate > 2.0 {
                    "Neutral"
                } else if current_rate > 0.5 {
                    "Accommodative"
                } else {
                    "Highly Accommodative"
                };
                println!("\nMonetary Policy Stance: {}", stance);
            }
        }
        Err(e) => eprintln!("Error fetching federal funds rate: {}", e),
    }

    // 9. Market risk premium
    println!("\n\n📊 Market Risk Premium by Country:");
    println!("{}", "=".repeat(60));
    match client.economics().get_market_risk_premium().await {
        Ok(premiums) => {
            println!("Top 10 Countries by Total Equity Risk Premium:\n");
            for premium in premiums.iter().take(10) {
                println!(
                    "{:<20} Total ERP: {:>5.2}%  Country Risk: {:>5.2}%",
                    premium.country,
                    premium.total_equity_risk_premium.unwrap_or(0.0),
                    premium.country_risk_premium.unwrap_or(0.0)
                );
            }
        }
        Err(e) => eprintln!("Error fetching market risk premium: {}", e),
    }

    // Summary
    println!("\n\n=== Economic Snapshot Summary ===");
    println!("This dashboard provides key macroeconomic indicators that");
    println!("influence financial markets, monetary policy, and investment decisions.");
    println!("\n✓ Dashboard Complete");

    Ok(())
}
