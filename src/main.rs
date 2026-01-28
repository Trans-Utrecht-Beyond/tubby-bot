mod cli;
mod config;
pub mod constants;
mod engine;
mod waha;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use dotenvy::dotenv;
use engine::dispatcher::Dispatcher;
use log::error;
use std::sync::Arc;
use waha::WahaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env and initialize logger
    dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();
    let config = Config::from_env().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    let config = Arc::new(config);

    match cli.command {
        Commands::Listen => {
            let mut dispatcher = Dispatcher::new();
            dispatcher.register_handler(Arc::new(engine::handlers::LoggingHandler {
                config: Arc::clone(&config),
            }));
            dispatcher.register_handler(Arc::new(engine::handlers::WahaSendSeenHandler::new(
                Arc::clone(&config),
            )));

            let dispatcher = Arc::new(dispatcher);
            // TODO: Register more handlers here as needed
            let waha_client = WahaClient::new((*config).clone(), dispatcher);
            waha_client.listen().await?;
        }
    }

    Ok(())
}
