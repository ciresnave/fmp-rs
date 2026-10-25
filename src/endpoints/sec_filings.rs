//! SEC filings endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::sec_filings::{
        CikSearch, CompanyName, Form13F, Form4, InsiderOwnership, SecFiling, SecFilingRss,
    },
};
use serde::Serialize;

/// SEC Filings API endpoints
pub struct SecFilings {
    client: FmpClient,
}

impl SecFilings {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get SEC filings for a symbol
    ///
    /// Returns SEC filings (10-K, 10-Q, 8-K, etc.) for a given stock symbol.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Maximum number of filings to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let filings = client.sec_filings().get_sec_filings("AAPL", Some(10)).await?;
    /// for filing in filings {
    ///     println!("{}: {} on {}",
    ///         filing.symbol,
    ///         filing.filing_type.unwrap_or_default(),
    ///         filing.filing_date.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sec_filings(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<SecFiling>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/sec_filings/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get SEC filings RSS feed
    ///
    /// Returns recent SEC filings from the RSS feed with optional filtering.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of filings to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let rss = client.sec_filings().get_sec_rss_feed(Some(20)).await?;
    /// for item in rss.iter().take(10) {
    ///     println!("{}: {} - {}",
    ///         item.symbol.as_deref().unwrap_or("N/A"),
    ///         item.form_type.as_deref().unwrap_or("N/A"),
    ///         item.title.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sec_rss_feed(&self, limit: Option<u32>) -> Result<Vec<SecFilingRss>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/rss_feed");
        self.client
            .get_with_query(
                &url,
                &Query {
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Form 13F filings
    ///
    /// Returns institutional holdings from Form 13F filings.
    ///
    /// # Arguments
    /// * `cik` - CIK number of the institution
    /// * `date` - Filing date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let holdings = client.sec_filings().get_form_13f("0001067983", None).await?;
    /// for holding in holdings.iter().take(10) {
    ///     println!("{}: {} shares valued at ${}",
    ///         holding.name_of_issuer.as_deref().unwrap_or("N/A"),
    ///         holding.shares.unwrap_or(0),
    ///         holding.market_value.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_form_13f(&self, cik: &str, date: Option<&str>) -> Result<Vec<Form13F>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            date: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/form-thirteen/{}", cik));
        self.client
            .get_with_query(
                &url,
                &Query {
                    date,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get cusip mapper (CUSIP to company info)
    ///
    /// Returns company information for a given CUSIP.
    ///
    /// # Arguments
    /// * `cusip` - CUSIP identifier
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let info = client.sec_filings().get_cusip_mapper("037833100").await?;
    /// for item in info {
    ///     println!("{}: {}", item.cik, item.name.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_cusip_mapper(&self, cusip: &str) -> Result<Vec<CikSearch>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/cusip/{}", cusip));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Search for CIK by company name
    ///
    /// Returns CIK numbers for companies matching the search query.
    ///
    /// # Arguments
    /// * `name` - Company name to search for
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let results = client.sec_filings().search_cik("Apple").await?;
    /// for result in results {
    ///     println!("CIK {}: {}", result.cik, result.name.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_cik(&self, name: &str) -> Result<Vec<CikSearch>> {
        #[derive(Serialize)]
        struct Query<'a> {
            name: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/cik-search");
        self.client
            .get_with_query(
                &url,
                &Query {
                    name,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company name by CIK
    ///
    /// Returns the company name for a given CIK number.
    ///
    /// # Arguments
    /// * `cik` - CIK number
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let company = client.sec_filings().get_company_name_by_cik("0000320193").await?;
    /// for c in company {
    ///     println!("CIK {}: {}", c.cik, c.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_company_name_by_cik(&self, cik: &str) -> Result<Vec<CompanyName>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/cik/{}", cik));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Form 4 filings (insider ownership changes)
    ///
    /// Returns Form 4 filings showing changes in beneficial ownership by insiders.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Maximum number of filings to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let form4 = client.sec_filings().get_form_4("AAPL", Some(20)).await?;
    /// for filing in form4 {
    ///     println!("{}: {} - {} shares at ${}",
    ///         filing.reporting_name.as_deref().unwrap_or("N/A"),
    ///         filing.transaction_type.as_deref().unwrap_or("N/A"),
    ///         filing.securities_transacted.unwrap_or(0.0),
    ///         filing.price.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_form_4(&self, symbol: &str, limit: Option<u32>) -> Result<Vec<Form4>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/form-four/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get insider ownership summary
    ///
    /// Returns a summary of insider ownership from Forms 3, 4, and 5.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Maximum number of records to return (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let ownership = client.sec_filings().get_insider_ownership("AAPL", Some(10)).await?;
    /// for record in ownership {
    ///     println!("{}: owns {} shares",
    ///         record.reporting_name.as_deref().unwrap_or("N/A"),
    ///         record.securities_owned.unwrap_or(0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_insider_ownership(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<InsiderOwnership>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/insider-ownership/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sec_filings() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_sec_filings("AAPL", Some(10)).await;
        assert!(result.is_ok());
        let filings = result.unwrap();
        assert!(!filings.is_empty());
        assert!(filings.len() <= 10);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sec_rss_feed() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_sec_rss_feed(Some(20)).await;
        assert!(result.is_ok());
        let rss = result.unwrap();
        assert!(!rss.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_cik() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().search_cik("Apple").await;
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(!results.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_form_4() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_form_4("AAPL", Some(10)).await;
        assert!(result.is_ok());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_sec_filings_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_sec_filings("AAPL", None).await;
        assert!(result.is_ok());
        let filings = result.unwrap();
        assert!(!filings.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_search_cik_partial_match() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().search_cik("Tech").await;
        assert!(result.is_ok());
        // Should return multiple companies with "Tech" in name
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.sec_filings().get_sec_filings("AAPL", Some(10)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_sec_filings("INVALID_XYZ123", Some(10))
            .await;
        assert!(result.is_ok());
        if let Ok(filings) = result {
            assert!(filings.is_empty());
        }
    }

    // Missing endpoint tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_form_13f() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_form_13f("0000320193", None).await; // Apple's CIK
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_form_13f_with_date() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_form_13f("0000320193", Some("2023-12-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cusip_mapper() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_cusip_mapper("037833100").await; // Apple CUSIP
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cusip_mapper_invalid() {
        let client = FmpClient::new().unwrap();
        let result = client.sec_filings().get_cusip_mapper("INVALID123").await;
        if let Ok(results) = result {
            assert!(results.is_empty())
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_company_name_by_invalid_cik() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_company_name_by_cik("9999999999")
            .await;
        if let Ok(names) = result {
            assert!(names.is_empty())
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_ownership() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_insider_ownership("AAPL", Some(5))
            .await;
        assert!(result.is_ok());
        let ownership = result.unwrap();
        assert!(ownership.len() <= 5);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_ownership_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_insider_ownership("AAPL", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_insider_ownership_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .sec_filings()
            .get_insider_ownership("INVALID_SYMBOL_12345", Some(1))
            .await;
        if let Ok(ownership) = result {
            assert!(ownership.is_empty())
        }
    }
}
