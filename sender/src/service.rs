use crate::handler::SQSMessage;
use chrono::DateTime;
use serenity::{
    http::Http,
    model::{prelude::Embed, webhook::Webhook},
    prelude::SerenityError,
};
use tracing::{error, info};

pub struct Service {
    client: Webhook,
}

impl Service {
    pub fn new(client: Webhook) -> Self {
        Self { client }
    }

    pub async fn send_embed_to_discord(&self, message: &SQSMessage) -> Result<(), SerenityError> {
        let mut embed_fields: Vec<(String, String, bool)> = Vec::new();

        for submission in &message.submissions.recent_ac_submission_list {
            // Format the timestamp as a readable date/time
            let timestamp_i64 = submission.timestamp.parse::<i64>().unwrap_or(0);
            let date_time = DateTime::from_timestamp(timestamp_i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| "Unknown time".to_string());

            embed_fields.push((
                submission.title.clone(),
                format!(
                    "[Link](https://leetcode.com/problems/{}) | Completed: {}",
                    submission.title_slug, date_time
                ),
                false,
            ));
        }

        // Get current timestamp for the embed footer
        let now = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let http = Http::new("");

        info!(
            "Sending Discord embed with {} submissions for user {}",
            message.submissions.recent_ac_submission_list.len(),
            message.username
        );

        match self
            .client
            .execute(&http, false, |w| {
                let embed = Embed::fake(|e| {
                    e.title(format!("{}'s New LeetCode Submission", message.username))
                        .thumbnail(&message.user_avatar)
                        .author(|a| a.name(&message.username).icon_url(&message.user_avatar))
                        .description(format!(
                            "{} has completed the following problem",
                            message.username
                        ))
                        .fields(embed_fields)
                        .footer(|f| f.text(format!("Processed at {}", now)))
                        .timestamp(now)
                        .color(0x00FF00) // Green color for success
                });
                w.username("LeetCode Bot")
                    .avatar_url(
                        "https://upload.wikimedia.org/wikipedia/commons/8/8e/LeetCode_Logo_1.png",
                    )
                    .embeds(vec![embed])
            })
            .await
        {
            Ok(_) => {
                info!(
                    "Successfully sent Discord embed for user {}",
                    message.username
                );
                Ok(())
            }
            Err(e) => {
                error!("Error sending Discord embed: {}", e);
                Err(e)
            }
        }
    }
}
