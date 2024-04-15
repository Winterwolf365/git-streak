use std::{io::BufRead, path::Path, process::Command};

use crate::{database, settings};
use chrono::NaiveDate;
use rfd::FileDialog;

// for windows https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#flags
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[allow(dead_code)] // becouse id is in the db but isn't used here
#[derive(Debug, sqlx::FromRow)]
struct Repository {
    id: i64,
    path: String,
}

pub async fn get_repositories_days() -> Vec<Vec<NaiveDate>> {
    let repositories = get_repositories().await; // in (id, path format) so repo.1  = path
    let mut author = String::new();

    if !settings::get_setting("all_authors").await {
        author = String::from_utf8(
            Command::new("git")
                .arg("config")
                .arg("user.name")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
    }

    repositories
        .into_iter()
        .map(|repository| {
            let mut binding = Command::new("git");
            let command = binding
                .arg("log")
                // format adds "\n" at end so we had to remove that
                .arg(format!("--author={author}").lines().next().unwrap())
                .arg("--pretty=%as") // get the committer date, short format (YYYY-MM-DD)
                .current_dir(Path::new(&repository.1)); // in (id, path format) so repo.1  = path;

            #[cfg(target_os = "windows")]
            let command = command.creation_flags(CREATE_NO_WINDOW);

            println!("command: {:?}", command);

            let output = command.output().expect("failed to execute process");

            let dates: Vec<NaiveDate> = output
                .stdout
                .lines()
                .filter_map(|date| {
                    NaiveDate::parse_from_str(date.unwrap().as_str(), "%Y-%m-%d").ok()
                })
                .collect();

            let output = dates.into_iter().rev().collect::<Vec<NaiveDate>>();
            println!("output: {:?}", output);
            output
        })
        .collect()
}

#[tauri::command]
pub async fn get_repositories() -> Vec<(i64, String)> {
    let pool = database::pool().await;

    let repositories = sqlx::query_as!(Repository, "SELECT * FROM repositories")
        .fetch_all(&pool)
        .await
        .unwrap();

    repositories
        .into_iter()
        .map(|repository| (repository.id, repository.path))
        .collect()
}

#[tauri::command]
pub async fn add_repositories() -> Result<(), String> {
    let pool = database::pool().await;

    if let Some(repositories) = FileDialog::new().pick_folders() {
        for repository in repositories {
            let repository = repository.to_str().unwrap();

            if !Path::new(format!("{repository}/.git").as_str()).exists() {
                return Err(format!("\"{repository}\" is not a git repository!"));
            }

            if let Err(error) = sqlx::query(
                format!("INSERT INTO repositories (path) VALUES ('{repository}')",).as_str(),
            )
            .execute(&pool)
            .await
            {
                if format!("{:?}", error).contains("2067" /* database unique error */) {
                    return Err(format!(
                        "\"{repository}\" is already stored in the database!"
                    ));
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_repository(id: i64) {
    let pool = database::pool().await;

    sqlx::query(format!("DELETE FROM repositories WHERE id={id}",).as_str())
        .execute(&pool)
        .await
        .unwrap();
}
