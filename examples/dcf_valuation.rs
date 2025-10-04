//! Comprehensive example of DCF and valuation analysis
//!
//! Demonstrates using all DCF and valuation endpoints to:
//! - Get current DCF valuation vs. market price
//! - Analyze historical DCF trends
//! - View detailed DCF modeling assumptions
//! - Compare levered vs unlevered valuations
//! - Calculate enterprise value

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the FMP client
    let client = FmpClient::new()?;
    let symbol = "AAPL";

    println!("=== DCF & Valuation Analysis for {} ===\n", symbol);

    // 1. Current DCF Valuation
    println!("📊 Current DCF Valuation:");
    match client.dcf().get_dcf(symbol).await {
        Ok(dcf_list) => {
            if let Some(dcf) = dcf_list.first() {
                let dcf_value = dcf.dcf.unwrap_or(0.0);
                let stock_price = dcf.stock_price.unwrap_or(0.0);
                let upside = ((dcf_value - stock_price) / stock_price) * 100.0;

                println!("  DCF Value: ${:.2}", dcf_value);
                println!("  Stock Price: ${:.2}", stock_price);
                println!("  Potential Upside: {:.2}%", upside);
                println!("  Date: {}", dcf.date);

                if upside > 0.0 {
                    println!("  💡 DCF suggests stock is undervalued");
                } else {
                    println!("  ⚠️  DCF suggests stock is overvalued");
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 2. Historical DCF Trend
    println!("\n📈 Historical DCF (Last 5 periods):");
    match client.dcf().get_historical_dcf(symbol).await {
        Ok(history) => {
            for record in history.iter().take(5) {
                let dcf_value = record.dcf.unwrap_or(0.0);
                let stock_price = record.stock_price.unwrap_or(0.0);
                let diff = dcf_value - stock_price;
                let diff_pct = (diff / stock_price) * 100.0;

                println!(
                    "  {}: DCF ${:.2} vs Stock ${:.2} ({:+.1}%)",
                    record.date, dcf_value, stock_price, diff_pct
                );
            }

            // Analyze trend
            if history.len() >= 2 {
                let recent_diff =
                    (history[0].dcf.unwrap_or(0.0) - history[0].stock_price.unwrap_or(0.0)).abs();
                let older_diff = (history[history.len() - 1].dcf.unwrap_or(0.0)
                    - history[history.len() - 1].stock_price.unwrap_or(0.0))
                .abs();

                if recent_diff < older_diff {
                    println!("  📉 Valuation gap narrowing over time");
                } else {
                    println!("  📈 Valuation gap widening over time");
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 3. Advanced DCF with Assumptions
    println!("\n🔬 Advanced DCF (Modeling Assumptions):");
    match client.dcf().get_advanced_dcf(symbol).await {
        Ok(dcf_list) => {
            if let Some(dcf) = dcf_list.first() {
                println!("  DCF Value: ${:.2}", dcf.dcf.unwrap_or(0.0));
                println!("  Stock Price: ${:.2}", dcf.stock_price.unwrap_or(0.0));
                println!("\n  Key Assumptions:");
                println!(
                    "    Revenue Growth: {:.2}%",
                    dcf.revenue_growth.unwrap_or(0.0) * 100.0
                );
                println!(
                    "    Operating Margin: {:.2}%",
                    dcf.operating_margin.unwrap_or(0.0) * 100.0
                );
                println!("    Tax Rate: {:.2}%", dcf.tax_rate.unwrap_or(0.0) * 100.0);
                println!("    WACC: {:.2}%", dcf.wacc.unwrap_or(0.0) * 100.0);
                println!(
                    "    Terminal Growth: {:.2}%",
                    dcf.terminal_growth_rate.unwrap_or(0.0) * 100.0
                );

                if !dcf.free_cash_flows.is_empty() {
                    println!("\n  Projected Free Cash Flows:");
                    for (year, fcf) in dcf.free_cash_flows.iter().enumerate() {
                        println!("    Year {}: ${:.2}M", year + 1, fcf / 1_000_000.0);
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 4. Levered DCF (Equity Value)
    println!("\n💰 Levered DCF (Equity Value):");
    match client.dcf().get_levered_dcf(symbol).await {
        Ok(dcf_list) => {
            if let Some(dcf) = dcf_list.first() {
                let enterprise_value = dcf.enterprise_value.unwrap_or(0.0);
                let total_debt = dcf.total_debt.unwrap_or(0.0);
                let cash = dcf.cash_and_equivalents.unwrap_or(0.0);

                println!(
                    "  Enterprise Value: ${:.2}B",
                    enterprise_value / 1_000_000_000.0
                );
                println!("  - Total Debt: ${:.2}B", total_debt / 1_000_000_000.0);
                println!("  + Cash: ${:.2}B", cash / 1_000_000_000.0);
                println!(
                    "  = Equity Value (Levered DCF): ${:.2}B",
                    dcf.levered_dcf.unwrap_or(0.0) / 1_000_000_000.0
                );
                println!("  Stock Price: ${:.2}", dcf.stock_price.unwrap_or(0.0));

                // Calculate net debt position
                let net_debt = total_debt - cash;
                if net_debt > 0.0 {
                    println!("  💳 Net Debt: ${:.2}B", net_debt / 1_000_000_000.0);
                } else {
                    println!("  💵 Net Cash: ${:.2}B", -net_debt / 1_000_000_000.0);
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 5. Enterprise Value Components
    println!("\n🏢 Enterprise Value Breakdown:");
    match client.dcf().get_enterprise_value(symbol).await {
        Ok(ev_list) => {
            if let Some(ev) = ev_list.first() {
                let market_cap = ev.market_capitalization.unwrap_or(0.0);
                let debt = ev.add_total_debt.unwrap_or(0.0);
                let cash = ev.minus_cash_and_cash_equivalents.unwrap_or(0.0);
                let enterprise_value = ev.enterprise_value.unwrap_or(0.0);

                println!(
                    "  Market Capitalization: ${:.2}B",
                    market_cap / 1_000_000_000.0
                );
                println!("  + Total Debt: ${:.2}B", debt / 1_000_000_000.0);
                println!("  - Cash & Equivalents: ${:.2}B", cash / 1_000_000_000.0);
                println!(
                    "  = Enterprise Value: ${:.2}B",
                    enterprise_value / 1_000_000_000.0
                );
                println!("  Date: {}", ev.date);

                // Calculate ratios
                let debt_to_ev = (debt / enterprise_value) * 100.0;
                let cash_to_ev = (cash / enterprise_value) * 100.0;

                println!("\n  Capital Structure:");
                println!("    Debt/EV: {:.1}%", debt_to_ev);
                println!("    Cash/EV: {:.1}%", cash_to_ev);

                if debt_to_ev > 50.0 {
                    println!("    ⚠️  High debt leverage");
                } else if debt_to_ev < 20.0 {
                    println!("    ✅ Conservative leverage");
                } else {
                    println!("    ✅ Moderate leverage");
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n=== Analysis Complete ===");

    Ok(())
}
