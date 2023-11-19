use crate::service::Service;
use aws_lambda_events::sqs::SqsEventObj;
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub struct Handler {
    pub service: Service,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecentAcSubmission {
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "data")]
pub struct RecentAcSubmissionResp {
    #[serde(rename = "recentAcSubmissionList")]
    pub recent_ac_submission_list: Vec<RecentAcSubmission>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SQSMessage {
    pub username: String,
    pub user_avatar: String,
    pub submissions: RecentAcSubmissionResp,
}

impl Handler {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub async fn function_handler(
        &self,
        event: LambdaEvent<SqsEventObj<SQSMessage>>,
    ) -> Result<(), Error> {
        for record in event.payload.records {
            info!(
                "The message {} for event source {} = {:#?}",
                record.message_id.unwrap_or_default(),
                record.event_source.unwrap_or_default(),
                record.body
            );
            if let Err(e) = self.service.send_embed_to_discord(&record.body).await {
                error!(
                    "Error while sending Embed to discord for user {}. Error: {}",
                    record.body.username, e
                );
                continue;
            }
        }
        Ok(())
    }
}
