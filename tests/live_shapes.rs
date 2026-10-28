//! Shapes and errors seen on the live /stable API (2026-10-28 smoke run, free-tier key).
//! Quote and profile fixtures are the recorded AAPL responses (public company data).

use fmp_rs::{Error, FmpClient};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client(server: &MockServer) -> FmpClient {
    FmpClient::builder()
        .api_key("test_key")
        .base_url(server.uri())
        .build()
        .unwrap()
}

fn fixture(name: &str) -> serde_json::Value {
    let p = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

#[tokio::test]
async fn quote_parses_the_live_shape() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/quote"))
        .and(query_param("symbol", "AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture("quote_aapl.live.json")))
        .expect(1)
        .mount(&server)
        .await;

    let quotes = client(&server)
        .await
        .quote()
        .get_quote("AAPL")
        .await
        .unwrap();
    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].symbol, "AAPL");
    assert_eq!(quotes[0].changes_percentage, -0.79084);
    assert_eq!(quotes[0].avg_volume, None);
}

#[tokio::test]
async fn profile_parses_the_live_shape() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/profile"))
        .and(query_param("symbol", "AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture("profile_aapl.live.json")))
        .expect(1)
        .mount(&server)
        .await;

    let profiles = client(&server)
        .await
        .company_info()
        .get_profile("AAPL")
        .await
        .unwrap();
    assert_eq!(profiles.len(), 1);
    assert_eq!(profiles[0].company_name, "Apple Inc.");
    assert_eq!(profiles[0].mkt_cap, 5025578853900);
    assert_eq!(profiles[0].exchange_short_name, "NASDAQ");
    assert_eq!(profiles[0].dcf, None);
}

#[tokio::test]
async fn http_402_is_a_typed_plan_restricted_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/commitment-of-traders-list"))
        .respond_with(ResponseTemplate::new(402).set_body_string(
            "Restricted Endpoint: This endpoint is not available under your current subscription",
        ))
        .mount(&server)
        .await;

    let err = client(&server)
        .await
        .cot()
        .get_report_list()
        .await
        .unwrap_err();
    assert!(err.is_plan_restricted(), "got {err:?}");
    assert!(matches!(err, Error::PlanRestricted { .. }));
    assert!(err.to_string().contains("subscription"));
}

#[tokio::test]
async fn legacy_path_endpoints_build_a_url_and_send_the_key() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/etf/list"))
        .and(query_param("apikey", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;

    let list = client(&server).await.etf().get_etf_list().await.unwrap();
    assert!(list.is_empty());
}
