use sqlx::MySqlPool;

#[tokio::test]
async fn test_db_link() {
    let pool = MySqlPool::connect("mysql://root:root@localhost:3306/movie_db")
        .await
        .unwrap();
    sqlx::query!("SELECT * FROM movies").fetch_all(&pool)
        .await
        .unwrap();

    assert!(true)
}
