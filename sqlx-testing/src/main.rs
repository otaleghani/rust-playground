use sqlx::{query, Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let filename = "database.db";
    // File::create(&filename).await?;
    // // let absolute_path = std::fs::canonicalize(&filename).unwrap().to_str().unwrap();
    // // let pool = SqlitePool::connect(
    // //     "sqlite://C:/Users/Oliviero/Desktop/Workspace/rust-playground/sqlx-testing/mydb.db",
    // // )
    // let pool = SqlitePool::connect(&format!("sqlite://./{}", &filename)).await?;
    // query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
    //     .execute(&pool)
    //     .await?;
    // Ok(())

    // You'll need a way to manage
    let mut database_map = HashMap::new();
    let filename = "database.db";
    let pool = create_database(filename).await?;
    database_map.insert(filename.to_string(), pool);

    Ok(())
}

// Create a new database based on name
async fn create_database(filename: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    tokio::fs::File::create(&filename).await.unwrap();
    let pool = SqlitePool::connect(&filename).await?;
    query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await?;
    Ok(pool)
}
