use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
use std::collections::HashMap;
use tracing::{info, warn};

pub struct Model {
    timestamp_table_name: String,
    dynamodb_client: Client,
}

impl Model {
    pub fn new(timestamp_table_name: String, dynamodb_client: Client) -> Self {
        Self {
            timestamp_table_name,
            dynamodb_client,
        }
    }

    pub async fn add_or_update_timestamp(
        &self,
        user: &String,
        timestamp: &String,
    ) -> Result<(), Error> {
        let mut item = HashMap::new();
        item.insert(
            "PK".to_string(),
            AttributeValue::S(format!("USER#{}", user)),
        );
        item.insert("SK".to_string(), AttributeValue::S("TIMESTAMP".to_string()));
        item.insert(
            "Timestamp".to_string(),
            AttributeValue::S(timestamp.to_string()),
        );
        item.insert("Username".to_string(), AttributeValue::S(user.to_string()));
        item.insert(
            "GSI1PK".to_string(),
            AttributeValue::S("TIMESTAMP".to_string()),
        );
        item.insert("GSI1SK".to_string(), AttributeValue::S(user.to_string()));

        let result = self
            .dynamodb_client
            .put_item()
            .table_name(&self.timestamp_table_name)
            .set_item(Some(item))
            .send()
            .await;

        match result {
            Ok(_) => {
                info!("Updated timestamp for user {} to {}", user, timestamp);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to update timestamp for user {}: {}", user, e);
                Err(e.into())
            }
        }
    }

    pub async fn get_latest_timestamp(&self, user: &str) -> Result<String, Error> {
        let pk = AttributeValue::S(format!("USER#{}", user));
        let sk = AttributeValue::S("TIMESTAMP".to_string());

        let resp = self
            .dynamodb_client
            .get_item()
            .table_name(&self.timestamp_table_name)
            .key("PK", pk)
            .key("SK", sk)
            .send()
            .await?;

        let timestamp = resp
            .item()
            .and_then(|item| item.get("Timestamp"))
            .and_then(|timestamp| timestamp.as_s().ok())
            .map(|timestamp_s| timestamp_s.to_string())
            .unwrap_or_else(|| "".to_string());

        info!(
            "Retrieved latest timestamp for user {}: {}",
            user,
            if timestamp.is_empty() {
                "none (new user)"
            } else {
                &timestamp
            }
        );

        Ok(timestamp)
    }

    // Record that we've processed a submission and check if it's already been processed
    pub async fn record_processed_submission(
        &self,
        user: &str,
        problem_slug: &str,
        timestamp: &str,
    ) -> Result<bool, Error> {
        let submission_id = format!("{}_{}", problem_slug, timestamp);
        // Calculate TTL - 7 days from now
        let ttl = (chrono::Utc::now() + chrono::Duration::days(7)).timestamp();

        let mut item = HashMap::new();
        // Primary key format: SUBMISSION#slug_timestamp
        item.insert(
            "PK".to_string(),
            AttributeValue::S(format!("SUBMISSION#{}", submission_id)),
        );
        item.insert(
            "SK".to_string(),
            AttributeValue::S(format!("USER#{}", user)),
        );
        item.insert(
            "ProblemSlug".to_string(),
            AttributeValue::S(problem_slug.to_string()),
        );
        item.insert(
            "SubmissionTime".to_string(),
            AttributeValue::S(timestamp.to_string()),
        );
        item.insert(
            "GSI1PK".to_string(),
            AttributeValue::S(format!("USER#{}", user)),
        );
        item.insert(
            "GSI1SK".to_string(),
            AttributeValue::S(format!("SUBMISSION#{}", submission_id)),
        );
        item.insert("TTL".to_string(), AttributeValue::N(ttl.to_string()));

        let result = self
            .dynamodb_client
            .put_item()
            .table_name(&self.timestamp_table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(PK)")
            .send()
            .await;

        match result {
            Ok(_) => {
                info!(
                    "Recorded new submission: User={}, Problem={}, Timestamp={}",
                    user, problem_slug, timestamp
                );
                Ok(true)
            }
            Err(err) => {
                if let aws_sdk_dynamodb::error::SdkError::ServiceError(context) = &err {
                    if context.err().is_conditional_check_failed_exception() {
                        info!(
                            "Submission already processed: User={}, Problem={}, Timestamp={}",
                            user, problem_slug, timestamp
                        );
                        return Ok(false);
                    }
                }
                warn!("Error recording submission: {}", err);
                Err(err.into())
            }
        }
    }
}
