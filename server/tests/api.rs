use axum::http::StatusCode;
mod helper;

#[tokio::test]
async fn root() {
    let res = helper::do_tmdb_request("/",  None).await;

    assert_eq!(res.status(), StatusCode::OK);
}