use crate::{repositories, settings};
use chrono::{Local, NaiveDate, Timelike};
use notify_rust::Notification;
use std::{thread, time::Duration};

pub async fn run_notifications() {
    loop {
        if !settings::get_setting("notifications").await {
            wait_and_continue();
        }

        let last_commits: Vec<NaiveDate> = repositories::get_repositories_days()
            .await
            .into_iter()
            .filter_map(|dates| {
                if dates.is_empty() {
                    None
                } else {
                    let newest_date = dates[dates.len() - 1];
                    Some(newest_date)
                }
            })
            .collect();

        let last_commit = last_commits.iter().max();

        if let Some(last_commit) = last_commit {
            if last_commit == &Local::now().date_naive() {
                wait_and_continue()
            }
        }

        if Local::now().hour() > 12 {
            wait_and_continue();
        }

        Notification::new()
            .summary("Git Commit")
            .body("Make an git commit today, so you won't lose your streak!")
            // .icon("firefox")
            .show()
            .unwrap();

        wait_and_continue();
    }
}

fn wait_and_continue() {
    thread::sleep(Duration::from_secs(
        60 /* minutes */ * 60, /* seconsds*/
    ));
}
