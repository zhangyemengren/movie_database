use axum::Router;
use axum::routing::get;
use crate::routers::root;

pub fn new_app() -> Router {
    Router::new().route("/", get(root))
}