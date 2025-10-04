//! Models for transcripts and communications endpoints

use serde::{Deserialize, Serialize};

/// Earnings call transcript data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsTranscript {
    /// Company symbol
    pub symbol: Option<String>,
    /// Quarter (e.g., "Q1", "Q2", "Q3", "Q4")
    pub quarter: Option<String>,
    /// Calendar year
    pub year: Option<i32>,
    /// Earnings date
    pub date: Option<String>,
    /// Full transcript content
    pub content: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Quarter fiscal period
    pub fiscal_quarter: Option<String>,
    /// Fiscal year
    pub fiscal_year: Option<i32>,
    /// Transcript ID
    pub transcript_id: Option<String>,
    /// Language of transcript
    pub language: Option<String>,
    /// Duration of call (in minutes)
    pub duration: Option<i32>,
    /// Number of analysts participating
    pub analyst_count: Option<i32>,
    /// Conference call type (e.g., "Earnings", "Special")
    pub call_type: Option<String>,
}

/// Earnings call transcript list/summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSummary {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Quarter
    pub quarter: Option<String>,
    /// Calendar year
    pub year: Option<i32>,
    /// Earnings date
    pub date: Option<String>,
    /// Transcript availability status
    pub is_available: Option<bool>,
    /// Call duration in minutes
    pub duration: Option<i32>,
    /// Transcript excerpt or summary
    pub excerpt: Option<String>,
    /// Fiscal quarter
    pub fiscal_quarter: Option<String>,
    /// Fiscal year
    pub fiscal_year: Option<i32>,
    /// Conference call type
    pub call_type: Option<String>,
    /// Last updated timestamp
    pub last_updated: Option<String>,
}

/// Press release data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressRelease {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Press release title
    pub title: Option<String>,
    /// Publication date
    pub date: Option<String>,
    /// Full press release content
    pub content: Option<String>,
    /// Press release type (e.g., "Earnings", "Product", "Corporate")
    pub release_type: Option<String>,
    /// Source (e.g., "Business Wire", "PR Newswire")
    pub source: Option<String>,
    /// Press release URL
    pub url: Option<String>,
    /// Language
    pub language: Option<String>,
    /// Word count
    pub word_count: Option<i32>,
    /// Summary or excerpt
    pub summary: Option<String>,
    /// Related ticker symbols
    pub related_symbols: Option<Vec<String>>,
    /// Industry tags
    pub tags: Option<Vec<String>>,
}

/// Conference call schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceCall {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Call title
    pub title: Option<String>,
    /// Scheduled date and time
    pub date_time: Option<String>,
    /// Call type (e.g., "Earnings", "Analyst Day", "Special")
    pub call_type: Option<String>,
    /// Quarter being reported
    pub quarter: Option<String>,
    /// Fiscal year
    pub fiscal_year: Option<i32>,
    /// Calendar year
    pub year: Option<i32>,
    /// Time zone
    pub timezone: Option<String>,
    /// Dial-in information
    pub dial_in_info: Option<String>,
    /// Webcast URL
    pub webcast_url: Option<String>,
    /// Call duration (estimated in minutes)
    pub estimated_duration: Option<i32>,
    /// Participant information
    pub participants: Option<Vec<String>>,
    /// Call status (e.g., "Scheduled", "In Progress", "Completed")
    pub status: Option<String>,
    /// Industry
    pub industry: Option<String>,
    /// Market cap category
    pub market_cap_category: Option<String>,
}

/// Management discussion and analysis (MD&A) content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementDiscussion {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Filing date
    pub date: Option<String>,
    /// Filing period (e.g., "Q1 2024", "FY 2023")
    pub period: Option<String>,
    /// Form type (e.g., "10-K", "10-Q")
    pub form_type: Option<String>,
    /// MD&A section content
    pub content: Option<String>,
    /// Section title
    pub section_title: Option<String>,
    /// CIK (Central Index Key)
    pub cik: Option<String>,
    /// Accession number
    pub accession_number: Option<String>,
    /// Filing URL
    pub filing_url: Option<String>,
    /// Word count
    pub word_count: Option<i32>,
    /// Key topics discussed
    pub key_topics: Option<Vec<String>>,
    /// Risk factors mentioned
    pub risk_factors: Option<Vec<String>>,
    /// Language
    pub language: Option<String>,
}
