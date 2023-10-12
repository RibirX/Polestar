use super::common::init_setup_db;
use sqlx::Row;

#[tokio::test]
async fn test_table_setup() {
  let pool = init_setup_db().await.expect("Failed to init database");
  let row = sqlx::query(
    r#"
      SELECT COUNT(*) as count
      FROM sqlite_master
      WHERE type = 'table'
        AND name != '_sqlx_migrations'
      "#,
  )
  .fetch_one(&pool)
  .await
  .expect("Failed to query database");

  let count: i32 = row.get(0);

  // msg and channel two tables
  assert_eq!(count, 2);
}
