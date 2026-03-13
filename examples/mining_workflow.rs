//! Example: End-to-End Mining Workflow
//!
//! This example demonstrates a complete mining workflow:
//! 1. Register a miner
//! 2. Query network state
//! 3. Determine optimal pool strategy
//! 4. Submit proofs
//! 5. Calculate and distribute rewards

use octo_o_weaver::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Octo-Weaver Mining Workflow Example ===\n");

    // Initialize RPC endpoint
    let rpc_endpoint = "http://localhost:8899".to_string();

    // 1. Register a miner
    println!("Step 1: Register miner...");
    let mut miner_manager = MinerLifecycle::new(rpc_endpoint.clone())?;

    let miner_pubkey = Pubkey::new_unique();
    let capabilities = MinerCapabilities {
        supported_operations: vec!["matrix_multiply".into(), "convolution".into()],
        max_tensor_size: 64 * 64 * 4,
        is_esp_device: false,
        max_computation_time: 300,
    };

    let miner = miner_manager.register_miner(miner_pubkey, capabilities)?;
    println!("  ✓ Miner registered: {}", miner.pubkey);
    println!("  - Reputation: {}", miner.reputation);
    println!("  - Is ESP Device: {}", miner.capabilities.is_esp_device);
    println!(
        "  - Max Tensor Size: {}",
        miner.capabilities.max_tensor_size
    );

    // 2. Query network state
    println!("\nStep 2: Query tensor network state...");
    let tensor_manager = TensorNetworkManager::new(rpc_endpoint.clone())?;
    let network_state = tensor_manager.query_network_state().await?;
    println!(
        "  ✓ Network entropy: {:.6}",
        network_state.total_entropy as f64 / 1_000_000.0
    );
    println!(
        "  ✓ Network coherence: {:.6}",
        network_state.coherence as f64 / 1_000_000.0
    );
    println!(
        "  ✓ Entanglement pairs: {}",
        network_state.entanglement_pairs
    );

    // 3. Determine optimal pool strategy
    println!("\nStep 3: Determine optimal pool strategy...");
    let pool_manager = PoolStrategyManager::new(rpc_endpoint.clone())?;
    let strategy = pool_manager.get_optimal_strategy(miner_pubkey, 5_000_000, 5)?;
    println!("  ✓ Recommended strategy: {:?}", strategy);

    // Calculate ROI
    let roi = pool_manager.calculate_roi(&strategy, 100, 1000)?;
    println!("  ✓ Daily return: {:.2}", roi.daily_return);
    println!("  ✓ Monthly return: {:.2}", roi.monthly_return);
    println!("  ✓ Annual return: {:.2}", roi.annual_return);

    // 4. Submit proofs (simulated)
    println!("\nStep 4: Submit proofs...");
    let mut orchestrator = ProofOrchestrator::new(rpc_endpoint.clone())?;

    // In a real scenario, these would be actual proofs from the mining device
    for i in 0..5 {
        println!("  - Submitting proof {}...", i + 1);
        // Simulate proof submission
        miner_manager.record_proof_acceptance(&miner_pubkey)?;
    }
    println!("  ✓ 5 proofs submitted");

    // 5. Calculate and distribute rewards
    println!("\nStep 5: Calculate and distribute rewards...");
    let rewards = RewardDistributor::new(rpc_endpoint.clone(), 5000)?; // 5000 lamports per proof

    let calculation = rewards.calculate_miner_rewards(
        &miner_pubkey,
        5,         // 5 accepted proofs
        1_150_000, // 1.15x tensor multiplier (coherence bonus)
        50_000,    // 0.05 coherence bonus
    )?;

    println!("  ✓ Base reward: {} lamports", calculation.base_reward);
    println!(
        "  ✓ Tensor multiplier: {:.3}x",
        calculation.tensor_multiplier as f64 / 1_000_000.0
    );
    println!(
        "  ✓ Coherence bonus: {:.6}",
        calculation.coherence_bonus as f64 / 1_000_000.0
    );
    println!("  ✓ Total reward: {} lamports", calculation.total_reward);

    // 6. Check final miner status
    println!("\nStep 6: Final miner status...");
    let updated_miner = miner_manager.get_miner(&miner_pubkey)?;
    println!("  ✓ Total proofs: {}", updated_miner.total_proofs);
    println!("  ✓ Accepted proofs: {}", updated_miner.accepted_proofs);
    println!("  ✓ Reputation: {}", updated_miner.reputation);

    // 7. Summary
    println!("\n=== Mining Session Summary ===");
    println!("Period: 1 hour (simulated)");
    println!("Proofs submitted: {}", updated_miner.total_proofs);
    println!("Reward earned: {} lamports", calculation.total_reward);
    println!(
        "Estimated daily: {} lamports",
        calculation.total_reward * 24
    );
    println!("Status: ✓ Complete\n");

    Ok(())
}
