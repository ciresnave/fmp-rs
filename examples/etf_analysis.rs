//! ETF analysis example
//!
//! Every ETF endpoint needs a paid FMP plan (a free key gets `Error::PlanRestricted`).
//! Rows are raw JSON because no response has been recorded yet.
//!
//! To run: cargo run --example etf_analysis
//!
//! Make sure the FMP_API_KEY environment variable is set.

use fmp_rs::FmpClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = FmpClient::new()?;
    let symbol = "SPY"; // S&P 500 ETF

    let etfs = client.etf().get_etf_list().await?;
    println!("{} ETFs listed", etfs.len());

    for (label, rows) in [
        ("info", client.etf().get_etf_info(symbol).await?),
        ("holdings", client.etf().get_etf_holdings(symbol).await?),
        (
            "sector weightings",
            client.etf().get_etf_sector_weighting(symbol).await?,
        ),
        (
            "country weightings",
            client.etf().get_etf_country_weighting(symbol).await?,
        ),
    ] {
        println!("\n{symbol} {label}: {} row(s)", rows.len());
        for row in rows.iter().take(3) {
            println!("  {row}");
        }
    }
    Ok(())
}
