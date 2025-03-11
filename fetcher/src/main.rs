mod configuration;
mod models;
mod service;
use configuration::Configuration;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let config = Configuration::new().await;
    info!("Starting LeetCode fetcher for {} users", config.users.len());

    for user in config.users {
        info!("Processing user: {}", user);

        let user_resp = match config.service.leetcode.get_user(&user).await {
            Ok(result) => result,
            Err(e) => {
                error!("Error fetching user {}: {}", user, e);
                continue;
            }
        };
        info!("Successfully retrieved profile for user: {}", user);

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

        let mut user_submissions = match config.service.leetcode.get_recent_submissions(&user).await
        {
            Ok(submissions) => submissions,
            Err(err) => {
                error!(
                    "Error while fetching submissions for user: {}. Error: {}",
                    user, err
                );
                continue;
            }
        };

        info!(
            "Retrieved {} submissions for user: {}",
            user_submissions.recent_ac_submission_list.len(),
            user
        );

        if !last_timestamp.is_empty() {
            if let Some(index) = user_submissions
                .recent_ac_submission_list
                .iter()
                .position(|submission| submission.timestamp == last_timestamp)
            {
                info!(
                    "Found {} new submissions since last timestamp for user: {}",
                    index, user
                );
                user_submissions.recent_ac_submission_list.truncate(index);
            }
        }

        if user_submissions.recent_ac_submission_list.is_empty() {
            info!("No new submissions for user: {}", user);
            continue;
        }

        user_submissions
            .recent_ac_submission_list
            .sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        info!(
            "Processing {} new submissions for user: {}",
            user_submissions.recent_ac_submission_list.len(),
            user
        );

        let mut update_timestamp = false;
        let latest_timestamp = user_submissions.recent_ac_submission_list[0]
            .timestamp
            .clone();

        for submission in &user_submissions.recent_ac_submission_list {
            match config
                .model
                .record_processed_submission(&user, &submission.title_slug, &submission.timestamp)
                .await
            {
                Ok(true) => {
                    let single_submission =
                        service::leetcode::submissions::RecentAcSubmissionResp {
                            recent_ac_submission_list: vec![submission.clone()],
                        };

                    match config
                        .service
                        .sqs
                        .send_to_sqs(
                            &user,
                            &user_resp.matched_user.profile.user_avatar,
                            single_submission,
                        )
                        .await
                    {
                        Ok(_) => {
                            info!(
                                "Successfully sent submission '{}' for user {}",
                                submission.title, user
                            );
                            update_timestamp = true;
                        }
                        Err(e) => {
                            error!(
                                "Error sending submission '{}' to SQS: {}",
                                submission.title, e
                            );
                        }
                    }
                }
                Ok(false) => {
                    info!(
                        "Skipping already processed submission '{}' for user {}",
                        submission.title, user
                    );
                }
                Err(e) => {
                    error!(
                        "Error checking submission status for '{}': {}",
                        submission.title, e
                    );
                }
            }
        }

        if update_timestamp {
            if let Err(e) = config
                .model
                .add_or_update_timestamp(&user, &latest_timestamp)
                .await
            {
                error!("Error updating timestamp for user {}: {}", user, e);
            } else {
                info!(
                    "Updated timestamp for user {} to {}",
                    user, latest_timestamp
                );
            }
        }
    }

    info!("Completed processing all users");
    Ok(json!({ "statusCode": 200, "message": "Processing completed successfully" }))
}
