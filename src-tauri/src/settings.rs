use sqlx::SqlitePool;

#[allow(dead_code)] // becouse id is in the db but isn't used here
#[derive(Debug, sqlx::FromRow)]
struct Settings {
    startup: bool,
    notifications: bool,
}

#[tauri::command]
pub async fn get_settings() -> Vec<bool> {
    let pool = SqlitePool::connect(".git-streak-database.sqlite")
        .await
        .unwrap();

    let settings = sqlx::query_as!(Settings, "SELECT * FROM settings")
        .fetch_one(&pool)
        .await
        .unwrap();

    vec![settings.startup, settings.notifications]
}

#[tauri::command]
pub async fn set_startup_setting(startup: bool) {
    let pool = SqlitePool::connect(".git-streak-database.sqlite")
        .await
        .unwrap();

    sqlx::query(format!("UPDATE settings SET startup = {startup}").as_str())
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn get_startup_setting() -> bool {
    let pool = SqlitePool::connect(".git-streak-database.sqlite")
        .await
        .unwrap();

    sqlx::query_as!(Settings, "SELECT * FROM settings")
        .fetch_one(&pool)
        .await
        .unwrap()
        .startup
}

#[tauri::command]
pub async fn set_notifications_setting(notifications: bool) {
    let pool = SqlitePool::connect(".git-streak-database.sqlite")
        .await
        .unwrap();

    sqlx::query(format!("UPDATE settings SET notifications = {notifications}").as_str())
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn get_notifications_setting() -> bool {
    let pool = SqlitePool::connect(".git-streak-database.sqlite")
        .await
        .unwrap();

    sqlx::query_as!(Settings, "SELECT * FROM settings")
        .fetch_one(&pool)
        .await
        .unwrap()
        .notifications
}
