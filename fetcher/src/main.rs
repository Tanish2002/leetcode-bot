mod configuration;
mod models;
mod service;

use configuration::Configuration;
use log::error;

#[tokio::main]
async fn main() {
    let config = Configuration::new().await;
    for user in config.users {
        // Check for valid user
        let user_resp = match config.service.leetcode.get_user(&user).await {
            Ok(result) => result,
            Err(_) => {
                continue;
            }
        };

        // Fetch last known timestamp
        let last_timestamp = match config.model.get_latest_timestamp(&user).await {
            Ok(timestamp) => timestamp,
            Err(err) => {
                error!(
                    "Error while getting timestamp for user: {}. Error: {}",
                    user, err
                );
                continue;
            }
        };

        // Fetch user's last 15 submissions
        let mut user_submissions = match config.service.leetcode.get_recent_submissions(&user).await
        {
            Ok(submissions) => submissions,
            Err(err) => {
                error!(
                    "Error while fetching user submissions for user: {}. Error: {}",
                    user, err
                );
                continue;
            }
        };
        // Filter new submissions
        if let Some(index) = user_submissions
            .recent_ac_submission_list
            .iter()
            .position(|submission| submission.timestamp == last_timestamp)
        {
            user_submissions.recent_ac_submission_list.truncate(index);
        }

        // We have no new submissions
        if user_submissions.recent_ac_submission_list.len() == 0 {
            continue;
        }

        // Update the new timestamp
        if let Err(e) = config
            .model
            .add_or_update_timestamp(
                &user,
                &user_submissions.recent_ac_submission_list[0].timestamp,
            )
            .await
        {
            error!("Error while saving latest Timestamp. Error: {}", e);
            continue;
        };

        // Send new data to sqs
        if let Err(e) = config
            .service
            .sqs
            .send_to_sqs(
                &user,
                &user_resp.matched_user.profile.user_avatar,
                user_submissions,
            )
            .await
        {
            error!(
                "Error while sending to sqs, Reverting back to old timestamp. Error: {}",
                e
            );
            // Revert back to old timestamp
            let _ = config
                .model
                .add_or_update_timestamp(&user, &last_timestamp)
                .await;
        }
    }
}
