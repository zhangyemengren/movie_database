use axum::debug_handler;
use axum::extract::State;
use sqlx::MySqlPool;
use crate::{get_author_header};

#[debug_handler]
pub async fn root() -> &'static str {
    "Hello, World!"
}


#[derive(serde::Deserialize, Debug)]
struct Response {
    page: u32,
    results: Vec<Movie>,
    total_pages: u32,
    total_results: u32,
}
#[derive(serde::Deserialize, Debug)]
struct Movie{
    backdrop_path: String,
    id: u32,
    title: String,
    original_title: String,
    overview: String,
    poster_path: String,
    adult: bool,
    original_language: String,
    genre_ids: Vec<u32>,
    popularity: f32,
    release_date: String,
    vote_average: f32,
    vote_count: u32,
}
pub async fn get_weekly_trending(State(pool): State<MySqlPool>) -> &'static str {
    let search = sqlx::query_as!(Movie, "SELECT * FROM weekly_trending_movies")
        .fetch_all(pool)
        .await
        .unwrap();
    println!("{:?}", search);
    // let client = reqwest::Client::new();
    // let res = client
    //     .get("https://api.themoviedb.org/3/trending/movie/week?language=zh-CN")
    //     .headers(get_author_header())
    //     .send()
    //     .await
    //     .unwrap();
    // println!("{:?}", res.json::<Response>().await.unwrap());
    "Hello, World!"
}