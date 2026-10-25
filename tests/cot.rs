//! COT endpoints against a local mock server (no network, no API key).
//! Fixtures are synthetic: see tests/fixtures/README.md.

use fmp_rs::FmpClient;
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
async fn report_sends_symbol_dates_and_key_and_parses_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/commitment-of-traders-report"))
        .and(query_param("symbol", "KC"))
        .and(query_param("from", "2026-10-01"))
        .and(query_param("to", "2026-10-20"))
        .and(query_param("apikey", "test_key"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(fixture("cot_report.synthetic.json")),
        )
        .expect(1)
        .mount(&server)
        .await;

    let rows = client(&server)
        .await
        .cot()
        .get_report("KC", Some("2026-10-01"), Some("2026-10-20"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].symbol.as_deref(), Some("KC"));
    assert_eq!(rows[0].date.as_deref(), Some("2026-10-13"));
    assert_eq!(rows[1].number("openInterest"), Some(990.5));
}

#[tokio::test]
async fn analysis_hits_its_own_path_and_omits_unset_dates() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/commitment-of-traders-analysis"))
        .and(query_param("symbol", "KC"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(fixture("cot_report.synthetic.json")),
        )
        .expect(1)
        .mount(&server)
        .await;

    let rows = client(&server)
        .await
        .cot()
        .get_analysis("KC", None, None)
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    let req = &server.received_requests().await.unwrap()[0];
    let q = req.url.query().unwrap();
    assert!(!q.contains("from=") && !q.contains("to="), "query was {q}");
}

#[tokio::test]
async fn list_parses_and_http_error_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/commitment-of-traders-list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture("cot_list.synthetic.json")))
        .mount(&server)
        .await;
    let items = client(&server).await.cot().get_report_list().await.unwrap();
    assert_eq!(items[0].name.as_deref(), Some("Coffee C"));

    let bad = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&bad)
        .await;
    assert!(client(&bad).await.cot().get_report_list().await.is_err());
}
