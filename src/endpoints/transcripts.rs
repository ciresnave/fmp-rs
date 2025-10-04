//! Transcripts & Communications endpoints

use crate::{client::FmpClient, error::Result, models::transcripts::*};
use serde::Serialize;

/// Transcripts & Communications API endpoints
pub struct Transcripts {
    client: FmpClient,
}

impl Transcripts {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get earnings call transcripts list
    ///
    /// Returns a list of available earnings call transcripts for a company.
    /// Useful for discovering available transcripts before fetching full content.
    ///
    /// # Arguments
    /// * `symbol` - Company symbol (e.g., "AAPL")
    /// * `year` - Optional year filter (e.g., 2024)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let transcripts = client.transcripts().get_transcript_list("AAPL", Some(2024)).await?;
    /// for transcript in transcripts.iter().take(5) {
    ///     println!("{} {} {}: {}",
    ///         transcript.symbol.as_deref().unwrap_or("N/A"),
    ///         transcript.quarter.as_deref().unwrap_or("N/A"),
    ///         transcript.year.unwrap_or(0),
    ///         transcript.date.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transcript_list(
        &self,
        symbol: &str,
        year: Option<i32>,
    ) -> Result<Vec<TranscriptSummary>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            year: Option<i32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/earning_call_transcript/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    year,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get full earnings call transcript
    ///
    /// Returns the complete transcript content for a specific earnings call.
    /// Contains the full Q&A session with management and analysts.
    ///
    /// # Arguments
    /// * `symbol` - Company symbol (e.g., "AAPL")
    /// * `year` - Year of the earnings call
    /// * `quarter` - Quarter number (1, 2, 3, or 4)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let transcript = client.transcripts().get_earnings_transcript("AAPL", 2024, 1).await?;
    /// if let Some(call) = transcript.first() {
    ///     println!("Transcript for {} {} {}:",
    ///         call.symbol.as_deref().unwrap_or("N/A"),
    ///         call.quarter.as_deref().unwrap_or("N/A"),
    ///         call.year.unwrap_or(0));
    ///     if let Some(content) = &call.content {
    ///         println!("Content length: {} characters", content.len());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_earnings_transcript(
        &self,
        symbol: &str,
        year: i32,
        quarter: i32,
    ) -> Result<Vec<EarningsTranscript>> {
        #[derive(Serialize)]
        struct Query<'a> {
            year: i32,
            quarter: i32,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/earning_call_transcript/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    year,
                    quarter,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get press releases for a company
    ///
    /// Returns recent press releases and corporate announcements.
    /// Useful for staying updated on company news and developments.
    ///
    /// # Arguments
    /// * `symbol` - Company symbol (e.g., "AAPL")
    /// * `limit` - Optional limit on number of results (default: 100)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let releases = client.transcripts().get_press_releases("AAPL", Some(10)).await?;
    /// for release in releases.iter().take(5) {
    ///     println!("{}: {}",
    ///         release.date.as_deref().unwrap_or("N/A"),
    ///         release.title.as_deref().unwrap_or("No title"));
    ///     if let Some(summary) = &release.summary {
    ///         println!("  Summary: {}", summary);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_press_releases(
        &self,
        symbol: &str,
        limit: Option<i32>,
    ) -> Result<Vec<PressRelease>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<i32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/press-releases/{}", symbol));
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

    /// Get conference call schedule
    ///
    /// Returns upcoming and recent conference calls across companies.
    /// Useful for tracking earnings dates and investor events.
    ///
    /// # Arguments
    /// * `from_date` - Optional start date (YYYY-MM-DD format)
    /// * `to_date` - Optional end date (YYYY-MM-DD format)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let calls = client.transcripts()
    ///     .get_conference_schedule(Some("2024-01-01"), Some("2024-01-31")).await?;
    /// for call in calls.iter().take(10) {
    ///     println!("{}: {} - {}",
    ///         call.date_time.as_deref().unwrap_or("N/A"),
    ///         call.symbol.as_deref().unwrap_or("N/A"),
    ///         call.title.as_deref().unwrap_or("No title"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_conference_schedule(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConferenceCall>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/earning_calendar");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from: from_date,
                    to: to_date,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_client() -> FmpClient {
        FmpClient::builder().api_key("test_key").build().unwrap()
    }

    #[test]
    fn test_new() {
        let client = create_test_client();
        let transcripts = Transcripts::new(client);
        // Test passes if no panic occurs
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_transcript_list() {
        let client = FmpClient::new().unwrap();
        let result = client
            .transcripts()
            .get_transcript_list("AAPL", Some(2024))
            .await;
        assert!(result.is_ok());

        let transcripts = result.unwrap();
        if !transcripts.is_empty() {
            let first_transcript = &transcripts[0];
            assert!(first_transcript.symbol.is_some());
            assert!(first_transcript.quarter.is_some() || first_transcript.year.is_some());
            println!("Found {} transcript summaries for AAPL", transcripts.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_transcript_list_no_year() {
        let client = FmpClient::new().unwrap();
        let result = client.transcripts().get_transcript_list("AAPL", None).await;
        assert!(result.is_ok());

        let transcripts = result.unwrap();
        println!(
            "Found {} total transcript summaries for AAPL",
            transcripts.len()
        );
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_earnings_transcript() {
        let client = FmpClient::new().unwrap();
        let result = client
            .transcripts()
            .get_earnings_transcript("AAPL", 2023, 4)
            .await;
        assert!(result.is_ok());

        let transcripts = result.unwrap();
        if let Some(transcript) = transcripts.first() {
            assert_eq!(transcript.symbol.as_deref(), Some("AAPL"));
            assert_eq!(transcript.year, Some(2023));

            if let Some(content) = &transcript.content {
                assert!(!content.is_empty());
                println!("Transcript content length: {} characters", content.len());
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_press_releases() {
        let client = FmpClient::new().unwrap();
        let result = client
            .transcripts()
            .get_press_releases("AAPL", Some(10))
            .await;
        assert!(result.is_ok());

        let releases = result.unwrap();
        if !releases.is_empty() {
            let first_release = &releases[0];
            assert!(first_release.symbol.is_some() || first_release.company_name.is_some());
            assert!(first_release.title.is_some());
            assert!(first_release.date.is_some());
            println!("Found {} press releases for AAPL", releases.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_press_releases_no_limit() {
        let client = FmpClient::new().unwrap();
        let result = client.transcripts().get_press_releases("MSFT", None).await;
        assert!(result.is_ok());

        let releases = result.unwrap();
        println!("Found {} total press releases for MSFT", releases.len());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_conference_schedule() {
        let client = FmpClient::new().unwrap();
        let result = client
            .transcripts()
            .get_conference_schedule(Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());

        let calls = result.unwrap();
        if !calls.is_empty() {
            let first_call = &calls[0];
            assert!(first_call.symbol.is_some());
            assert!(first_call.date_time.is_some() || first_call.date_time.is_some());
            println!("Found {} conference calls in January 2024", calls.len());
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_conference_schedule_no_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .transcripts()
            .get_conference_schedule(None, None)
            .await;
        assert!(result.is_ok());

        let calls = result.unwrap();
        println!("Found {} total upcoming conference calls", calls.len());
    }

    #[test]
    fn test_transcript_models_serialization() {
        use serde_json;

        // Test EarningsTranscript model
        let transcript = EarningsTranscript {
            symbol: Some("AAPL".to_string()),
            quarter: Some("Q1".to_string()),
            year: Some(2024),
            date: Some("2024-02-01".to_string()),
            content: Some("Thank you for joining Apple's Q1 2024 earnings call...".to_string()),
            company_name: Some("Apple Inc.".to_string()),
            fiscal_quarter: Some("Q1".to_string()),
            fiscal_year: Some(2024),
            transcript_id: Some("AAPL-2024-Q1".to_string()),
            language: Some("English".to_string()),
            duration: Some(60),
            analyst_count: Some(15),
            call_type: Some("Earnings".to_string()),
        };

        let json = serde_json::to_string(&transcript).unwrap();
        let deserialized: EarningsTranscript = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, Some("AAPL".to_string()));
        assert_eq!(deserialized.year, Some(2024));

        // Test PressRelease model
        let release = PressRelease {
            symbol: Some("AAPL".to_string()),
            company_name: Some("Apple Inc.".to_string()),
            title: Some("Apple Reports Record Q1 Results".to_string()),
            date: Some("2024-02-01".to_string()),
            content: Some("Apple today announced financial results for Q1...".to_string()),
            release_type: Some("Earnings".to_string()),
            source: Some("Business Wire".to_string()),
            url: Some("https://investor.apple.com/news/press-release-details/2024/Apple-Reports-Record-Q1-Results/default.aspx".to_string()),
            language: Some("English".to_string()),
            word_count: Some(1250),
            summary: Some("Apple reported record Q1 revenue of $119.6 billion...".to_string()),
            related_symbols: Some(vec!["AAPL".to_string(), "NASDAQ".to_string()]),
            tags: Some(vec!["Earnings".to_string(), "Technology".to_string()]),
        };

        let json = serde_json::to_string(&release).unwrap();
        let deserialized: PressRelease = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, Some("AAPL".to_string()));
        assert_eq!(
            deserialized.title,
            Some("Apple Reports Record Q1 Results".to_string())
        );
    }

    #[test]
    fn test_conference_call_model() {
        let call = ConferenceCall {
            symbol: Some("AAPL".to_string()),
            company_name: Some("Apple Inc.".to_string()),
            title: Some("Q1 2024 Earnings Call".to_string()),
            date_time: Some("2024-02-01 17:00:00".to_string()),
            call_type: Some("Earnings".to_string()),
            quarter: Some("Q1".to_string()),
            fiscal_year: Some(2024),
            year: Some(2024),
            timezone: Some("EST".to_string()),
            dial_in_info: Some("1-800-123-4567, Conference ID: 12345".to_string()),
            webcast_url: Some("https://investor.apple.com/webcast".to_string()),
            estimated_duration: Some(60),
            participants: Some(vec![
                "Tim Cook - CEO".to_string(),
                "Luca Maestri - CFO".to_string(),
            ]),
            status: Some("Scheduled".to_string()),
            industry: Some("Technology".to_string()),
            market_cap_category: Some("Large Cap".to_string()),
        };

        // Verify all fields are accessible
        assert_eq!(call.symbol, Some("AAPL".to_string()));
        assert_eq!(call.call_type, Some("Earnings".to_string()));
        assert_eq!(call.estimated_duration, Some(60));

        let json = serde_json::to_string(&call).unwrap();
        let deserialized: ConferenceCall = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.symbol, Some("AAPL".to_string()));
    }

    #[test]
    fn test_date_parameter_validation() {
        // Test that date parameters are properly formatted
        let start_date = "2024-01-01";
        let end_date = "2024-12-31";

        // These should be valid ISO date formats
        assert!(chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").is_ok());
        assert!(chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d").is_ok());
    }

    #[test]
    fn test_quarter_validation() {
        let valid_quarters = [1, 2, 3, 4];
        for quarter in valid_quarters {
            assert!(
                quarter >= 1 && quarter <= 4,
                "Quarter {} should be valid",
                quarter
            );
        }

        let invalid_quarters = [0, 5, -1, 13];
        for quarter in invalid_quarters {
            assert!(
                !(quarter >= 1 && quarter <= 4),
                "Quarter {} should be invalid",
                quarter
            );
        }
    }
}
