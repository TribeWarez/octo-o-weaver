//! # Octo-Weaver: PaaS Abstraction Layer for TribeWarez v0.4.0
//!
//! Octo-Weaver unifies the TribeWarez ecosystem (9 Anchor programs + 4 off-chain core crates)
//! into a single, coherent interface for clients. It provides 8 "tentacles" that solve key DeFi
//! problems:
//!
//! ## 8 Core Tentacles
//!
//! 1. **Proof Orchestration** - Unified proof submission & validation (MUST-HAVE)
//! 2. **Miner Lifecycle** - Register, track capabilities, reputation (MUST-HAVE)
//! 3. **Tensor Network Management** - Query entropy, entanglement, coherence (MUST-HAVE)
//! 4. **Reward Distribution** - Calculate & distribute with tensor multipliers (MUST-HAVE)
//! 5. **Pool Strategy Abstraction** - Solo/Proportional/PPLNS pools, ROI (MUST-HAVE)
//! 6. **Cross-Chain Bridge** - Token wrapping, signature verification (NICE-TO-HAVE)
//! 7. **Governance & Treasury** - Proposals, voting, execution (NICE-TO-HAVE)
//! 8. **Liquidity & Swap Engine** - Quoting, multi-hop routing, TWAP (NICE-TO-HAVE)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use octo_o_weaver::prelude::*;
//! use solana_sdk::pubkey::Pubkey;
//!
//! // Example: Submit a proof and track mining rewards
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut orchestrator = ProofOrchestrator::new("http://localhost:8899".to_string())?;
//! let mut miner_manager = MinerLifecycle::new("http://localhost:8899".to_string())?;
//! let tensor_manager = TensorNetworkManager::new("http://localhost:8899".to_string())?;
//! let mut reward_dist = RewardDistributor::new("http://localhost:8899".to_string(), 1000)?;
//!
//! let pubkey = Pubkey::new_unique();
//! let caps = MinerCapabilities {
//!     supported_operations: vec!["matrix_multiply".into()],
//!     max_tensor_size: 4096,
//!     is_esp_device: false,
//!     max_computation_time: 300,
//! };
//! let _miner = miner_manager.register_miner(pubkey, caps)?;
//! # Ok(())
//! # }
//! ```

// Re-export core types from dependencies
pub use ai3_lib::{MinerCapabilities, TensorEngine};
pub use pot_o_core::{Block, TensorNetworkState, Transaction};
pub use pot_o_extensions::ChainBridge;
pub use pot_o_mining::{Challenge, PotOConsensus, PotOProof, ProofPayload};

pub mod error;
pub mod types;

// MUST-HAVE Tentacles
pub mod miner_lifecycle;
pub mod pool_strategy;
pub mod proof_orchestration;
pub mod reward_distribution;
pub mod tensor_network;

// NICE-TO-HAVE Tentacles
pub mod cross_chain_bridge;
pub mod governance;
pub mod liquidity;

// Prelude for convenient imports
pub mod prelude {
    pub use crate::cross_chain_bridge::CrossChainBridge;
    pub use crate::error::{OctoError, OctoResult};
    pub use crate::governance::GovernanceManager;
    pub use crate::liquidity::LiquidityEngine;
    pub use crate::miner_lifecycle::MinerLifecycle;
    pub use crate::pool_strategy::PoolStrategyManager;
    pub use crate::proof_orchestration::ProofOrchestrator;
    pub use crate::reward_distribution::RewardDistributor;
    pub use crate::tensor_network::TensorNetworkManager;
    pub use crate::types::*;
    // Re-export commonly used types from dependencies
    pub use ai3_lib::MinerCapabilities;
    pub use pot_o_core::{Block, TensorNetworkState, Transaction};
    pub use pot_o_mining::{Challenge, ProofPayload};
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get version information
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), "0.1.0");
    }
}
