use reqwest::Client;
use crate::models::news_article::NewsArticle;

pub async fn get_news_articles(search: Option<String>) -> Vec<NewsArticle> {
    let api_url = "https://api.coinpaprika.com/v1/coins";
    let client = Client::new();

    let mut articles = Vec::new();

    if let Ok(resp) = client.get(api_url).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(cryptos) = json.as_array() {
                for crypto in cryptos.iter() {
                    let name = crypto["name"].as_str().unwrap_or("Unknown").to_string();
                    let symbol = crypto["id"].as_str().unwrap_or("").to_string();

                    if let Some(ref query) = search {
                        if !name.to_lowercase().contains(&query.to_lowercase()) &&
                           !symbol.to_lowercase().contains(&query.to_lowercase()) {
                            continue;
                        }
                    }
                    let id = crypto["id"].as_str().unwrap_or("").to_string();
                    let description = crypto["description"].as_str().unwrap_or("No description available").to_string();
                    let logo = crypto["logo"].as_str().unwrap_or("").to_string();
                    let link = crypto["whitepaper"]["link"].as_str().unwrap_or("").to_string();
                    let started_at = crypto["started_at"].as_str().unwrap_or("Unknown").to_string();

                    articles.push(NewsArticle {
                        name,
                        id,
                        symbol,
                        logo,
                        description,
                        started_at,
                        link,
                    });
                }
            }
        }
    }

    articles
}
