use aws_sdk_dynamodb as dynamodb;
use std::env;

use crate::models::Model;
use crate::service::Service;

pub struct Configuration<'a> {
    pub users: Vec<String>,
    pub model: Model,
    pub service: Service<'a>,
}

impl Configuration<'_> {
    pub async fn new<'a>() -> Configuration<'a> {
        // Initialize Logger
        env_logger::init();

        let config = aws_config::load_from_env().await;
        let dynamo_client = dynamodb::Client::new(&config);
        let sqs_client = aws_sdk_sqs::Client::new(&config);

        Configuration {
            users: Self::get_users().await,
            model: Model::new(Self::get_table_name().await, dynamo_client),
            service: Service::new(Self::get_sqs_url().await, sqs_client),
        }
    }

    async fn get_users() -> Vec<String> {
        match env::var("USERS") {
            Ok(v) => v.split(",").map(String::from).collect(),
            Err(e) => panic!("USERS env var missing, {}", e),
        }
    }

    async fn get_sqs_url() -> String {
        match env::var("SQS_QUEUE_URL") {
            Ok(v) => v,
            Err(e) => panic!("SQS_QUEUE_URL env var missing, {}", e),
        }
    }

    async fn get_table_name() -> String {
        match env::var("DYNAMODB_TABLE_NAME") {
            Ok(v) => v,
            Err(e) => panic!("DYNAMODB_TABLE_NAME env var missing, {}", e),
        }
    }
}
