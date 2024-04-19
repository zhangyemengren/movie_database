// use sqlx::MySqlPool;

#[sqlx::test]
async fn test_db_link() {
    // let pool = MySqlPool::connect("mysql://root:qwer1234@localhost:3306/nba-data")
    //     .await
    //     .unwrap();
    // let teams = sqlx::query!("SELECT * FROM team")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();
    // assert_eq!(teams.len(), 30);
}
