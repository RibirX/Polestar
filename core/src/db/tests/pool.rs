



use super::common::init_db;
use sqlx::Row;

#[tokio::test]
async fn test_table_setup() {
  let pool = init_db().await.expect("Failed to init database");
  let rows = sqlx::query(
    r#"
      SELECT name
      FROM sqlite_master
      WHERE type = 'table'
        AND name != '_sqlx_migrations'
      "#,
  )
  .fetch_all(&pool)
  .await
  .expect("Failed to query database");

  rows.iter().for_each(|r| {
    let name: String = r.get(0);
    println!("table name: {}", name);
  });

  let row = sqlx::query(
    r#"
      SELECT COUNT(*) as count
      FROM sqlite_master
      WHERE type = 'table'
        AND name != '_sqlx_migrations'
        AND name != "sqlite_sequence"
      "#,
  )
  .fetch_one(&pool)
  .await
  .expect("Failed to query database");

  let count: i32 = row.get(0);

  println!("count: {}", count);

  // msg/channel/attachment three tables
  assert_eq!(count, 3);
}
