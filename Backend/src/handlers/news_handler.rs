use axum::{extract::Query, Json, response::IntoResponse};
use std::collections::HashMap;
use crate::services::coinpaprika_service::get_news_articles;
use crate::models::news_article::NewsArticle;

pub async fn fetch_crypto_news(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let search_query = params.get("search").map(String::to_owned);
    let articles: Vec<NewsArticle> = get_news_articles(search_query).await;
    Json(articles)
}
