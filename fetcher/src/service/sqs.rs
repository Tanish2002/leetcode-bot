use std::fmt;

use super::leetcode::submissions::RecentAcSubmissionResp;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::Error as SQSError;
use serde::Serialize;
use serde_json::Error as SerdeError;
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
}

impl SQSMessage {
    pub fn new(username: String, user_avatar: String, submissions: RecentAcSubmissionResp) -> Self {
        Self {
            username,
            user_avatar,
            submissions,
        }
    }
}

pub enum Error {
    SQSError(SQSError),     // can't do this as ErroSQS is a enum
    SerdeError(SerdeError), // this is fine.
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

        match self
            .sqs_client
            .send_message()
            .queue_url(&self.sqs_url)
            .message_body(&message_body)
            .send()
            .await
        {
            Ok(_) => {
                info!("Message sent successfully!");
                return Ok(());
            }
            Err(e) => {
                error!("Error sending message to SQS: {}", e);
                return Err(Error::SQSError(e.into()));
            }
        }
    }
}
