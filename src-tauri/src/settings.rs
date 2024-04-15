use crate::database;

#[allow(dead_code)] // becouse id is in the db but isn't used here
#[derive(Debug, sqlx::FromRow)]
struct Settings {
    startup: bool,
    notifications: bool,
    all_authors: bool,
}

#[tauri::command]
pub async fn get_settings() -> Vec<bool> {
    let pool = database::pool().await;

    let settings = sqlx::query_as!(Settings, "SELECT * FROM settings")
        .fetch_one(&pool)
        .await
        .unwrap();

    vec![
        settings.startup,
        settings.notifications,
        settings.all_authors,
    ]
}

#[tauri::command]
pub async fn set_setting(setting: String, value: bool) {
    let pool = database::pool().await;

    sqlx::query(format!("UPDATE settings SET {setting} = {value}").as_str())
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn get_setting(setting: &str) -> bool {
    let settings = get_settings().await;

    match setting {
        "startup" => settings[0],
        "notifications" => settings[1],
        "all_authors" => settings[2],
        _ => false,
    }
}
