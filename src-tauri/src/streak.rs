use crate::repositories;

use chrono::{Duration, Local};

#[tauri::command]
pub async fn get_streak() -> i32 {
    let streaks = repositories::get_repositories_days()
        .await
        .into_iter()
        .map(|dates| {
            let mut streak = 0;

            if dates.is_empty() {
                return streak;
            }

            let newest_date = dates[dates.len() - 1];
            let today = Local::now().date_naive();
            let yesterday = Local::now().date_naive() - Duration::days(1);

            let commited_today_or_yesterday = newest_date == today || newest_date == yesterday;

            streak = i32::from(commited_today_or_yesterday);

            if dates.len() == 1 {
                return streak;
            }

            for index in 0..dates.len() - 1 {
                let current_date = dates[index];
                let next_date = dates[index + 1];

                if current_date + Duration::days(1) == next_date {
                    streak += 1;
                }

                if current_date + Duration::days(2) <= next_date {
                    streak = i32::from(commited_today_or_yesterday);
                }
            }

            streak
        });

    streaks.into_iter().max().unwrap_or_default()
}
