use axum::{debug_handler, routing::get, Router};
use reqwest::header::HeaderMap;

fn get_env_var(key: &str) -> String {
    dotenvy::dotenv().ok();
    if let Ok(v) = std::env::var(key) {
        return v;
    }

    "".to_string()
}

#[debug_handler]
async fn root() -> &'static str {
    let client = reqwest::Client::new();
    let key = get_env_var("TMDB_API_KEY");
    println!("key: {}", key);
    let h1 = format!("Bearer {}", key);
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", h1.parse().unwrap());
    let res = client
        .get("https://api.themoviedb.org/3/trending/movie/week?language=zh-CN")
        .headers(headers)
        .send()
        .await
        .unwrap();
    println!("{:?}", res.text().await.unwrap());
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
