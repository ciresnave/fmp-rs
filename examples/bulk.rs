//! Bulk Data Analysis Dashboard
//!
//! This example demonstrates comprehensive bulk data capabilities for:
//! - Large-scale market data processing
//! - Bulk financial statement analysis  
//! - Institutional holdings research
//! - ETF composition analysis
//! - Data management best practices

use fmp_rs::{FmpClient, error::Result};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let client = FmpClient::builder().api_key("your_api_key_here").build()?;

    println!("📊 BULK DATA ANALYSIS DASHBOARD");
    println!("═══════════════════════════════");
    println!("⚠️  Note: Using sample sizes to avoid massive downloads");
    println!();

    // Analyze bulk data capabilities
    analyze_bulk_data_info(&client).await?;

    println!();

    // Sample market data analysis
    analyze_bulk_market_data(&client).await?;

    println!();

    // Sample financial statements analysis
    analyze_bulk_financials(&client).await?;

    println!();

    // Sample institutional holdings analysis
    analyze_bulk_institutional_data(&client).await?;

    println!();

    // ETF holdings analysis
    analyze_bulk_etf_data(&client).await?;

    Ok(())
}

/// Analyze available bulk datasets and their characteristics
async fn analyze_bulk_data_info(client: &FmpClient) -> Result<()> {
    println!("📋 BULK DATA CATALOG ANALYSIS");
    println!("─────────────────────────────");

    let bulk = client.bulk();

    // Get metadata about available datasets
    let data_info = bulk.get_bulk_data_info().await?;
    println!("📊 Found {} bulk datasets", data_info.len());

    let mut total_size_mb = 0.0;
    let mut total_records = 0i64;

    for info in &data_info {
        let dataset_name = info.dataset.as_deref().unwrap_or("Unknown Dataset");
        let size_mb = info
            .file_size
            .map(|s| s as f64 / 1024.0 / 1024.0)
            .unwrap_or(0.0);
        let records = info.record_count.unwrap_or(0);
        let format = info.format.as_deref().unwrap_or("Unknown");
        let updated = info.last_updated.as_deref().unwrap_or("Unknown");

        total_size_mb += size_mb;
        total_records += records;

        println!("  📁 {}", dataset_name);
        println!(
            "     Size: {:.1} MB | Records: {} | Format: {}",
            size_mb, records, format
        );
        println!("     Last Updated: {}", updated);

        if let Some(url) = &info.download_url {
            println!("     Download: {}", url);
        }
        println!();
    }

    println!("📈 BULK DATA SUMMARY:");
    println!("  Total Datasets: {}", data_info.len());
    println!(
        "  Combined Size: {:.1} MB ({:.2} GB)",
        total_size_mb,
        total_size_mb / 1024.0
    );
    println!("  Total Records: {}", total_records);

    // Get historical prices metadata
    let historical_meta = bulk.get_historical_prices_metadata(Some("NYSE")).await?;
    if !historical_meta.is_empty() {
        println!("\n📊 NYSE HISTORICAL DATA:");
        for meta in &historical_meta {
            if let Some(symbols) = meta.symbols_count {
                println!("  Symbols: {}", symbols);
            }
            if let Some(size) = meta.estimated_size_mb {
                println!("  Estimated Size: {:.1} MB", size);
            }
            if let Some(format) = &meta.file_format {
                println!("  Format: {}", format);
            }
        }
    }

    println!("\n💡 BULK DATA INSIGHTS:");
    println!("  • Bulk datasets enable large-scale market analysis");
    println!("  • Consider streaming or chunking for memory efficiency");
    println!("  • Schedule downloads during off-peak hours");
    println!("  • Implement incremental updates to minimize bandwidth");

    Ok(())
}

/// Analyze sample bulk market data
async fn analyze_bulk_market_data(client: &FmpClient) -> Result<()> {
    println!("📈 BULK MARKET DATA ANALYSIS");
    println!("────────────────────────────");

    let bulk = client.bulk();

    // Get sample of bulk stock prices (limit to avoid huge download)
    let sample_size = 100;
    let sample_prices = bulk.get_bulk_prices_sample(sample_size).await?;
    println!(
        "📊 Analyzing sample of {} stock prices",
        sample_prices.len()
    );

    if !sample_prices.is_empty() {
        let mut exchange_stats: HashMap<String, i32> = HashMap::new();
        let mut price_ranges: Vec<f64> = vec![];
        let mut volume_stats: Vec<f64> = vec![];
        let mut market_cap_stats: Vec<f64> = vec![];

        for price in &sample_prices {
            // Analyze by exchange
            if let Some(exchange) = &price.exchange {
                *exchange_stats.entry(exchange.clone()).or_insert(0) += 1;
            }

            // Collect price data
            if let Some(p) = price.price {
                price_ranges.push(p);
            }

            if let Some(vol) = price.volume {
                volume_stats.push(vol as f64);
            }

            if let Some(mcap) = price.market_cap {
                market_cap_stats.push(mcap);
            }

            // Show sample entries
            if price_ranges.len() <= 5 {
                let symbol = price.symbol.as_deref().unwrap_or("N/A");
                let name = price.name.as_deref().unwrap_or("N/A");
                let current_price = price.price.unwrap_or(0.0);
                let change_pct = price.changes_percentage.unwrap_or(0.0);
                let volume = price.volume.unwrap_or(0);

                let change_indicator = if change_pct > 0.0 {
                    "🟢"
                } else if change_pct < 0.0 {
                    "🔴"
                } else {
                    "⚪"
                };

                println!("  {} {} ({})", change_indicator, symbol, name);
                println!(
                    "     Price: ${:.2} | Change: {:.2}% | Volume: {}",
                    current_price, change_pct, volume
                );
            }
        }

        // Calculate statistics
        price_ranges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        volume_stats.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!("\n📊 MARKET DATA STATISTICS:");

        if !price_ranges.is_empty() {
            let min_price = price_ranges.first().unwrap();
            let max_price = price_ranges.last().unwrap();
            let median_price = price_ranges[price_ranges.len() / 2];

            println!("  Price Range: ${:.2} - ${:.2}", min_price, max_price);
            println!("  Median Price: ${:.2}", median_price);
        }

        if !volume_stats.is_empty() {
            let avg_volume = volume_stats.iter().sum::<f64>() / volume_stats.len() as f64;
            println!("  Average Volume: {:.0}", avg_volume);
        }

        println!("\n🏛️ EXCHANGE BREAKDOWN:");
        for (exchange, count) in exchange_stats {
            println!("  {}: {} stocks", exchange, count);
        }

        println!("\n💡 BULK MARKET INSIGHTS:");
        println!("  • Sample represents broader market composition");
        println!("  • Full dataset contains ALL listed securities");
        println!("  • Real-time bulk data enables market-wide analysis");
        println!("  • Use for screening, ranking, and comparative analysis");
    }

    Ok(())
}

/// Analyze sample bulk financial statements
async fn analyze_bulk_financials(client: &FmpClient) -> Result<()> {
    println!("📊 BULK FINANCIALS ANALYSIS");
    println!("───────────────────────────");

    let bulk = client.bulk();

    // Get sample financial statements
    let sample_statements = bulk
        .get_bulk_financials_sample("annual", Some(2023), 20)
        .await?;
    println!(
        "📈 Analyzing {} financial statements",
        sample_statements.len()
    );

    if !sample_statements.is_empty() {
        let mut revenue_data = vec![];
        let mut profit_margins = vec![];
        let mut debt_ratios = vec![];

        for statement in &sample_statements[..10.min(sample_statements.len())] {
            let symbol = statement.symbol.as_deref().unwrap_or("N/A");
            let revenue = statement.revenue.unwrap_or(0.0);
            let net_income = statement.net_income.unwrap_or(0.0);
            let total_debt = statement.total_debt.unwrap_or(0.0);
            let total_assets = statement.total_assets.unwrap_or(0.0);

            revenue_data.push(revenue);

            if revenue > 0.0 {
                profit_margins.push(net_income / revenue * 100.0);
            }

            if total_assets > 0.0 {
                debt_ratios.push(total_debt / total_assets * 100.0);
            }

            println!("  📊 {} Financial Profile:", symbol);
            println!("     Revenue: ${:.0}M", revenue / 1_000_000.0);
            println!("     Net Income: ${:.0}M", net_income / 1_000_000.0);

            if revenue > 0.0 {
                let margin = net_income / revenue * 100.0;
                println!("     Profit Margin: {:.2}%", margin);
            }

            if total_assets > 0.0 {
                let debt_ratio = total_debt / total_assets * 100.0;
                println!("     Debt Ratio: {:.2}%", debt_ratio);
            }
            println!();
        }

        println!("📈 AGGREGATE FINANCIAL METRICS:");

        if !revenue_data.is_empty() {
            let total_revenue: f64 = revenue_data.iter().sum();
            let avg_revenue = total_revenue / revenue_data.len() as f64;
            println!(
                "  Total Sample Revenue: ${:.1}B",
                total_revenue / 1_000_000_000.0
            );
            println!("  Average Revenue: ${:.1}M", avg_revenue / 1_000_000.0);
        }

        if !profit_margins.is_empty() {
            let avg_margin = profit_margins.iter().sum::<f64>() / profit_margins.len() as f64;
            println!("  Average Profit Margin: {:.2}%", avg_margin);
        }

        if !debt_ratios.is_empty() {
            let avg_debt_ratio = debt_ratios.iter().sum::<f64>() / debt_ratios.len() as f64;
            println!("  Average Debt Ratio: {:.2}%", avg_debt_ratio);
        }

        println!("\n💡 BULK FINANCIALS INSIGHTS:");
        println!("  • Bulk financials enable sector-wide analysis");
        println!("  • Compare companies across industries systematically");
        println!("  • Build comprehensive financial screening models");
        println!("  • Track financial health trends across markets");
    }

    Ok(())
}

/// Analyze sample institutional holdings data
async fn analyze_bulk_institutional_data(client: &FmpClient) -> Result<()> {
    println!("🏦 BULK INSTITUTIONAL HOLDINGS ANALYSIS");
    println!("───────────────────────────────────────");

    let bulk = client.bulk();

    // Get sample institutional holdings
    let sample_holdings = bulk
        .get_bulk_institutional_holdings_sample(None, 25)
        .await?;
    println!(
        "🏛️ Analyzing {} institutional holdings",
        sample_holdings.len()
    );

    if !sample_holdings.is_empty() {
        let mut institution_counts: HashMap<String, i32> = HashMap::new();
        let mut holding_values = vec![];
        let mut top_holdings = vec![];

        for holding in &sample_holdings {
            // Count institutions
            if let Some(name) = &holding.name_of_issuer {
                *institution_counts.entry(name.clone()).or_insert(0) += 1;
            }

            // Collect holding values
            if let Some(value) = holding.value {
                holding_values.push(value);

                if let (Some(ticker), Some(name)) =
                    (&holding.ticker_symbol, &holding.name_of_issuer)
                {
                    top_holdings.push((ticker.clone(), name.clone(), value));
                }
            }
        }

        // Sort top holdings by value
        top_holdings.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        println!("🎯 TOP INSTITUTIONAL HOLDINGS (by value):");
        for (ticker, institution, value) in top_holdings.iter().take(8) {
            println!("  📈 {} held by {}", ticker, institution);
            println!("     Value: ${:.1}M", value / 1_000_000.0);
        }

        println!("\n🏦 INSTITUTIONAL ACTIVITY SUMMARY:");
        let total_value: f64 = holding_values.iter().sum();
        let avg_holding = if !holding_values.is_empty() {
            total_value / holding_values.len() as f64
        } else {
            0.0
        };

        println!(
            "  Total Sample Value: ${:.1}B",
            total_value / 1_000_000_000.0
        );
        println!("  Average Holding: ${:.1}M", avg_holding / 1_000_000.0);
        println!("  Unique Institutions: {}", institution_counts.len());

        println!("\n🔍 MOST ACTIVE INSTITUTIONS:");
        let mut sorted_institutions: Vec<_> = institution_counts.iter().collect();
        sorted_institutions.sort_by(|a, b| b.1.cmp(a.1));

        for (institution, count) in sorted_institutions.iter().take(5) {
            println!("  🏛️ {}: {} holdings", institution, count);
        }

        println!("\n💡 INSTITUTIONAL INSIGHTS:");
        println!("  • Institutional ownership indicates professional confidence");
        println!("  • Large institutions often drive market movements");
        println!("  • 13F filings provide transparency into big money flows");
        println!("  • Bulk data reveals institution-wide positioning trends");
    }

    Ok(())
}

/// Analyze sample ETF holdings data
async fn analyze_bulk_etf_data(client: &FmpClient) -> Result<()> {
    println!("📊 BULK ETF HOLDINGS ANALYSIS");
    println!("─────────────────────────────");

    let bulk = client.bulk();

    // Get sample ETF holdings
    let sample_holdings = bulk.get_bulk_etf_holdings_sample(30).await?;
    println!("📈 Analyzing {} ETF holdings", sample_holdings.len());

    if !sample_holdings.is_empty() {
        let mut etf_counts: HashMap<String, i32> = HashMap::new();
        let mut asset_popularity: HashMap<String, i32> = HashMap::new();
        let mut weight_analysis = vec![];

        for holding in &sample_holdings {
            // Count ETFs
            if let Some(etf) = &holding.etf_symbol {
                *etf_counts.entry(etf.clone()).or_insert(0) += 1;
            }

            // Track popular assets
            if let Some(asset) = &holding.asset_symbol {
                *asset_popularity.entry(asset.clone()).or_insert(0) += 1;
            }

            // Collect weights
            if let Some(weight) = holding.weight_percentage {
                weight_analysis.push(weight);
            }
        }

        println!("🎯 ETF COMPOSITION INSIGHTS:");

        // Show sample holdings
        for holding in sample_holdings.iter().take(8) {
            let etf = holding.etf_symbol.as_deref().unwrap_or("N/A");
            let asset = holding.asset_symbol.as_deref().unwrap_or("N/A");
            let asset_name = holding.name.as_deref().unwrap_or("N/A");
            let weight = holding.weight_percentage.unwrap_or(0.0);
            let value = holding.market_value.unwrap_or(0.0);

            println!("  📊 {} holds {} ({})", etf, asset, asset_name);
            println!(
                "     Weight: {:.2}% | Value: ${:.1}M",
                weight,
                value / 1_000_000.0
            );
        }

        println!("\n📈 ETF STATISTICS:");
        println!("  Unique ETFs: {}", etf_counts.len());
        println!("  Unique Assets: {}", asset_popularity.len());

        if !weight_analysis.is_empty() {
            let avg_weight = weight_analysis.iter().sum::<f64>() / weight_analysis.len() as f64;
            let max_weight = weight_analysis.iter().fold(0.0f64, |a, &b| a.max(b));
            println!("  Average Weight: {:.2}%", avg_weight);
            println!("  Max Weight: {:.2}%", max_weight);
        }

        println!("\n🔥 MOST HELD ASSETS:");
        let mut sorted_assets: Vec<_> = asset_popularity.iter().collect();
        sorted_assets.sort_by(|a, b| b.1.cmp(a.1));

        for (asset, count) in sorted_assets.iter().take(6) {
            println!("  📊 {}: held by {} ETFs", asset, count);
        }

        println!("\n💡 ETF INSIGHTS:");
        println!("  • ETF holdings reveal passive investing trends");
        println!("  • Popular holdings indicate market consensus");
        println!("  • Weight distributions show diversification strategies");
        println!("  • Bulk data enables ETF overlap and concentration analysis");
    }

    // Sample earnings estimates
    let sample_estimates = bulk
        .get_bulk_earnings_estimates_sample("quarter", 15)
        .await?;
    if !sample_estimates.is_empty() {
        println!("\n📊 EARNINGS ESTIMATES SAMPLE:");

        let mut estimate_accuracy = vec![];

        for estimate in sample_estimates.iter().take(5) {
            let symbol = estimate.symbol.as_deref().unwrap_or("N/A");
            let date = estimate.date.as_deref().unwrap_or("N/A");
            let eps_avg = estimate.estimated_eps_avg.unwrap_or(0.0);
            let eps_high = estimate.estimated_eps_high.unwrap_or(0.0);
            let eps_low = estimate.estimated_eps_low.unwrap_or(0.0);
            let analysts = estimate.number_analyst_estimated_eps.unwrap_or(0);

            println!("  📈 {} Earnings Forecast ({})", symbol, date);
            println!(
                "     EPS Range: ${:.2} - ${:.2} (Avg: ${:.2})",
                eps_low, eps_high, eps_avg
            );
            println!("     Analysts: {}", analysts);

            if eps_high > eps_low {
                let estimate_spread = ((eps_high - eps_low) / eps_avg * 100.0).abs();
                estimate_accuracy.push(estimate_spread);
                println!("     Estimate Spread: {:.1}%", estimate_spread);
            }
        }

        if !estimate_accuracy.is_empty() {
            let avg_spread = estimate_accuracy.iter().sum::<f64>() / estimate_accuracy.len() as f64;
            println!(
                "\n💡 Average Estimate Spread: {:.1}% (lower = more consensus)",
                avg_spread
            );
        }
    }

    Ok(())
}
