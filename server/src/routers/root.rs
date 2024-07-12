use crate::get_author_header;
use axum::extract::State;
use axum::{debug_handler, Json};
use sqlx::{MySql, MySqlPool, QueryBuilder};

#[debug_handler]
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Response {
    page: u32,
    results: Vec<Movie>,
    total_pages: u32,
    total_results: u32,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Movie {
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
pub async fn get_weekly_trending(State(pool): State<MySqlPool>) -> Json<Vec<Movie>> {
    let search = sqlx::query!(r#"select id from weekly_trending_movies"#)
        .fetch_all(&pool)
        .await
        .unwrap();
    if search.len() != 0 {}
    println!("{:?}", search);
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.themoviedb.org/3/trending/movie/week?language=zh-CN")
        .headers(get_author_header())
        .send()
        .await
        .unwrap();
    let data = res.json::<Response>().await.unwrap().results;
    println!("{:?}", data);

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        "INSERT INTO weekly_trending_movies(
        id,
        backdrop_path,
        title,
        original_title,
        overview,
        poster_path,
        adult,
        original_language,
        genre_ids,
        popularity,
        release_date,
        vote_average,
        vote_count)",
    );
    query_builder.push_values(data.clone(), |mut b, movie| {
        b.push_bind(movie.id)
            .push_bind(movie.backdrop_path)
            .push_bind(movie.title)
            .push_bind(movie.original_title)
            .push_bind(movie.overview)
            .push_bind(movie.poster_path)
            .push_bind(movie.adult)
            .push_bind(movie.original_language)
            .push_bind(
                movie
                    .genre_ids
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .push_bind(movie.popularity)
            .push_bind(movie.release_date)
            .push_bind(movie.vote_average)
            .push_bind(movie.vote_count);
    });
    let query = query_builder.build();
    query.execute(&pool).await.unwrap();
    data.into()
}
