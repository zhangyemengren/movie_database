use axum::{
    body::Body,
    http::{ Request, Method},
    response::Response,
};
use axum::http::header;
use tower::ServiceExt;
use server::{new_app, get_author_header};

pub async fn do_tmdb_request(uri: &str, body: Option<Body>) -> Response {
    let body = body.unwrap_or(Body::empty());
    let app = new_app();
    let headers = get_author_header();
    app.oneshot(
        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(header::AUTHORIZATION, headers.get(header::AUTHORIZATION).unwrap().to_str().unwrap())
            .body(body)
            .unwrap(),
    )
        .await
        .unwrap()
}