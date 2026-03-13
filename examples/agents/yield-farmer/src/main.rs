//! Yield Farmer Agent Binary

use octo_o_weaver::prelude::*;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("Starting Octo-O-Weaver Yield Farmer Agent");

    // Load config from environment or use defaults
    let config = yield_farmer::YieldFarmerConfig {
        rpc_endpoint: std::env::var("RPC_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
        wallet_pubkey: std::env::var("WALLET_PUBKEY").unwrap_or_default(),
        ..Default::default()
    };

    // Create and run the farmer
    let farmer = yield_farmer::YieldFarmer::new(config).await?;

    // Run the main loop
    farmer.run().await?;

    Ok(())
}
