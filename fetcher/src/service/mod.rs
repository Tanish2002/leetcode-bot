mod leetcode;
mod sqs;

use aws_sdk_sqs::Client;
use leetcode::Leetcode;
use sqs::SQS;

pub struct Service<'a> {
    pub sqs: SQS,
    pub leetcode: Leetcode<'a>, // leetcode_api_url: todo!(),
}

impl Service<'_> {
    pub fn new(sqs_url: String, sqs_client: Client) -> Self {
        Self {
            sqs: SQS::new(sqs_url, sqs_client),
            leetcode: Leetcode::new(),
        }
    }
}
