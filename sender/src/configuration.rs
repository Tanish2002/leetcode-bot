use super::{handler::Handler, service::Service};
use serenity::{http::Http, model::webhook::Webhook, prelude::SerenityError};
use std::env;
use tracing::{error, info};

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

        info!("Initializing LeetCode sender configuration");

        let webhook_url = Self::get_webhook_url().await;
        info!("Discord webhook URL loaded (masked for security)");

        let webhook_client = match Self::create_webhook_client(webhook_url).await {
            Ok(v) => {
                info!("Discord webhook client created successfully");
                v
            }
            Err(e) => {
                error!("Failed to create Discord webhook client: {}", e);
                panic!("Error while creating webhook client: {}", e);
            }
        };

        info!("Configuration initialized successfully");

        Self {
            handler: Handler::new(Service::new(webhook_client)),
        }
    }

    async fn get_webhook_url() -> String {
        match env::var("DISCORD_WEBHOOK_URL") {
            Ok(v) => v,
            Err(e) => {
                error!("DISCORD_WEBHOOK_URL env var missing: {}", e);
                panic!("DISCORD_WEBHOOK_URL env var missing, {}", e);
            }
        }
    }

    async fn create_webhook_client(url: String) -> Result<Webhook, SerenityError> {
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, url.as_str()).await?;
        Ok(webhook)
    }
}
