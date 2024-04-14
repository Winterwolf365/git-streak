// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod icon;
mod notifications;
mod repositories;
mod settings;
mod streak;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[tokio::main]
async fn main() {
    let options = SqliteConnectOptions::new()
        .filename(".git-streak-database.sqlite")
        .create_if_missing(true);
    let database_pool = SqlitePool::connect_with(options).await.unwrap();

    sqlx::migrate!().run(&database_pool).await.unwrap();

    let autostart = settings::get_startup_setting().await;

    tokio::spawn(async {
        #[cfg(not(target_os = "macos"))]
        notifications::run_notifications().await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
        }))
        .setup(move |app| {
            let autostart_manager = app.autolaunch();

            if autostart {
                autostart_manager.enable().unwrap();
            } else {
                autostart_manager.disable().unwrap();
            }

            let open = MenuItemBuilder::with_id("open", "Open").build(app)?;
            let restart = MenuItemBuilder::with_id("restart", "Restart").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit Git Streak").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&open])
                .separator()
                .items(&[&restart, &quit])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "open" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                    }
                    "restart" => {
                        app.restart();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if event.click_type == ClickType::Left {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                })
                .icon(Image::from_bytes(&icon::get_icon_bytes()).unwrap())
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            streak::get_streak,
            settings::get_settings,
            settings::set_startup_setting,
            settings::set_notifications_setting,
            settings::set_all_authors_setting,
            repositories::get_repositories,
            repositories::add_repositories,
            repositories::delete_repository
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
