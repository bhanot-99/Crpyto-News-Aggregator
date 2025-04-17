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

                    let price = crypto["quotes"]["USD"]["price"].as_f64().unwrap_or(0.0);
                    let market_cap = crypto["quotes"]["USD"]["market_cap"].as_f64().unwrap_or(0.0);
                    let last_updated = crypto["type"].as_str().unwrap_or("Unknown").to_string();

                    articles.push(NewsArticle {
                        title: format!("{} ({})", name, symbol),
                        source: "CoinPaprika".to_string(),
                        date: last_updated,
                        summary: format!(
                            "Price: ${:.2}, Market Cap: ${:.2}B",
                            price,
                            market_cap / 1_000_000_000.0
                        ),
                        url: format!("https://coinpaprika.com/coin/{}/", symbol.to_lowercase()),
                    });
                }
            }
        }
    }

    articles
}
