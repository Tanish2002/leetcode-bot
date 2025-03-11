use crate::models::Model;
use crate::service::Service;
use aws_sdk_dynamodb as dynamodb;
use std::env;
use tracing::{error, info};

pub struct Configuration<'a> {
    pub users: Vec<String>,
    pub model: Model,
    pub service: Service<'a>,
}

impl Configuration<'_> {
    pub async fn new<'a>() -> Configuration<'a> {
        // Initialize Logger
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            // disable printing the name of the module in every log line.
            .with_target(false)
            // disabling time is handy because CloudWatch will add the ingestion time.
            .without_time()
            .init();

        info!("Initializing LeetCode fetcher configuration");

        // Load AWS config
        let config = aws_config::load_from_env().await;
        info!("AWS configuration loaded");

        // Initialize AWS clients
        let dynamo_client = dynamodb::Client::new(&config);
        let sqs_client = aws_sdk_sqs::Client::new(&config);
        info!("AWS clients initialized");

        // Get environment variables
        let users = Self::get_users().await;
        let table_name = Self::get_table_name().await;
        let sqs_url = Self::get_sqs_url().await;

        info!(
            "Configuration initialized with {} users, table: {}, SQS: {}",
            users.len(),
            table_name,
            sqs_url
        );

        Configuration {
            users,
            model: Model::new(table_name, dynamo_client),
            service: Service::new(sqs_url, sqs_client),
        }
    }

    async fn get_users() -> Vec<String> {
        match env::var("USERS") {
            Ok(v) => {
                let users: Vec<String> = v.split(",").map(|s| s.trim().to_string()).collect();
                info!("Loaded {} users from environment: {}", users.len(), v);
                users
            }
            Err(e) => {
                error!("USERS env var missing, {}", e);
                panic!("USERS env var missing, {}", e);
            }
        }
    }

    async fn get_sqs_url() -> String {
        match env::var("SQS_QUEUE_URL") {
            Ok(v) => {
                info!("Loaded SQS URL from environment: {}", v);
                v
            }
            Err(e) => {
                error!("SQS_QUEUE_URL env var missing, {}", e);
                panic!("SQS_QUEUE_URL env var missing, {}", e);
            }
        }
    }

    async fn get_table_name() -> String {
        match env::var("DYNAMODB_TABLE_NAME") {
            Ok(v) => {
                info!("Loaded DynamoDB table name from environment: {}", v);
                v
            }
            Err(e) => {
                error!("DYNAMODB_TABLE_NAME env var missing, {}", e);
                panic!("DYNAMODB_TABLE_NAME env var missing, {}", e);
            }
        }
    }
}
