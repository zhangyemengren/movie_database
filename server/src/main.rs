use axum::{routing::get, Router};
use reqwest;
use reqwest::header::HeaderMap;

async fn root() -> &'static str {
    let client = reqwest::Client::new();
    let key = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiIzZTRjMmUzNzIwY2EzNjc5ZTdkMzZlYTlmMmU0YjY3ZCIsIm5iZiI6MTcyMDI2NDU3Ny43MTY1OSwic3ViIjoiNjU0Y2Q1ODMxYWMyOTI3YjJkY2ZmYzZjIiwic2NvcGVzIjpbImFwaV9yZWFkIl0sInZlcnNpb24iOjF9.g61yEF8VbD8zgFbnS28nXRSmbrHm9lE-yB2z-mceKJQ";
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
