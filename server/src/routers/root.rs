use axum::debug_handler;
use reqwest::header::HeaderMap;
use crate::{get_author_header, get_env_var};

#[debug_handler]
pub async fn root() -> &'static str {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.themoviedb.org/3/trending/movie/week?language=zh-CN")
        .headers(get_author_header())
        .send()
        .await
        .unwrap();
    println!("{:?}", res.text().await.unwrap());
    "Hello, World!"
}