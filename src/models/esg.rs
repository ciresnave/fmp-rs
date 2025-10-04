//! Models for ESG and social sentiment endpoints

use serde::{Deserialize, Serialize};

/// ESG (Environmental, Social, Governance) score data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgScore {
    /// Symbol
    pub symbol: String,
    /// Company name
    pub company_name: Option<String>,
    /// ESG score (0-100)
    pub esg_score: Option<f64>,
    /// Environmental score
    pub environment_score: Option<f64>,
    /// Social score
    pub social_score: Option<f64>,
    /// Governance score
    pub governance_score: Option<f64>,
    /// ESG performance
    pub esg_performance: Option<String>,
    /// Date of assessment
    pub date: Option<String>,
    /// Industry
    pub industry: Option<String>,
    /// Year
    pub year: Option<String>,
    /// Form type
    pub form_type: Option<String>,
}

/// ESG risk rating data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgRiskRating {
    /// Symbol
    pub symbol: String,
    /// Company name
    pub company_name: Option<String>,
    /// Industry
    pub industry: Option<String>,
    /// Year
    pub year: Option<String>,
    /// ESG risk score
    pub esg_risk_score: Option<f64>,
    /// Industry rank
    pub industry_rank: Option<String>,
    /// ESG risk percentile
    pub esg_risk_percentile: Option<f64>,
    /// ESG risk level
    pub esg_risk_level: Option<String>,
    /// Environment risk score
    pub environment_risk_score: Option<f64>,
    /// Governance risk score
    pub governance_risk_score: Option<f64>,
    /// Social risk score
    pub social_risk_score: Option<f64>,
}

/// ESG benchmark data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgBenchmark {
    /// Year
    pub year: Option<String>,
    /// Industry
    pub industry: Option<String>,
    /// ESG benchmark
    pub esg_benchmark: Option<f64>,
    /// Environment benchmark
    pub environment_benchmark: Option<f64>,
    /// Social benchmark
    pub social_benchmark: Option<f64>,
    /// Governance benchmark
    pub governance_benchmark: Option<f64>,
}

/// Congressional trading data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongressionalTrading {
    /// Disclosure date
    pub disclosure_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Representative name
    pub representative: Option<String>,
    /// Transaction type (Purchase/Sale)
    pub transaction: Option<String>,
    /// Stock symbol
    pub ticker: Option<String>,
    /// Asset description
    pub asset_description: Option<String>,
    /// Asset type
    pub asset_type: Option<String>,
    /// Amount (range)
    pub amount: Option<String>,
    /// House or Senate
    pub house: Option<String>,
    /// Political party
    pub party: Option<String>,
    /// State
    pub state: Option<String>,
    /// District
    pub district: Option<String>,
}

/// Senate trading data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenateTrading {
    /// Disclosure date
    pub disclosure_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Senator name
    pub senator: Option<String>,
    /// Transaction type
    pub transaction_type: Option<String>,
    /// Stock symbol
    pub ticker: Option<String>,
    /// Asset description
    pub asset_description: Option<String>,
    /// Asset type
    pub asset_type: Option<String>,
    /// Amount
    pub amount: Option<String>,
    /// Political party
    pub party: Option<String>,
    /// State
    pub state: Option<String>,
    /// Date received
    pub date_received: Option<String>,
}

/// Social sentiment data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSentiment {
    /// Date
    pub date: String,
    /// Symbol
    pub symbol: String,
    /// Stock tweeter posts
    pub stocktwits_posts: Option<i32>,
    /// Twitter posts
    pub twitter_posts: Option<i32>,
    /// Twitter comments
    pub twitter_comments: Option<i32>,
    /// Twitter likes
    pub twitter_likes: Option<i32>,
    /// Twitter sentiment
    pub twitter_sentiment: Option<f64>,
    /// StockTwits sentiment
    pub stocktwits_sentiment: Option<f64>,
    /// General sentiment
    pub general_sentiment: Option<String>,
}
