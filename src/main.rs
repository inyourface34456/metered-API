#![allow(non_snake_case)]
use sqlx::mysql::MySqlPoolOptions;
// use tokio;
// etc.

// #[async_std::main] // Requires the `attributes` feature of `async-std`
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("sqlite://test.db").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}