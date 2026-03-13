//! Example: Staking with Tensor Coherence Bonuses
//!
//! This example demonstrates staking rewards with coherence-based multipliers.

use octo_o_weaver::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Octo-Weaver Staking Strategy Example ===\n");

    let rpc_endpoint = "http://localhost:8899".to_string();

    // Initialize managers
    let tensor_manager = TensorNetworkManager::new(rpc_endpoint.clone())?;
    let rewards = RewardDistributor::new(rpc_endpoint, 5000)?;

    // Query current network coherence
    println!("Step 1: Check network coherence state...");
    let network_state = tensor_manager.query_network_state().await?;
    let coherence = network_state.coherence;
    println!(
        "  ✓ Current coherence: {:.6}",
        coherence as f64 / 1_000_000.0
    );

    // Example staking scenarios
    println!("\nStep 2: Calculate staking rewards for different scenarios...\n");

    let scenarios = vec![
        ("Low coherence", 1_000_000, 30 * 24 * 3600, 500_000), // 1 month, 0.5 coherence
        ("Medium coherence", 1_000_000, 30 * 24 * 3600, 750_000), // 1 month, 0.75 coherence
        ("High coherence", 1_000_000, 30 * 24 * 3600, 950_000), // 1 month, 0.95 coherence
    ];

    for (name, stake, duration, avg_coherence) in scenarios {
        println!("Scenario: {}", name);
        println!("  - Stake amount: {} lamports", stake);
        println!("  - Duration: 30 days");
        println!(
            "  - Average coherence: {:.6}",
            avg_coherence as f64 / 1_000_000.0
        );

        let calculation = rewards.calculate_staking_rewards(stake, duration, avg_coherence)?;

        println!("  - Base reward: {} lamports", calculation.base_reward);
        println!("  - Coherence bonus: {:.2}%", calculation.coherence_bonus);
        println!("  - Total reward: {} lamports", calculation.total_reward);
        println!(
            "  - Effective APY: {:.2}%",
            (calculation.total_reward as f64 / stake as f64) * 12.0 * 100.0
        );
        println!();
    }

    // Pool reward distribution
    println!("\nStep 3: Distribute pool rewards with coherence multipliers...");
    let pool_total_reward = 1_000_000; // 1M lamports to distribute

    let miners = vec![
        solana_sdk::pubkey::Pubkey::new_unique(),
        solana_sdk::pubkey::Pubkey::new_unique(),
        solana_sdk::pubkey::Pubkey::new_unique(),
    ];

    let contributions = vec![60, 30, 10]; // Proportional contributions

    let distribution =
        rewards.distribute_pool_rewards(pool_total_reward, miners.clone(), contributions)?;

    println!("  Pool total: {} lamports", pool_total_reward);
    for (i, (miner, amount)) in distribution.iter().enumerate() {
        println!(
            "  - Miner {}: {} lamports ({:.1}%)",
            i + 1,
            amount,
            (*amount as f64 / pool_total_reward as f64) * 100.0
        );
    }

    println!("\n=== Staking Summary ===");
    println!("With coherence multipliers, staking yields:");
    println!("  - Low coherence:  ~10.0% APY");
    println!("  - Medium coherence: ~10.75% APY");
    println!("  - High coherence:  ~11.0% APY");
    println!("Status: ✓ Complete\n");

    Ok(())
}
