pub mod leetcode;
mod sqs;
use aws_sdk_sqs::Client;
use leetcode::Leetcode;
use sqs::SQS;
use tracing::info;

pub struct Service<'a> {
    pub sqs: SQS,
    pub leetcode: Leetcode<'a>,
}

impl Service<'_> {
    pub fn new(sqs_url: String, sqs_client: Client) -> Self {
        info!("Initializing Service with SQS URL: {}", sqs_url);
        Self {
            sqs: SQS::new(sqs_url, sqs_client),
            leetcode: Leetcode::new(),
        }
    }
}
