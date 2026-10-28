//! Probe the `/stable` replacements for the dead legacy `/api/v3` and `/api/v4` paths.
//!
//! One read-only GET per candidate (about 16, well inside the free tier's 250/day).
//! The key comes only from `FMP_API_KEY`. Output is the HTTP status and field names
//! with value types: never the key, never a URL, never data values. With an output
//! directory argument, each 2xx JSON body is also saved as a candidate fixture, arrays
//! cut to their first 3 rows; a body containing the key is refused.
//!
//! ```text
//! with-secret FMP_API_KEY -- cargo run --example stable_probe -- <out-dir>
//! ```
//!
//! The candidate paths are unverified guesses from FMP's published `/stable` docs.

use std::process::ExitCode;

use serde_json::Value;

const CANDIDATES: &[&str] = &[
    "etf-list",
    "etf/holdings?symbol=SPY",
    "etf/info?symbol=SPY",
    "etf/sector-weightings?symbol=SPY",
    "etf/country-weightings?symbol=SPY",
    "search-symbol?query=vanguard",
    "insider-trading/search?symbol=AAPL",
    "insider-trading/latest",
    "institutional-ownership/latest",
    "institutional-ownership/extract-analytics/holder?symbol=AAPL&year=2025&quarter=2",
    "institutional-ownership/extract?cik=0001067983&year=2025&quarter=2",
    "institutional-ownership/holder-performance-summary?cik=0001067983",
    "institutional-ownership/symbol-positions-summary?symbol=AAPL&year=2025&quarter=2",
    "fail-to-deliver?symbol=AAPL",
    "cik-list",
    "search-cik?cik=0001067983",
];

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

fn fields(o: &serde_json::Map<String, Value>) -> String {
    o.iter()
        .map(|(k, v)| format!("{k}:{}", kind(v)))
        .collect::<Vec<_>>()
        .join(",")
}

fn shape(v: &Value) -> String {
    match v {
        Value::Array(a) => format!(
            "array of {} [{}]",
            a.len(),
            a.first()
                .and_then(|x| x.as_object())
                .map(fields)
                .unwrap_or_default()
        ),
        Value::Object(o) => format!("object [{}]", fields(o)),
        other => kind(other).to_string(),
    }
}

fn file_name(path: &str) -> String {
    let stem: String = path
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("{stem}.json")
}

#[tokio::main]
async fn main() -> ExitCode {
    let Ok(key) = std::env::var("FMP_API_KEY") else {
        eprintln!("!!! FMP_API_KEY is NOT SET: probe SKIPPED, no request was made !!!");
        return ExitCode::from(2);
    };
    let out_dir = std::env::args().nth(1);
    if let Some(d) = &out_dir {
        if let Err(e) = std::fs::create_dir_all(d) {
            eprintln!("cannot create {d}: {e}");
            return ExitCode::FAILURE;
        }
    }
    let http = reqwest::Client::new();

    for (i, path) in CANDIDATES.iter().enumerate() {
        let url = format!("https://financialmodelingprep.com/stable/{path}");
        let sep = if path.contains('?') { '&' } else { '?' };
        let resp = http.get(format!("{url}{sep}apikey={key}")).send().await;
        let n = i + 1;
        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                println!("[{n}] {path}: ERR {}", e.without_url());
                continue;
            }
        };
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if body.contains(&key) {
            println!("[{n}] {path}: HTTP {status}, body contained the key: REFUSED");
            continue;
        }
        match serde_json::from_str::<Value>(&body) {
            Ok(mut v) if status.is_success() => {
                println!("[{n}] {path}: HTTP {status}, {}", shape(&v));
                if let (Some(d), Value::Array(a)) = (&out_dir, &mut v) {
                    a.truncate(3);
                    let text = serde_json::to_string_pretty(&v).unwrap_or_default();
                    let _ = std::fs::write(
                        format!("{d}/{}", file_name(path)),
                        text + "
",
                    );
                }
            }
            Ok(v) => println!("[{n}] {path}: HTTP {status}, {}", shape(&v)),
            Err(_) => println!(
                "[{n}] {path}: HTTP {status}, not JSON, {} bytes",
                body.len()
            ),
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    ExitCode::SUCCESS
}
