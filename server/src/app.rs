use axum::Router;
use axum::routing::get;
use sqlx::mysql::MySqlPoolOptions;
use crate::routers::{root, get_weekly_trending};

pub async fn new_app() -> Router {
    let pool = MySqlPoolOptions::new()
        .max_connections(5).connect("mysql://root:root@localhost:3306/movie_db").await
        .expect("Failed to connect to Mysql.");
    Router::new().route("/", get(root)).route("/weekly_trending", get(get_weekly_trending)).with_state(pool)
}