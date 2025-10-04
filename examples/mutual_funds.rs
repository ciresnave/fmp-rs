//! Mutual Funds Analysis Dashboard
//!
//! This example demonstrates comprehensive mutual fund analysis including
//! fund search, performance metrics, holdings analysis, and portfolio allocation.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("            📊 MUTUAL FUNDS ANALYSIS DASHBOARD 📊");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Fund Discovery and Search
    println!("🔍 FUND DISCOVERY & SEARCH");
    println!("─────────────────────────────────────────────────────────");

    // Search for Vanguard funds
    println!("  🏛️ Vanguard Fund Family:");
    let vanguard_funds = client.mutual_funds().search_funds(Some("Vanguard")).await?;
    if !vanguard_funds.is_empty() {
        println!();
        for fund in vanguard_funds.iter().take(8) {
            let symbol = fund.symbol.as_deref().unwrap_or("N/A");
            let name = fund.name.as_deref().unwrap_or("Unknown Fund");
            let category = fund.category.as_deref().unwrap_or("N/A");
            let nav = fund.nav.unwrap_or(0.0);
            let assets = fund.total_assets.unwrap_or(0.0) / 1_000_000_000.0; // Convert to billions
            let expense_ratio = fund.expense_ratio.unwrap_or(0.0) * 100.0; // Convert to percentage

            println!("    {} - {}", symbol, name);
            println!(
                "      Category: {} | NAV: ${:.2} | AUM: ${:.1}B | ER: {:.2}%",
                category, nav, assets, expense_ratio
            );
            println!();
        }
    } else {
        println!("    No Vanguard funds found in search results");
    }

    // Search for Fidelity funds
    println!("  🏦 Fidelity Fund Family:");
    let fidelity_funds = client.mutual_funds().search_funds(Some("Fidelity")).await?;
    if !fidelity_funds.is_empty() {
        println!();
        for fund in fidelity_funds.iter().take(5) {
            let symbol = fund.symbol.as_deref().unwrap_or("N/A");
            let name = fund.name.as_deref().unwrap_or("Unknown Fund");
            let nav = fund.nav.unwrap_or(0.0);
            let expense_ratio = fund.expense_ratio.unwrap_or(0.0) * 100.0;

            println!(
                "    {} - {} | NAV: ${:.2} | ER: {:.2}%",
                symbol, name, nav, expense_ratio
            );
        }
        println!();
    }

    // 2. Detailed Fund Analysis
    println!("📈 DETAILED FUND ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    let analysis_funds = ["VTSAX", "FXNAX", "VTSMX"]; // Popular index funds
    let fund_names = [
        "Vanguard Total Stock Market",
        "Fidelity US Bond Index",
        "Vanguard Total Stock Market Investor",
    ];

    for (symbol, display_name) in analysis_funds.iter().zip(fund_names.iter()) {
        println!("  💼 {} ({}):", display_name, symbol);

        let fund_details = client.mutual_funds().get_fund_details(symbol).await?;
        if let Some(fund) = fund_details.first() {
            let nav = fund.nav.unwrap_or(0.0);
            let _prev_nav = fund.previous_nav.unwrap_or(0.0);
            let change = fund.change.unwrap_or(0.0);
            let change_pct = fund.change_percent.unwrap_or(0.0);
            let assets = fund.total_assets.unwrap_or(0.0) / 1_000_000_000.0;
            let expense_ratio = fund.expense_ratio.unwrap_or(0.0) * 10000.0; // Convert to basis points
            let min_investment = fund.minimum_investment.unwrap_or(0.0);
            let yield_val = fund.yield_.unwrap_or(0.0);
            let inception = fund.inception_date.as_deref().unwrap_or("N/A");

            let change_emoji = if change >= 0.0 { "🟢" } else { "🔴" };

            println!(
                "    NAV: ${:.2} {} {:+.2} ({:+.2}%)",
                nav, change_emoji, change, change_pct
            );
            println!("    Assets Under Management: ${:.1}B", assets);
            println!(
                "    Expense Ratio: {:.0} basis points ({:.3}%)",
                expense_ratio,
                expense_ratio / 100.0
            );
            println!("    Minimum Investment: ${:.0}", min_investment);
            println!("    Current Yield: {:.2}%", yield_val);
            println!("    Inception Date: {}", inception);

            if let Some(category) = &fund.category {
                println!("    Category: {}", category);
            }
            if let Some(objective) = &fund.investment_objective {
                println!("    Objective: {}", objective);
            }
        } else {
            println!("    Fund details not available");
        }
        println!();
    }

    // 3. Performance Analysis
    println!("🚀 PERFORMANCE ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    for symbol in &analysis_funds {
        let performance = client.mutual_funds().get_fund_performance(symbol).await?;
        if let Some(perf) = performance.first() {
            let name = perf.name.as_deref().unwrap_or("Unknown Fund");
            println!("  📊 {} ({}):", name, symbol);
            println!("    ┌─────────────┬──────────┬──────────────────────────┐");
            println!("    │ Time Period │  Return  │ Visualization            │");
            println!("    ├─────────────┼──────────┼──────────────────────────┤");

            let periods = [
                ("1 Day", perf.one_day),
                ("1 Week", perf.one_week),
                ("1 Month", perf.one_month),
                ("3 Months", perf.three_months),
                ("YTD", perf.ytd),
                ("1 Year", perf.one_year),
                ("3 Years", perf.three_years),
                ("5 Years", perf.five_years),
                ("10 Years", perf.ten_years),
            ];

            for (period, return_val) in periods {
                if let Some(ret) = return_val {
                    let bar_length = ((ret.abs() / 2.0) as usize).min(20);
                    let bar = if ret >= 0.0 {
                        "█".repeat(bar_length)
                    } else {
                        "▓".repeat(bar_length)
                    };
                    let color = if ret >= 0.0 { "🟢" } else { "🔴" };

                    println!("    │ {:11} │ {:+6.2}%  │ {} {}", period, ret, color, bar);
                } else {
                    println!(
                        "    │ {:11} │    N/A   │ No data available        │",
                        period
                    );
                }
            }

            println!("    └─────────────┴──────────┴──────────────────────────┘");

            // Risk metrics
            if let (Some(sharpe), Some(beta), Some(alpha)) =
                (perf.sharpe_ratio, perf.beta, perf.alpha)
            {
                println!("    Risk Metrics:");
                println!(
                    "      Sharpe Ratio: {:.2} | Beta: {:.2} | Alpha: {:.2}%",
                    sharpe, beta, alpha
                );

                if let Some(std_dev) = perf.standard_deviation {
                    println!("      Standard Deviation: {:.2}%", std_dev);
                }
            }
            println!();
        }
    }

    // 4. Fund Holdings Analysis
    println!("🏢 TOP HOLDINGS ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    let holdings_symbol = "VTSAX"; // Focus on one fund for detailed holdings
    println!("  📋 {} Top Holdings:", holdings_symbol);
    let holdings = client
        .mutual_funds()
        .get_fund_holdings(holdings_symbol)
        .await?;

    if !holdings.is_empty() {
        println!();
        println!("    ┌──────┬─────────────────────────────────┬──────────┬──────────┐");
        println!("    │ Rank │ Security                        │  Weight  │  Sector  │");
        println!("    ├──────┼─────────────────────────────────┼──────────┼──────────┤");

        for (i, holding) in holdings.iter().take(15).enumerate() {
            let rank = i + 1;
            let symbol = holding.symbol.as_deref().unwrap_or("N/A");
            let name = holding.name.as_deref().unwrap_or("Unknown");
            let weight = holding.weight_percentage.unwrap_or(0.0);
            let sector = holding.sector.as_deref().unwrap_or("N/A");

            // Truncate long names
            let display_name = if name.len() > 25 {
                format!("{}...", &name[..22])
            } else {
                format!("{:25}", name)
            };

            let sector_display = if sector.len() > 8 {
                format!("{}...", &sector[..5])
            } else {
                format!("{:8}", sector)
            };

            println!(
                "    │ {:4} │ {} ({:4}) │ {:6.2}%  │ {} │",
                rank, display_name, symbol, weight, sector_display
            );
        }
        println!("    └──────┴─────────────────────────────────┴──────────┴──────────┘");
        println!();
    }

    // 5. Sector Allocation Analysis
    println!("🏭 SECTOR ALLOCATION ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    let sectors = client
        .mutual_funds()
        .get_fund_sectors(holdings_symbol)
        .await?;
    if !sectors.is_empty() {
        println!("  🎯 {} Sector Breakdown:", holdings_symbol);
        println!();

        for sector in sectors.iter().take(12) {
            let sector_name = sector.sector.as_deref().unwrap_or("Unknown");
            let weight = sector.weight_percentage.unwrap_or(0.0);

            // Create visual bar
            let bar_length = (weight / 2.0) as usize; // Scale for display
            let bar = "█".repeat(bar_length.min(25));

            println!("    {:20} {:5.1}% {}", sector_name, weight, bar);
        }
        println!();
    }

    // 6. Historical Performance Trend
    println!("📅 HISTORICAL PERFORMANCE TREND");
    println!("─────────────────────────────────────────────────────────");

    // Get last 30 days of data
    let current_date = chrono::Utc::now();
    let start_date = current_date - chrono::Duration::days(30);
    let end_date = current_date;

    let history = client
        .mutual_funds()
        .get_fund_historical(
            holdings_symbol,
            Some(&start_date.format("%Y-%m-%d").to_string()),
            Some(&end_date.format("%Y-%m-%d").to_string()),
        )
        .await?;

    if !history.is_empty() {
        println!("  📈 {} - Last 30 Days NAV Trend:", holdings_symbol);
        println!();

        // Show recent trend (last 10 days)
        for entry in history.iter().take(10) {
            let date = entry.date.as_deref().unwrap_or("N/A");
            let nav = entry.nav.unwrap_or(0.0);
            let change = entry.change.unwrap_or(0.0);
            let change_pct = entry.change_percent.unwrap_or(0.0);

            let trend_emoji = if change >= 0.0 { "📈" } else { "📉" };
            let change_color = if change >= 0.0 { "🟢" } else { "🔴" };

            println!(
                "    {} {} NAV: ${:.2} {} {:+.2} ({:+.2}%)",
                date, trend_emoji, nav, change_color, change, change_pct
            );
        }

        // Calculate 30-day statistics
        if let (Some(first), Some(last)) = (history.last(), history.first()) {
            let first_nav = first.nav.unwrap_or(0.0);
            let last_nav = last.nav.unwrap_or(0.0);
            let total_return = ((last_nav - first_nav) / first_nav) * 100.0;

            println!();
            println!("    30-Day Summary:");
            println!(
                "      Start NAV: ${:.2} | End NAV: ${:.2}",
                first_nav, last_nav
            );
            println!("      Total Return: {:+.2}%", total_return);

            // Volatility calculation
            let daily_returns: Vec<f64> = history
                .windows(2)
                .filter_map(|pair| {
                    if let (Some(prev_nav), Some(curr_nav)) = (pair[1].nav, pair[0].nav) {
                        Some(((curr_nav - prev_nav) / prev_nav) * 100.0)
                    } else {
                        None
                    }
                })
                .collect();

            if !daily_returns.is_empty() {
                let avg_return = daily_returns.iter().sum::<f64>() / daily_returns.len() as f64;
                let variance = daily_returns
                    .iter()
                    .map(|r| (r - avg_return).powi(2))
                    .sum::<f64>()
                    / (daily_returns.len() - 1) as f64;
                let std_dev = variance.sqrt();
                let annualized_vol = std_dev * (252.0_f64).sqrt(); // Trading days per year

                println!(
                    "      30-Day Volatility: {:.2}% (Annualized: {:.2}%)",
                    std_dev, annualized_vol
                );
            }
        }
        println!();
    }

    // 7. Investment Insights & Summary
    println!("💡 MUTUAL FUND INVESTMENT INSIGHTS");
    println!("─────────────────────────────────────────────────────────");
    println!("  🎯 Key Investment Considerations:");
    println!("     • Expense Ratio: Lower fees mean more returns stay in your pocket");
    println!("     • Diversification: Broad market funds reduce individual stock risk");
    println!("     • Tax Efficiency: Index funds typically have lower turnover");
    println!("     • Minimum Investment: Consider initial and subsequent investment amounts");
    println!();
    println!("  📊 Performance Evaluation:");
    println!("     • Compare returns to relevant benchmarks (S&P 500, bond indices)");
    println!("     • Consider risk-adjusted returns (Sharpe ratio)");
    println!("     • Evaluate consistency over multiple time periods");
    println!("     • Review manager tenure and investment philosophy");
    println!();
    println!("  🏭 Portfolio Construction:");
    println!("     • Asset Allocation: Mix stocks, bonds, and other asset classes");
    println!("     • Geographic Diversification: Domestic and international exposure");
    println!("     • Sector Balance: Avoid over-concentration in single sectors");
    println!("     • Cost Management: Monitor total portfolio expense ratios");
    println!();
    println!("  ⏰ Investment Horizon:");
    println!("     • Long-term (10+ years): Higher equity allocation acceptable");
    println!("     • Medium-term (3-10 years): Balanced approach with some bonds");
    println!("     • Short-term (<3 years): Conservative allocation, more bonds/cash");
    println!("     • Regular rebalancing: Maintain target allocations over time");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("⚠️  INVESTMENT DISCLAIMER: Past performance does not guarantee");
    println!("future results. Mutual fund investing involves risk including");
    println!("potential loss of principal. Consider your investment objectives,");
    println!("risk tolerance, and time horizon before investing. Consult");
    println!("with a financial advisor for personalized guidance.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
