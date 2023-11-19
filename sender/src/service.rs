use serenity::{
    http::Http,
    model::{prelude::Embed, webhook::Webhook},
    prelude::SerenityError,
};

use crate::handler::SQSMessage;
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
            embed_fields.push((
                submission.title.clone(),
                format!(
                    "[Link](https://leetcode.com/problems/{})",
                    submission.title_slug
                ),
                false,
            ));
        }
        let http = Http::new("");
        self.client
            .execute(&http, false, |w| {
                let embed = Embed::fake(|e| {
                    e.title(format!("{}'s New Submissions", message.username))
                        .thumbnail(&message.user_avatar)
                        .author(|a| a.name(&message.username).icon_url(&message.user_avatar))
                        .description(format!(
                            "{} has done these questions in the last hour",
                            message.username
                        ))
                        .fields(embed_fields)
                });

                w.username("Leetcode")
                    .avatar_url(
                        "https://upload.wikimedia.org/wikipedia/commons/8/8e/LeetCode_Logo_1.png",
                    )
                    .embeds(vec![embed])
            })
            .await?;
        Ok(())
    }
}
