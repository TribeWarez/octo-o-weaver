//! Tensor Miner Agent Binary

use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("Starting Octo-O-Weaver Tensor Miner Agent");

    let config = tensor_miner::TensorMinerConfig {
        rpc_endpoint: std::env::var("RPC_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
        wallet_pubkey: std::env::var("WALLET_PUBKEY").unwrap_or_default(),
        ..Default::default()
    };

    let miner = tensor_miner::TensorMiner::new(config, config.wallet_pubkey.clone()).await?;
    miner.run().await?;

    Ok(())
}
