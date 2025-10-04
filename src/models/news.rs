//! News article models.

use serde::{Deserialize, Serialize};

/// News article
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticle {
    pub symbol: Option<String>,
    pub published_date: String,
    pub title: String,
    pub image: String,
    pub site: String,
    pub text: String,
    pub url: String,
}

/// Press release
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PressRelease {
    pub symbol: String,
    pub date: String,
    pub title: String,
    pub text: String,
}
