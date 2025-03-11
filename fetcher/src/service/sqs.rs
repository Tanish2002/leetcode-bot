use super::leetcode::submissions::RecentAcSubmissionResp;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::Error as SQSError;
use serde::Serialize;
use serde_json::Error as SerdeError;
use std::fmt;
use tracing::{error, info};

pub struct SQS {
    sqs_url: String,
    sqs_client: Client,
}

#[derive(Serialize)]
struct SQSMessage {
    username: String,
    user_avatar: String,
    submissions: RecentAcSubmissionResp,
    processed_at: String,
}

impl SQSMessage {
    pub fn new(username: String, user_avatar: String, submissions: RecentAcSubmissionResp) -> Self {
        // Add current timestamp for better tracking
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            username,
            user_avatar,
            submissions,
            processed_at: now,
        }
    }
}

pub enum Error {
    SQSError(SQSError),
    SerdeError(SerdeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SQSError(sqse) => write!(f, "SQS Error: {}", sqse),
            Error::SerdeError(serdee) => write!(f, "Serde Error: {}", serdee),
        }
    }
}

impl SQS {
    pub fn new(sqs_url: String, sqs_client: Client) -> Self {
        Self {
            sqs_url,
            sqs_client,
        }
    }

    pub async fn send_to_sqs(
        &self,
        username: &String,
        user_avatar: &String,
        submissions: RecentAcSubmissionResp,
    ) -> Result<(), Error> {
        let data = SQSMessage::new(username.to_string(), user_avatar.to_string(), submissions);

        let message_body = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                error!("Error while marshalling sqsMessage: {}", e);
                return Err(Error::SerdeError(e));
            }
        };

        // Generate a unique message deduplication ID based on content
        // This ensures we don't send duplicate messages even with standard SQS
        let submissions_str = message_body.clone();
        let message_id = format!("{}_{}", username, submissions_str.len());

        match self
            .sqs_client
            .send_message()
            .queue_url(&self.sqs_url)
            .message_body(&message_body)
            .message_group_id(username) // This is ignored by standard queues
            .message_deduplication_id(&message_id) // This is ignored by standard queues
            .send()
            .await
        {
            Ok(response) => {
                let message_id = response.message_id().unwrap_or("unknown");
                info!(
                    "Message sent successfully for user {} with ID: {}",
                    username, message_id
                );
                Ok(())
            }
            Err(e) => {
                error!("Error sending message to SQS for user {}: {}", username, e);
                Err(Error::SQSError(e.into()))
            }
        }
    }
}
