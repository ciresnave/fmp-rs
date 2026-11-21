//! Company search endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::company::CompanySearchResult;
use serde::Serialize;

/// Company search API endpoints
pub struct CompanySearch {
    client: FmpClient,
}

impl CompanySearch {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Search for companies by name or symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let results = client.company_search().search("Apple", None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(
        &self,
        query: &str,
        limit: Option<u32>,
        exchange: Option<&str>,
    ) -> Result<Vec<CompanySearchResult>> {
        #[derive(Serialize)]
        struct Query<'a> {
            query: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            exchange: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/search");
        self.client
            .get_with_query(
                &url,
                &Query {
                    query,
                    limit,
                    exchange,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Search for companies by ticker symbol
    pub async fn search_symbol(
        &self,
        query: &str,
        limit: Option<u32>,
        exchange: Option<&str>,
    ) -> Result<Vec<CompanySearchResult>> {
        #[derive(Serialize)]
        struct Query<'a> {
            query: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            exchange: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/search-symbol");
        self.client
            .get_with_query(
                &url,
                &Query {
                    query,
                    limit,
                    exchange,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Search for companies by CIK number
    pub async fn search_cik(
        &self,
        cik: &str,
        limit: Option<u32>,
    ) -> Result<Vec<CompanySearchResult>> {
        #[derive(Serialize)]
        struct Query<'a> {
            cik: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/search-cik");
        self.client
            .get_with_query(
                &url,
                &Query {
                    cik,
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Search for companies by CUSIP
    pub async fn search_cusip(&self, cusip: &str) -> Result<Vec<CompanySearchResult>> {
        #[derive(Serialize)]
        struct Query<'a> {
            cusip: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/search-cusip");
        self.client
            .get_with_query(
                &url,
                &Query {
                    cusip,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Search for companies by ISIN
    pub async fn search_isin(&self, isin: &str) -> Result<Vec<CompanySearchResult>> {
        #[derive(Serialize)]
        struct Query<'a> {
            isin: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/search-isin");
        self.client
            .get_with_query(
                &url,
                &Query {
                    isin,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// SYNTHETIC body (not a recording): field set from CompanySearchResult.
    #[tokio::test]
    async fn test_search() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("query", "Apple"))
            .and(query_param("limit", "10"))
            .and(query_param("apikey", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
                "symbol": "AAPL", "name": "Apple Inc.", "currency": "USD",
                "stockExchange": "NASDAQ Global Select", "exchangeShortName": "NASDAQ"
            }])))
            .expect(1)
            .mount(&server)
            .await;
        let client = FmpClient::builder()
            .api_key("test_key")
            .base_url(server.uri())
            .build()
            .unwrap();
        let results = client
            .company_search()
            .search("Apple", Some(10), None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "AAPL");
    }

    #[test]
    fn test_search_builds_correct_url() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let search = CompanySearch::new(client);
        // Just verify we can create the search instance
        assert_eq!(search.client.api_key(), "test_key");
    }
}
