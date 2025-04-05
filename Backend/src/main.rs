use axum::{Router, routing::get, Json};
use hyper::Server;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use axum::response::IntoResponse;
use axum::extract::Query;
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Configure CORS to allow requests from your frontend
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<hyper::header::HeaderValue>().unwrap())
        .allow_methods([axum::http::Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(|| async { "Crypto News Aggregator API" }))
        .route("/health", get(|| async { "Server is running" }))
        .route("/news", get(fetch_crypto_news))
        .layer(cors); // Add the CORS layer here

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    url: String,
}

// Function to fetch crypto market updates from CoinPaprika
// Function to fetch crypto market updates from CoinPaprika
async fn fetch_crypto_news(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let search_query = params.get("search");

    // CoinPaprika API Endpoint
    let api_url = "https://api.coinpaprika.com/v1/tickers";

    let client = Client::new();
    let response = client.get(api_url).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let mut articles = Vec::new();
                    if let Some(cryptos) = json.as_array() {
                        for crypto in cryptos.iter() {
                            let name = crypto["name"].as_str().unwrap_or("Unknown").to_string();
                            let symbol = crypto["id"].as_str().unwrap_or("").to_string();

                            // If search is provided, filter results by name or symbol
                            if let Some(query) = search_query {
                                if !name.to_lowercase().contains(&query.to_lowercase()) &&
                                   !symbol.to_lowercase().contains(&query.to_lowercase()) {
                                    continue; // Skip if it doesn't match the search query
                                }
                            }

                            let price = crypto["quotes"]["USD"]["price"].as_f64().unwrap_or(0.0);
                            let market_cap = crypto["quotes"]["USD"]["market_cap"].as_f64().unwrap_or(0.0);
                            let last_updated = crypto["type"].as_str().unwrap_or("Unknown").to_string();

                            let article = NewsArticle {
                                title: format!("{} ({})", name, symbol),
                                source: "CoinPaprika".to_string(),
                                date: last_updated.clone(),
                                summary: format!(
                                    "Price: ${:.2}, Market Cap: ${:.2}B",
                                    price,
                                    market_cap / 1_000_000_000.0
                                ),
                                url: format!("https://coinpaprika.com/coin/{}/", symbol.to_lowercase()),
                            };

                            // Print search result in the console
                            if search_query.is_some() {
                                println!(
                                    "Searched Crypto: {}\nSymbol: {}\nPrice: ${:.2}\nMarket Cap: ${:.2}B\nLast Updated: {}\n",
                                    name, symbol, price, market_cap / 1_000_000_000.0, last_updated
                                );
                            }

                            articles.push(article);
                        }
                    }
                    return Json(articles);
                } else {
                    eprintln!("Failed to parse JSON from response");
                }
            } else {
                eprintln!("Error: Received non-OK response: {}", resp.status());
            }
        }
        Err(err) => eprintln!("Failed to fetch data from CoinPaprika: {}", err),
    }

    Json(vec![]) // Return empty list if API fails
}
