//! Analyst data models.

use serde::{Deserialize, Serialize};

/// Analyst estimates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnalystEstimates {
    pub symbol: String,
    pub date: String,
    pub estimated_revenue_low: i64,
    pub estimated_revenue_high: i64,
    pub estimated_revenue_avg: i64,
    pub estimated_ebitda_low: i64,
    pub estimated_ebitda_high: i64,
    pub estimated_ebitda_avg: i64,
    pub estimated_ebit_low: i64,
    pub estimated_ebit_high: i64,
    pub estimated_ebit_avg: i64,
    pub estimated_net_income_low: i64,
    pub estimated_net_income_high: i64,
    pub estimated_net_income_avg: i64,
    pub estimated_sga_expense_low: i64,
    pub estimated_sga_expense_high: i64,
    pub estimated_sga_expense_avg: i64,
    pub estimated_eps_avg: f64,
    pub estimated_eps_high: f64,
    pub estimated_eps_low: f64,
    pub number_analyst_estimated_revenue: i32,
    pub number_analysts_estimated_eps: i32,
}

/// Price target
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceTarget {
    pub symbol: String,
    pub published_date: String,
    pub news_url: String,
    pub news_title: String,
    pub analyst_name: String,
    pub price_target: f64,
    pub adj_price_target: f64,
    pub price_when_posted: f64,
    pub news_publisher: String,
    pub news_base_url: String,
    pub analyst_company: String,
}

/// Analyst recommendation/grade
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnalystGrade {
    pub symbol: String,
    pub published_date: String,
    pub grading_company: String,
    pub new_grade: String,
    pub previous_grade: Option<String>,
    pub grade_action: String,
}

/// Consensus summary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConsensusSummary {
    pub symbol: String,
    pub strong_buy: i32,
    pub buy: i32,
    pub hold: i32,
    pub sell: i32,
    pub strong_sell: i32,
    pub consensus: String,
}
