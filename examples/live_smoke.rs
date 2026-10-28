//! Live smoke test against the real FMP API: six read-only requests, well under
//! the free tier's 250 requests/day.
//!
//! The key comes only from `FMP_API_KEY` (supply it with the vault tooling).
//! Output is shapes and statuses only: never the key, never a URL, never data
//! values. Any error text has the key scrubbed. With no key set it prints a loud
//! message and exits 2 without making a request.
//!
//! ```text
//! with-secret FMP_API_KEY -- cargo run --example live_smoke
//! ```
//!
//! Each line is `[n] label: result`. Expected results:
//!
//! 1. `quote AAPL (typed)`: `OK, 1 item`
//! 2. `profile AAPL (typed)`: `OK, 1 item`
//! 3. `cot list (typed)`: `OK, N items` (N > 0), or `ERR ...` if the typed model is wrong
//! 4. `cot report KC (typed)`: `OK, N items`, or `ERR ...` (params symbol/from/to unconfirmed)
//! 5. `etf list via crate (/stable/api/v3 path)`: expected `ERR` (the crate joins a
//!    legacy `/api/v3` path onto the `/stable` base; a 404 or 403 confirms ~25 endpoints are dead)
//! 6. `legacy /api/v3/quote/AAPL (raw)`: `HTTP 403` or similar (legacy endpoints are
//!    closed to keys created after 2025-08-31); `HTTP 200` would mean legacy still works
//!
//! Raw calls print `HTTP <status>, JSON <array of N | object>` plus the first
//! element's field names with value types.

use std::process::ExitCode;

use serde_json::Value;

fn scrub(text: &str, key: &str) -> String {
    text.replace(key, "***")
}

fn shape(body: &str) -> String {
    match serde_json::from_str::<Value>(body) {
        Ok(Value::Array(a)) => {
            let fields = a
                .first()
                .and_then(|v| v.as_object())
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| format!("{k}:{}", kind(v)))
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .unwrap_or_default();
            format!("JSON array of {} [{fields}]", a.len())
        }
        Ok(Value::Object(o)) => {
            let fields = o
                .iter()
                .map(|(k, v)| format!("{k}:{}", kind(v)))
                .collect::<Vec<_>>()
                .join(",");
            format!("JSON object [{fields}]")
        }
        Ok(other) => format!("JSON {}", kind(&other)),
        Err(_) => "not JSON".to_string(),
    }
}

fn kind(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "num",
        Value::String(_) => "str",
        Value::Array(_) => "arr",
        Value::Object(_) => "obj",
    }
}

fn report<T>(n: u8, label: &str, result: fmp_rs::Result<Vec<T>>, key: &str) -> bool {
    match result {
        Ok(items) => {
            println!("[{n}] {label}: OK, {} item(s)", items.len());
            true
        }
        Err(e) => {
            println!("[{n}] {label}: ERR {}", scrub(&e.to_string(), key));
            false
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let Ok(key) = std::env::var("FMP_API_KEY") else {
        eprintln!("!!! FMP_API_KEY is NOT SET: live smoke test SKIPPED, no request was made !!!");
        return ExitCode::from(2);
    };
    let client = match fmp_rs::FmpClient::with_api_key(key.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cannot build client: {}", scrub(&e.to_string(), &key));
            return ExitCode::FAILURE;
        }
    };

    let mut ok = 0;
    ok += report(
        1,
        "quote AAPL (typed)",
        client.quote().get_quote("AAPL").await,
        &key,
    ) as u8;
    ok += report(
        2,
        "profile AAPL (typed)",
        client.company_info().get_profile("AAPL").await,
        &key,
    ) as u8;
    ok += report(
        3,
        "cot list (typed)",
        client.cot().get_report_list().await,
        &key,
    ) as u8;
    ok += report(
        4,
        "cot report KC (typed)",
        client
            .cot()
            .get_report("KC", Some("2026-09-01"), None)
            .await,
        &key,
    ) as u8;
    ok += report(
        5,
        "etf list via crate (/stable/api/v3 path)",
        client.etf().get_etf_list().await,
        &key,
    ) as u8;

    // 6: raw legacy path, against the host root (not /stable).
    let legacy = reqwest::Client::new()
        .get("https://financialmodelingprep.com/api/v3/quote/AAPL")
        .query(&[("apikey", key.as_str())])
        .send()
        .await;
    match legacy {
        Ok(r) => {
            let status = r.status();
            let body = r.text().await.unwrap_or_default();
            let body = scrub(&body, &key);
            println!(
                "[6] legacy /api/v3/quote/AAPL (raw): HTTP {status}, {}",
                shape(&body)
            );
        }
        Err(e) => println!(
            "[6] legacy /api/v3/quote/AAPL (raw): ERR {}",
            scrub(&e.without_url().to_string(), &key)
        ),
    }

    println!("typed calls OK: {ok}/5 (call 5 is expected to fail)");
    ExitCode::SUCCESS
}
