//! Comprehensive Financial analysis example
//!
//! Demonstrates complete financial analysis using multiple FMP endpoints:
//! - Financial statements (income, balance sheet, cash flow)
//! - Financial growth metrics
//! - Financial ratios and key metrics (TTM)
//! - As-reported data (XBRL from SEC filings)
//! - Revenue segmentation (product and geographic)
//!
//! To run: cargo run --example financial_analysis
//!
//! Make sure FMP_API_KEY environment variable is set

use fmp_rs::{models::common::Period, FmpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;
    let symbol = "AAPL";

    println!("=== Financial Analysis for {} ===\n", symbol);

    // Get key metrics
    println!("📊 Key Metrics (TTM):");
    let metrics = client.financials().get_key_metrics_ttm(symbol).await?;
    if let Some(metric) = metrics.first() {
        println!("   P/E Ratio: {:.2}", metric.pe_ratio);
        println!("   Price to Sales: {:.2}", metric.price_to_sales_ratio);
        println!("   Debt to Equity: {:.2}", metric.debt_to_equity);
        println!("   Current Ratio: {:.2}", metric.current_ratio);
        println!("   ROE: {:.2}%", metric.roe * 100.0);
        println!("   Dividend Yield: {:.2}%", metric.dividend_yield * 100.0);
    }

    // Get financial ratios
    println!("\n📈 Financial Ratios (TTM):");
    let ratios = client.financials().get_ratios_ttm(symbol).await?;
    if let Some(ratio) = ratios.first() {
        println!(
            "   Gross Profit Margin: {:.2}%",
            ratio.gross_profit_margin * 100.0
        );
        println!(
            "   Operating Profit Margin: {:.2}%",
            ratio.operating_profit_margin * 100.0
        );
        println!(
            "   Net Profit Margin: {:.2}%",
            ratio.net_profit_margin * 100.0
        );
        println!(
            "   Return on Assets: {:.2}%",
            ratio.return_on_assets * 100.0
        );
        println!(
            "   Return on Equity: {:.2}%",
            ratio.return_on_equity * 100.0
        );
    }

    // Get recent income statements
    println!("\n💰 Revenue & Profit Trends:");
    let income = client
        .financials()
        .get_income_statement(symbol, Period::Annual, Some(5))
        .await?;

    for statement in income {
        let margin = (statement.net_income as f64 / statement.revenue as f64) * 100.0;
        println!(
            "   {} - Revenue: ${:>12} | Net Income: ${:>12} | Margin: {:.1}%",
            statement.calendar_year,
            format_large_number(statement.revenue),
            format_large_number(statement.net_income),
            margin
        );
    }

    // Get balance sheet highlights
    println!("\n🏦 Balance Sheet Highlights:");
    let balance = client
        .financials()
        .get_balance_sheet(symbol, Period::Annual, Some(1))
        .await?;

    if let Some(sheet) = balance.first() {
        println!(
            "   Total Assets: ${}",
            format_large_number(sheet.total_assets)
        );
        println!(
            "   Total Liabilities: ${}",
            format_large_number(sheet.total_liabilities)
        );
        println!(
            "   Total Equity: ${}",
            format_large_number(sheet.total_stockholders_equity)
        );
        println!(
            "   Cash: ${}",
            format_large_number(sheet.cash_and_cash_equivalents)
        );
        println!("   Total Debt: ${}", format_large_number(sheet.total_debt));
    }

    // Get cash flow
    println!("\n💵 Cash Flow Analysis:");
    let cashflow = client
        .financials()
        .get_cash_flow_statement(symbol, Period::Annual, Some(3))
        .await?;

    for cf in cashflow {
        println!(
            "   {} - Operating CF: ${:>12} | Free CF: ${:>12}",
            cf.calendar_year,
            format_large_number(cf.operating_cash_flow),
            format_large_number(cf.free_cash_flow)
        );
    }

    // Get financial growth data
    println!("\n📈 Financial Growth Analysis (YoY):");
    let growth = client
        .financials()
        .get_financial_growth(symbol, Period::Annual, Some(3))
        .await?;

    for g in growth {
        println!("   {}:", g.date);
        println!("     Revenue Growth: {:.2}%", g.revenue_growth * 100.0);
        println!(
            "     Net Income Growth: {:.2}%",
            g.net_income_growth * 100.0
        );
        println!("     EPS Growth: {:.2}%", g.eps_growth * 100.0);
        println!(
            "     Free Cash Flow Growth: {:.2}%",
            g.free_cash_flow_growth * 100.0
        );
    }

    // Get revenue segmentation
    println!("\n🗺️ Revenue Segmentation Analysis:");

    // Product segmentation
    let product_segments = client
        .financials()
        .get_revenue_product_segmentation(symbol, Period::Annual, None)
        .await?;

    if let Some(seg) = product_segments.first() {
        println!("   By Product ({}):", seg.date);
        for (product, revenue) in &seg.segments {
            if let Some(rev_val) = revenue.as_f64() {
                println!("     {}: ${}", product, format_large_number(rev_val as i64));
            }
        }
    }

    // Geographic segmentation
    let geo_segments = client
        .financials()
        .get_revenue_geographic_segmentation(symbol, Period::Annual, None)
        .await?;

    if let Some(seg) = geo_segments.first() {
        println!("   By Geography ({}):", seg.date);
        for (region, revenue) in &seg.segments {
            if let Some(rev_val) = revenue.as_f64() {
                println!("     {}: ${}", region, format_large_number(rev_val as i64));
            }
        }
    }

    // Get as-reported data
    println!("\n📄 As-Reported Data (SEC Filings):");
    let as_reported = client
        .financials()
        .get_financial_as_reported(symbol, Period::Annual, Some(1))
        .await?;

    if let Some(report) = as_reported.first() {
        println!("   Filing Date: {}", report.date);
        println!("   Accepted: {}", report.accepted_date);
        println!("   Period: {}", report.period);
        println!("   Contains {} XBRL fields", report.data.len());
    }

    println!("\n✅ Comprehensive financial analysis complete!");
    println!("\nAnalysis includes:");
    println!("  • Financial statements (income, balance sheet, cash flow)");
    println!("  • Year-over-year growth metrics");
    println!("  • Financial ratios and key metrics");
    println!("  • Revenue segmentation by product and geography");
    println!("  • SEC filing data with XBRL fields");

    Ok(())
}

fn format_large_number(num: i64) -> String {
    if num >= 1_000_000_000 {
        format!("{:.2}B", num as f64 / 1_000_000_000.0)
    } else if num >= 1_000_000 {
        format!("{:.2}M", num as f64 / 1_000_000.0)
    } else {
        format!("{}", num)
    }
}
