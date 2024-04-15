use std::fs;

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DATABASE_URL: &str = ".git-streak.database.sqlite";

pub async fn pool() -> Pool<Sqlite> {
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap() {
        Sqlite::create_database(DATABASE_URL).await.unwrap();
    }

    SqlitePool::connect(DATABASE_URL).await.unwrap()
}

pub async fn migrate() {
    let database_pool = pool().await;

    if let Err(_error) = sqlx::migrate!().run(&database_pool).await {
        fs::remove_file(DATABASE_URL).unwrap();
        sqlx::migrate!().run(&database_pool).await.unwrap();
    }
}
