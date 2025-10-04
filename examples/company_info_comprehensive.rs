//! Comprehensive example demonstrating company information endpoints

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable FMP_API_KEY
    let client = FmpClient::new()?;

    let symbol = "AAPL";
    println!("=== Comprehensive Company Information for {} ===\n", symbol);

    // Get company profile
    println!("📋 Company Profile:");
    match client.company_info().get_profile(symbol).await {
        Ok(profiles) => {
            if let Some(profile) = profiles.first() {
                println!("  Company: {}", profile.company_name);
                println!(
                    "  Symbol: {} | Exchange: {}",
                    profile.symbol, profile.exchange
                );
                println!(
                    "  Sector: {} | Industry: {}",
                    profile.sector, profile.industry
                );
                println!("  CEO: {}", profile.ceo);
                println!("  Website: {}", profile.website);
                println!("  Employees: {}", profile.full_time_employees);
                println!(
                    "  Market Cap: ${:.2}B",
                    profile.mkt_cap as f64 / 1_000_000_000.0
                );
                println!("  Current Price: ${:.2}", profile.price);
                println!("  Beta: {:.2}", profile.beta);
                println!("  IPO Date: {}", profile.ipo_date);
                println!(
                    "  Country: {} | City: {}, {}",
                    profile.country, profile.city, profile.state
                );
            }
        }
        Err(e) => eprintln!("Error fetching profile: {}", e),
    }

    // Get comprehensive company outlook
    println!("\n🔍 Company Outlook:");
    match client.company_info().get_outlook(symbol).await {
        Ok(outlook) => {
            println!("  Company: {}", outlook.profile.company_name);
            println!(
                "  Description: {}",
                &outlook.profile.description[..200.min(outlook.profile.description.len())]
            );

            if let Some(rating) = &outlook.rating {
                if let Some(r) = rating.first() {
                    println!("\n  Current Rating:");
                    println!("    Overall: {} (Score: {})", r.rating, r.rating_score);
                    println!("    Recommendation: {}", r.rating_recommendation);
                    println!(
                        "    DCF Score: {} ({})",
                        r.rating_details_dcf_score, r.rating_details_dcf_recommendation
                    );
                    println!(
                        "    ROE Score: {} ({})",
                        r.rating_details_roe_score, r.rating_details_roe_recommendation
                    );
                    println!(
                        "    P/E Score: {} ({})",
                        r.rating_details_pe_score, r.rating_details_pe_recommendation
                    );
                }
            }

            if let Some(executives) = &outlook.key_executives {
                println!("\n  Key Executives:");
                for exec in executives.iter().take(5) {
                    if let Some(pay) = exec.pay {
                        println!("    {}: {} (${:.0})", exec.title, exec.name, pay);
                    } else {
                        println!("    {}: {}", exec.title, exec.name);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error fetching outlook: {}", e),
    }

    // Get company rating
    println!("\n⭐ Company Rating:");
    match client.company_info().get_rating(symbol).await {
        Ok(ratings) => {
            for rating in ratings.iter().take(1) {
                println!("  Date: {}", rating.date);
                println!(
                    "  Rating: {} (Score: {})",
                    rating.rating, rating.rating_score
                );
                println!("  Recommendation: {}", rating.rating_recommendation);
                println!("\n  Detailed Scores:");
                println!(
                    "    DCF: {} - {}",
                    rating.rating_details_dcf_score, rating.rating_details_dcf_recommendation
                );
                println!(
                    "    ROE: {} - {}",
                    rating.rating_details_roe_score, rating.rating_details_roe_recommendation
                );
                println!(
                    "    ROA: {} - {}",
                    rating.rating_details_roa_score, rating.rating_details_roa_recommendation
                );
                println!(
                    "    D/E: {} - {}",
                    rating.rating_details_de_score, rating.rating_details_de_recommendation
                );
                println!(
                    "    P/E: {} - {}",
                    rating.rating_details_pe_score, rating.rating_details_pe_recommendation
                );
                println!(
                    "    P/B: {} - {}",
                    rating.rating_details_pb_score, rating.rating_details_pb_recommendation
                );
            }
        }
        Err(e) => eprintln!("Error fetching rating: {}", e),
    }

    // Get historical rating
    println!("\n📈 Historical Rating Trend (Last 5):");
    match client
        .company_info()
        .get_historical_rating(symbol, Some(5))
        .await
    {
        Ok(ratings) => {
            for rating in ratings {
                println!(
                    "  {}: {} (Score: {}) - {}",
                    rating.date, rating.rating, rating.rating_score, rating.rating_recommendation
                );
            }
        }
        Err(e) => eprintln!("Error fetching historical rating: {}", e),
    }

    // Get key executives
    println!("\n👔 Key Executives:");
    match client.company_info().get_executives(symbol).await {
        Ok(executives) => {
            for exec in executives.iter().take(8) {
                let pay_info = if let Some(pay) = exec.pay {
                    format!("(${:.0})", pay)
                } else {
                    String::new()
                };
                let gender_info = exec
                    .gender
                    .as_ref()
                    .map(|g| format!(" [{}]", g))
                    .unwrap_or_default();
                println!(
                    "  {}: {}{} {}",
                    exec.title, exec.name, gender_info, pay_info
                );
            }
        }
        Err(e) => eprintln!("Error fetching executives: {}", e),
    }

    // Get market cap
    println!("\n💰 Market Capitalization:");
    match client.company_info().get_market_cap(symbol).await {
        Ok(market_caps) => {
            for cap in market_caps.iter().take(1) {
                println!("  Symbol: {}", cap.symbol);
                println!("  Date: {}", cap.date);
                println!(
                    "  Market Cap: ${:.2}B",
                    cap.market_cap as f64 / 1_000_000_000.0
                );
            }
        }
        Err(e) => eprintln!("Error fetching market cap: {}", e),
    }

    // Get share float
    println!("\n📊 Share Float:");
    match client.company_info().get_share_float(symbol).await {
        Ok(floats) => {
            for float in floats.iter().take(1) {
                println!("  Symbol: {}", float.symbol);
                println!("  Date: {}", float.date);
                println!("  Outstanding Shares: {:.0}", float.outstanding_shares);
                println!("  Float Shares: {:.0}", float.float_shares);
                println!("  Free Float: {:.2}%", float.free_float * 100.0);
            }
        }
        Err(e) => eprintln!("Error fetching share float: {}", e),
    }

    // Get stock peers
    println!("\n🤝 Stock Peers (Similar Companies):");
    match client.company_info().get_peers(symbol).await {
        Ok(peers_list) => {
            for peers in peers_list {
                println!("  {}'s peers:", peers.symbol);
                for (i, peer) in peers.peers.iter().enumerate() {
                    print!("    {}", peer);
                    if (i + 1) % 5 == 0 {
                        println!();
                    } else {
                        print!(" | ");
                    }
                }
                println!();
            }
        }
        Err(e) => eprintln!("Error fetching peers: {}", e),
    }

    // Get company core information
    println!("\n🔑 Core Information:");
    match client.company_info().get_core_info(symbol).await {
        Ok(core_infos) => {
            for info in core_infos {
                println!("  CIK: {}", info.cik);
                println!("  Symbol: {}", info.symbol);
                println!("  Exchange: {}", info.exchange);
            }
        }
        Err(e) => eprintln!("Error fetching core info: {}", e),
    }

    Ok(())
}
