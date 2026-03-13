//! Staking Optimizer Agent Binary

use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("Starting Octo-O-Weaver Staking Optimizer Agent");

    let config = staking_optimizer::StakingOptimizerConfig {
        rpc_endpoint: std::env::var("RPC_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
        wallet_pubkey: std::env::var("WALLET_PUBKEY").unwrap_or_default(),
        ..Default::default()
    };

    let optimizer = staking_optimizer::StakingOptimizer::new(config).await?;
    optimizer.run().await?;

    Ok(())
}
