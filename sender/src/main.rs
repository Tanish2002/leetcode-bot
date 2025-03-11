use lambda_runtime::{run, service_fn, Error};
use tracing::{error, info};

mod configuration;
mod handler;
mod service;

#[tokio::main]
async fn main() -> Result<(), Error> {
    info!("Starting LeetCode sender Lambda");

    // Initialize the configuration
    let config = match configuration::Configuration::new().await {
        config => {
            info!("Configuration initialized successfully");
            config
        }
    };

    // Run the Lambda handler
    info!("Starting Lambda runtime");
    match run(service_fn(|req| config.handler.function_handler(req))).await {
        Ok(_) => {
            info!("Lambda runtime completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Lambda runtime error: {}", e);
            Err(e)
        }
    }
}
