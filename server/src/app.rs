use crate::routers::{get_weekly_trending, root};
use axum::routing::get;
use axum::Router;
use sqlx::mysql::MySqlPoolOptions;

pub async fn new_app() -> Router {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost:3306/movie_db")
        .await
        .expect("Failed to connect to Mysql.");
    Router::new()
        .route("/", get(root))
        .route("/weekly_trending", get(get_weekly_trending))
        .with_state(pool)
}
