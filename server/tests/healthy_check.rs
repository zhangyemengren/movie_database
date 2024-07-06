use sqlx::MySqlPool;

#[tokio::test]
async fn test_db_link() {
    let _pool = MySqlPool::connect("mysql://root:root@localhost:3306/movie_db")
        .await
        .unwrap();

    assert!(true)
}
