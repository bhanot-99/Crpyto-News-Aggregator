use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub name: String,
    pub id: String,
    pub symbol: String,
    pub logo: String,
    pub description: String,
    pub started_at: String,
    pub link: String,
}