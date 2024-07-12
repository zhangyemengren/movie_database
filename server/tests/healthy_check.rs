use sqlx::MySqlPool;

#[tokio::test]
async fn test_db_link() {
    let pool = MySqlPool::connect("mysql://root:root@localhost:3306/movie_db")
        .await
        .unwrap();
    let list = sqlx::query!("SELECT * FROM weekly_trending_movies")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(list.len(), 0);
}
