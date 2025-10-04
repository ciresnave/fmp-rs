//! Example demonstrating news and analyst data retrieval

use fmp_rs::{FmpClient, models::common::Period};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable FMP_API_KEY
    let client = FmpClient::new()?;

    let symbol = "AAPL";
    println!("=== News and Analyst Data for {} ===\n", symbol);

    // Get latest news for the symbol
    println!("📰 Latest News:");
    match client
        .news()
        .get_symbol_news(symbol, Some(0), Some(5))
        .await
    {
        Ok(news) => {
            for article in news {
                println!("  • {}", article.title);
                println!("    Published: {}", article.published_date);
                println!("    Site: {}", article.site);
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching news: {}", e),
    }

    // Get analyst estimates
    println!("\n📊 Analyst Estimates (Annual):");
    match client
        .analyst()
        .get_estimates(symbol, Some(Period::Annual), Some(3))
        .await
    {
        Ok(estimates) => {
            for estimate in estimates {
                println!("  Date: {}", estimate.date);
                println!(
                    "    Estimated EPS: ${:.2} (Low: ${:.2}, High: ${:.2})",
                    estimate.estimated_eps_avg,
                    estimate.estimated_eps_low,
                    estimate.estimated_eps_high
                );
                println!(
                    "    Number of Analysts: {}",
                    estimate.number_analysts_estimated_eps
                );
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching estimates: {}", e),
    }

    // Get price targets
    println!("\n🎯 Recent Price Targets:");
    match client.analyst().get_price_targets(symbol).await {
        Ok(targets) => {
            for (i, target) in targets.iter().take(5).enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    target.analyst_company,
                    target.published_date
                );
                println!(
                    "     Price Target: ${:.2} (Adj: ${:.2})",
                    target.price_target, target.adj_price_target
                );
                println!("     Analyst: {}", target.analyst_name);
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching price targets: {}", e),
    }

    // Get analyst grades/recommendations
    println!("\n⭐ Recent Analyst Ratings:");
    match client.analyst().get_grades(symbol).await {
        Ok(grades) => {
            for (i, grade) in grades.iter().take(10).enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    grade.grading_company,
                    grade.published_date
                );
                if let Some(prev) = &grade.previous_grade {
                    println!(
                        "     {} → {} ({})",
                        prev, grade.new_grade, grade.grade_action
                    );
                } else {
                    println!("     {} ({})", grade.new_grade, grade.grade_action);
                }
            }
        }
        Err(e) => eprintln!("Error fetching grades: {}", e),
    }

    // Get recommendation consensus
    println!("\n📈 Recommendation Consensus:");
    match client.analyst().get_recommendation_consensus(symbol).await {
        Ok(consensus) => {
            for rec in consensus {
                println!("  Symbol: {}", rec.symbol);
                println!(
                    "  Strong Buy: {} | Buy: {} | Hold: {} | Sell: {} | Strong Sell: {}",
                    rec.strong_buy, rec.buy, rec.hold, rec.sell, rec.strong_sell
                );
                println!("  Consensus: {}", rec.consensus);
            }
        }
        Err(e) => eprintln!("Error fetching consensus: {}", e),
    }

    // Get general stock news
    println!("\n\n📰 General Stock Market News:");
    match client.news().get_general_news(Some(0), Some(5)).await {
        Ok(news) => {
            for article in news {
                println!("  • {}", article.title);
                println!("    {}", article.published_date);
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching general news: {}", e),
    }

    // Get crypto news
    println!("\n💰 Crypto News:");
    match client.news().get_crypto_news(Some(0), Some(3)).await {
        Ok(news) => {
            for article in news {
                println!("  • {}", article.title);
                if let Some(sym) = article.symbol {
                    println!("    Symbol: {}", sym);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching crypto news: {}", e),
    }

    Ok(())
}
