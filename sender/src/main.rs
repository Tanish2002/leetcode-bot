use lambda_runtime::{run, service_fn, Error};
mod configuration;
mod handler;
mod service;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = configuration::Configuration::new().await;
    run(service_fn(|req| config.handler.function_handler(req))).await
}
