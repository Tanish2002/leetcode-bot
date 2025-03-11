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
    #[serde(default)]
    pub processed_at: Option<String>, // Added for tracking when processed
}

impl Handler {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub async fn function_handler(
        &self,
        event: LambdaEvent<SqsEventObj<SQSMessage>>,
    ) -> Result<(), Error> {
        info!("Processing {} SQS messages", event.payload.records.len());

        for record in event.payload.records {
            let message_id = record.message_id.unwrap_or_default();
            let event_source = record.event_source.unwrap_or_default();

            info!(
                "Processing message {} from event source {}",
                message_id, event_source
            );

            if record.body.submissions.recent_ac_submission_list.is_empty() {
                info!("Skipping message with empty submission list");
                continue;
            }

            // Log the submissions we're about to process
            for submission in &record.body.submissions.recent_ac_submission_list {
                info!(
                    "Processing submission: User={}, Problem='{}', Timestamp={}",
                    record.body.username, submission.title, submission.timestamp
                );
            }

            match self.service.send_embed_to_discord(&record.body).await {
                Ok(_) => {
                    info!(
                        "Successfully sent Discord embed for user {}",
                        record.body.username
                    );
                }
                Err(e) => {
                    error!(
                        "Error sending Discord embed for user {}: {}",
                        record.body.username, e
                    );

                    // Note: By default, returning an error from this function would cause
                    // the Lambda runtime to fail this invocation and retry the message.
                    // For this application, we'll log the error but continue processing
                    // other messages rather than failing the entire batch.
                }
            }
        }

        Ok(())
    }
}
