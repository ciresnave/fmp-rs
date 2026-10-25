//! Record one real FMP response as a test fixture.
//!
//! The key comes only from the `FMP_API_KEY` environment variable (supply it
//! with the vault tooling, never a file in this repo). It is sent to FMP and is
//! never written: the fixture holds the response body only, and the run aborts
//! if that body contains the key.
//!
//! ```text
//! cargo run --example record_fixture -- /commitment-of-traders-list tests/fixtures/cot_list.json
//! cargo run --example record_fixture -- /commitment-of-traders-report tests/fixtures/cot_report.json symbol=KC from=2026-09-01
//! ```

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: record_fixture <path> <out-file> [key=value ...]");
        return ExitCode::from(2);
    }
    let Ok(key) = std::env::var("FMP_API_KEY") else {
        eprintln!("FMP_API_KEY is not set");
        return ExitCode::from(2);
    };
    let mut query: Vec<(String, String)> = Vec::new();
    for kv in &args[2..] {
        let Some((k, v)) = kv.split_once('=') else {
            eprintln!("bad query argument {kv:?}, expected key=value");
            return ExitCode::from(2);
        };
        query.push((k.to_string(), v.to_string()));
    }
    query.push(("apikey".to_string(), key.clone()));

    let url = format!(
        "{}/{}",
        fmp_rs::client::FMP_API_BASE_URL,
        args[0].trim_start_matches('/')
    );
    let response = match reqwest::Client::new().get(url).query(&query).send().await {
        Ok(r) => r,
        Err(e) => {
            // reqwest errors can embed the request URL, which carries the key.
            eprintln!("request failed: {}", e.to_string().replace(&key, "***"));
            return ExitCode::FAILURE;
        }
    };
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        eprintln!("HTTP {status}; nothing written");
        return ExitCode::FAILURE;
    }
    if body.contains(&key) {
        eprintln!("response body contains the API key; nothing written");
        return ExitCode::FAILURE;
    }
    if serde_json::from_str::<serde_json::Value>(&body).is_err() {
        eprintln!("response is not JSON; nothing written");
        return ExitCode::FAILURE;
    }
    if let Err(e) = std::fs::write(&args[1], &body) {
        eprintln!("cannot write {}: {e}", args[1]);
        return ExitCode::FAILURE;
    }
    println!("wrote {} ({} bytes, HTTP {status})", args[1], body.len());
    ExitCode::SUCCESS
}
