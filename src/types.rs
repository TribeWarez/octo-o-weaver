//! Shared types for Octo-Weaver tentacles

use ai3_lib::MinerCapabilities;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Represents a miner in the TribeWarez network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miner {
    pub pubkey: Pubkey,
    pub capabilities: MinerCapabilities,
    pub reputation: u64,
    pub total_proofs: u64,
    pub accepted_proofs: u64,
    pub rejected_proofs: u64,
}

/// Proof submission response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSubmissionResult {
    pub transaction_hash: String,
    pub proof_id: String,
    pub status: ProofStatus,
    pub timestamp: u64,
}

/// Status of a proof
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    Pending,
    Accepted,
    Rejected,
    Verified,
}

/// Tensor network query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorNetworkSnapshot {
    pub total_entropy: u64, // Fixed-point scale 1e6
    pub coherence: u64,     // Fixed-point scale 1e6
    pub entanglement_pairs: u64,
    pub network_efficiency: u64, // Fixed-point scale 1e6
    pub timestamp: u64,
}

/// Reward calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculation {
    pub base_reward: u64,
    pub tensor_multiplier: u64, // Fixed-point scale 1e6
    pub coherence_bonus: u64,
    pub total_reward: u64,
    pub currency: String,
}

/// Pool strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolStrategyConfig {
    Solo {
        miner_pubkey: Pubkey,
    },
    Proportional {
        pool_id: Pubkey,
        fee_percent: u8,
    },
    PPLNS {
        pool_id: Pubkey,
        fee_percent: u8,
        share_multiplier: u64,
    },
}

/// ROI calculation for a pool strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolROI {
    pub strategy: String,
    pub daily_return: f64,
    pub monthly_return: f64,
    pub annual_return: f64,
    pub projected_revenue: u64,
}

/// Cross-chain bridge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRequest {
    pub source_chain: String,
    pub target_chain: String,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub recipient: Pubkey,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: Pubkey,
    pub voting_power: u64,
    pub status: ProposalStatus,
}

/// Status of a governance proposal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Failed,
    Executed,
    Canceled,
}

/// Liquidity pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPoolInfo {
    pub pool_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_supply: u64,
    pub fee_tier: u16, // basis points (e.g., 30 = 0.30%)
}

/// Swap quote result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuote {
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub price_impact: u64, // Fixed-point scale 1e6
    pub swap_fee: u64,
    pub execution_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_status_serialization() {
        let status = ProofStatus::Accepted;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Accepted\"");
    }

    #[test]
    fn test_proposal_status_comparison() {
        let status1 = ProposalStatus::Active;
        let status2 = ProposalStatus::Active;
        assert_eq!(status1, status2);
    }
}
