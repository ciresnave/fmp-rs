//! ESG & Social Impact Dashboard
//!
//! This example demonstrates ESG (Environmental, Social, Governance) analysis,
//! social sentiment tracking, and Congressional trading monitoring.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("        🌱 ESG & SOCIAL IMPACT DASHBOARD 🌱");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. ESG Scores Analysis
    println!("🌍 ESG SCORES ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    // Analyze ESG scores for different companies
    let esg_companies = ["AAPL", "TSLA", "MSFT", "XOM", "JPM"];
    let company_names = ["Apple", "Tesla", "Microsoft", "ExxonMobil", "JPMorgan"];

    println!("  Comparing ESG Performance:");
    println!();

    for (symbol, name) in esg_companies.iter().zip(company_names.iter()) {
        let esg_scores = client.esg().get_esg_score(symbol).await?;
        if let Some(score) = esg_scores.first() {
            let esg = score.esg_score.unwrap_or(0.0);
            let env = score.environment_score.unwrap_or(0.0);
            let social = score.social_score.unwrap_or(0.0);
            let gov = score.governance_score.unwrap_or(0.0);

            // ESG rating interpretation
            let esg_rating = if esg >= 80.0 {
                "🟢 Excellent"
            } else if esg >= 60.0 {
                "🟡 Good"
            } else if esg >= 40.0 {
                "🟠 Fair"
            } else {
                "🔴 Poor"
            };

            println!("    {} ({}):", name, symbol);
            println!("      Overall ESG: {:.1}/100 {}", esg, esg_rating);
            println!(
                "      Environmental: {:.1}  Social: {:.1}  Governance: {:.1}",
                env, social, gov
            );

            // Create visual bars for scores
            let env_bar = "█".repeat((env / 10.0) as usize);
            let soc_bar = "█".repeat((social / 10.0) as usize);
            let gov_bar = "█".repeat((gov / 10.0) as usize);
            println!("      E: {} S: {} G: {}", env_bar, soc_bar, gov_bar);
            println!();
        }
    }

    // 2. ESG Risk Analysis
    println!("⚠️  ESG RISK ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    let risk_symbols = ["TSLA", "XOM"]; // High-profile companies
    for symbol in &risk_symbols {
        let esg_risk = client.esg().get_esg_risk_rating(symbol).await?;
        if let Some(risk) = esg_risk.first() {
            let risk_score = risk.esg_risk_score.unwrap_or(0.0);
            let risk_level = risk.esg_risk_level.as_deref().unwrap_or("Unknown");
            let percentile = risk.esg_risk_percentile.unwrap_or(0.0);

            println!(
                "  {} ({}):",
                risk.company_name.as_deref().unwrap_or("N/A"),
                symbol
            );
            println!("    Risk Score: {:.1}", risk_score);
            println!("    Risk Level: {}", risk_level);
            println!("    Industry Percentile: {:.1}%", percentile);

            if let Some(env_risk) = risk.environment_risk_score {
                println!("    Environmental Risk: {:.1}", env_risk);
            }
            if let Some(social_risk) = risk.social_risk_score {
                println!("    Social Risk: {:.1}", social_risk);
            }
            if let Some(gov_risk) = risk.governance_risk_score {
                println!("    Governance Risk: {:.1}", gov_risk);
            }
            println!();
        }
    }

    // 3. Industry ESG Benchmarks
    println!("📊 INDUSTRY ESG BENCHMARKS");
    println!("─────────────────────────────────────────────────────────");
    let benchmarks = client.esg().get_esg_sector_benchmark(Some("2023")).await?;

    println!("  Top ESG Industries (2023):");
    println!();

    // Sort benchmarks by ESG score
    let mut sorted_benchmarks = benchmarks.clone();
    sorted_benchmarks.sort_by(|a, b| {
        b.esg_benchmark
            .unwrap_or(0.0)
            .partial_cmp(&a.esg_benchmark.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for benchmark in sorted_benchmarks.iter().take(8) {
        if let Some(industry) = &benchmark.industry {
            let esg_bench = benchmark.esg_benchmark.unwrap_or(0.0);
            let env_bench = benchmark.environment_benchmark.unwrap_or(0.0);
            let soc_bench = benchmark.social_benchmark.unwrap_or(0.0);
            let gov_bench = benchmark.governance_benchmark.unwrap_or(0.0);

            let bar = "█".repeat((esg_bench / 5.0) as usize);
            println!(
                "    {:25} ESG: {:4.1} {} (E:{:.1} S:{:.1} G:{:.1})",
                industry, esg_bench, bar, env_bench, soc_bench, gov_bench
            );
        }
    }
    println!();

    // 4. Congressional Trading Activity
    println!("🏛️  CONGRESSIONAL TRADING ACTIVITY");
    println!("─────────────────────────────────────────────────────────");

    // Get recent Senate trading
    println!("  Recent Senate Trading Activity:");
    let senate_trades = client.esg().get_senate_trading(None).await?;
    if !senate_trades.is_empty() {
        println!();
        for trade in senate_trades.iter().take(10) {
            let date = trade.transaction_date.as_deref().unwrap_or("N/A");
            let representative = trade.representative.as_deref().unwrap_or("N/A");
            let transaction_type = trade.transaction.as_deref().unwrap_or("N/A");
            let ticker = trade.ticker.as_deref().unwrap_or("N/A");
            let amount = trade.amount.as_deref().unwrap_or("N/A");
            let party = trade.party.as_deref().unwrap_or("N/A");

            let transaction_emoji =
                if transaction_type.contains("Purchase") || transaction_type.contains("Buy") {
                    "🟢 BUY"
                } else if transaction_type.contains("Sale") || transaction_type.contains("Sell") {
                    "🔴 SELL"
                } else {
                    "🟡 OTHER"
                };

            println!(
                "    {} | {} {} {} | {} ({}) | {}",
                date, transaction_emoji, ticker, amount, representative, party, transaction_type
            );
        }
    } else {
        println!("    No recent Senate trading data available");
    }
    println!();

    // Get House trading for specific stock
    println!("  House Trading - Apple (AAPL):");
    let house_aapl = client.esg().get_house_disclosure(Some("AAPL")).await?;
    if !house_aapl.is_empty() {
        println!();
        for trade in house_aapl.iter().take(5) {
            let date = trade.transaction_date.as_deref().unwrap_or("N/A");
            let representative = trade.representative.as_deref().unwrap_or("N/A");
            let transaction_type = trade.transaction.as_deref().unwrap_or("N/A");
            let amount = trade.amount.as_deref().unwrap_or("N/A");

            println!(
                "    {} | {} | {} | {}",
                date, representative, transaction_type, amount
            );
        }
    } else {
        println!("    No recent House AAPL trading data available");
    }
    println!();

    // 5. Social Sentiment Analysis
    println!("📱 SOCIAL SENTIMENT ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    let sentiment_symbols = ["AAPL", "TSLA", "GME", "AMC"];
    for symbol in &sentiment_symbols {
        let sentiment = client.esg().get_social_sentiment(symbol).await?;
        if !sentiment.is_empty() {
            println!("  {} Social Sentiment:", symbol);

            for data in sentiment.iter().take(5) {
                let general = data.general_sentiment.as_deref().unwrap_or("Neutral");
                let twitter_sent = data.twitter_sentiment.unwrap_or(0.0);
                let stocktwits_sent = data.stocktwits_sentiment.unwrap_or(0.0);
                let twitter_posts = data.twitter_posts.unwrap_or(0);
                let stocktwits_posts = data.stocktwits_posts.unwrap_or(0);

                // Sentiment interpretation
                let sentiment_emoji = match general {
                    "Bullish" => "🟢 📈",
                    "Bearish" => "🔴 📉",
                    _ => "🟡 ➡️",
                };

                println!(
                    "    {} | {} {} | Twitter: {:.2} ({} posts) | StockTwits: {:.2} ({} posts)",
                    data.date,
                    sentiment_emoji,
                    general,
                    twitter_sent,
                    twitter_posts,
                    stocktwits_sent,
                    stocktwits_posts
                );
            }
            println!();
        } else {
            println!("  {}: No sentiment data available", symbol);
        }
    }

    // 6. ESG Investment Insights
    println!("💡 ESG INVESTMENT INSIGHTS");
    println!("─────────────────────────────────────────────────────────");
    println!("  🌱 Sustainable Investing Trends:");
    println!("     • ESG funds have grown 300%+ in recent years");
    println!("     • Companies with high ESG scores often show lower volatility");
    println!("     • Environmental factors increasingly impact valuations");
    println!("     • Social responsibility affects consumer preferences");
    println!("     • Good governance reduces regulatory and legal risks");
    println!();
    println!("  📊 Key ESG Considerations:");
    println!("     • Environmental: Carbon footprint, renewable energy, waste management");
    println!("     • Social: Employee relations, community impact, product safety");
    println!("     • Governance: Board diversity, executive compensation, transparency");
    println!();
    println!("  🏛️  Political Trading Insights:");
    println!("     • Congressional trades must be disclosed within 45 days");
    println!("     • Large trades often indicate policy knowledge or economic outlook");
    println!("     • Bipartisan trading patterns may signal broad market trends");
    println!("     • Sector-specific trades may indicate regulatory changes");
    println!();
    println!("  📱 Social Sentiment Impact:");
    println!("     • Social media can drive short-term price volatility");
    println!("     • Retail investor sentiment affects meme stocks");
    println!("     • Negative sentiment may indicate reputation risks");
    println!("     • Positive sentiment can create momentum in growth stocks");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("⚠️  DISCLAIMER: ESG scores and social sentiment are");
    println!("supplementary tools. Combine with fundamental analysis");
    println!("and consider your investment goals and risk tolerance.");
    println!("Congressional trading data is for informational purposes only.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
