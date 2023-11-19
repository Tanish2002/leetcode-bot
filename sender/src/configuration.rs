use super::{handler::Handler, service::Service};
use serenity::{http::Http, model::webhook::Webhook, prelude::SerenityError};
use std::env;

pub struct Configuration {
    pub handler: Handler,
}

impl Configuration {
    pub async fn new() -> Self {
        // Initialize Logging
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            // disable printing the name of the module in every log line.
            .with_target(false)
            // disabling time is handy because CloudWatch will add the ingestion time.
            .without_time()
            .init();

        let webhook_url = Self::get_webhook_url().await;
        let webhook_client = match Self::create_webhook_client(webhook_url).await {
            Ok(v) => v,
            Err(e) => panic!("Error while creating webhook client. Error: {}", e),
        };
        Self {
            handler: Handler::new(Service::new(webhook_client)),
        }
    }

    async fn get_webhook_url() -> String {
        match env::var("DISCORD_WEBHOOK_URL") {
            Ok(v) => v,
            Err(e) => panic!("DISCORD_WEBHOOK_URL env var missing, {}", e),
        }
    }

    async fn create_webhook_client(url: String) -> Result<Webhook, SerenityError> {
        let http = Http::new("");
        Ok(Webhook::from_url(&http, url.as_str()).await?)
    }
}
