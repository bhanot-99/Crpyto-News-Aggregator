use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub url: String,
}
