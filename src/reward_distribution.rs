//! Reward Distribution Tentacle
//!
//! Calculates and distributes rewards with tensor-based multipliers.
//! This is a MUST-HAVE tentacle that handles mining rewards and incentives.

use crate::error::{OctoError, OctoResult};
use crate::types::RewardCalculation;
use solana_sdk::pubkey::Pubkey;

/// Reward distributor for calculating and managing mining rewards
#[derive(Debug, Clone)]
pub struct RewardDistributor {
    rpc_endpoint: String,
    base_reward_rate: u64, // Base reward in lamports per proof
}

impl RewardDistributor {
    /// Create a new reward distributor
    pub fn new(rpc_endpoint: String, base_reward_rate: u64) -> OctoResult<Self> {
        if rpc_endpoint.is_empty() {
            return Err(OctoError::InvalidConfig(
                "RPC endpoint cannot be empty".to_string(),
            ));
        }

        if base_reward_rate == 0 {
            return Err(OctoError::InvalidConfig(
                "Base reward rate must be greater than 0".to_string(),
            ));
        }

        Ok(RewardDistributor {
            rpc_endpoint,
            base_reward_rate,
        })
    }

    /// Calculate rewards for a miner
    pub fn calculate_miner_rewards(
        &self,
        miner: &Pubkey,
        accepted_proofs: u64,
        tensor_multiplier: u64, // Fixed-point scale 1e6
        coherence_bonus: u64,   // Fixed-point scale 1e6
    ) -> OctoResult<RewardCalculation> {
        // Validate inputs
        if tensor_multiplier > 2_000_000 {
            return Err(OctoError::ValidationError(
                "Tensor multiplier exceeds maximum (2.0)".to_string(),
            ));
        }

        if coherence_bonus > 100_000 {
            return Err(OctoError::ValidationError(
                "Coherence bonus exceeds maximum (0.1)".to_string(),
            ));
        }

        // Base reward = base_reward_rate * accepted_proofs
        let base_reward = self.base_reward_rate.saturating_mul(accepted_proofs);

        // Tensor multiplier applied: base_reward * (tensor_multiplier / 1e6)
        let tensor_adjusted =
            (base_reward as u128).saturating_mul(tensor_multiplier as u128) / 1_000_000u128;

        // Coherence bonus applied on top: tensor_adjusted * (1 + coherence_bonus/1e6)
        let total_reward = ((tensor_adjusted as u128)
            .saturating_mul(1_000_000 + coherence_bonus as u128)
            / 1_000_000u128) as u64;

        Ok(RewardCalculation {
            base_reward,
            tensor_multiplier,
            coherence_bonus,
            total_reward,
            currency: "SOL".to_string(),
        })
    }

    /// Calculate pool rewards distribution
    pub fn distribute_pool_rewards(
        &self,
        total_reward: u64,
        miners: Vec<Pubkey>,
        contributions: Vec<u64>, // Each miner's contribution
    ) -> OctoResult<Vec<(Pubkey, u64)>> {
        // Validate inputs
        if miners.len() != contributions.len() {
            return Err(OctoError::ValidationError(
                "Miners and contributions length mismatch".to_string(),
            ));
        }

        if miners.is_empty() {
            return Err(OctoError::ValidationError(
                "Must have at least one miner".to_string(),
            ));
        }

        // Calculate total contributions
        let total_contribution: u64 = contributions.iter().sum();
        if total_contribution == 0 {
            return Err(OctoError::ValidationError(
                "Total contributions must be greater than 0".to_string(),
            ));
        }

        // Distribute proportionally
        let mut distributions = Vec::new();
        let mut distributed = 0u64;

        for (i, miner) in miners.iter().enumerate() {
            let share = (total_reward as u128).saturating_mul(contributions[i] as u128)
                / (total_contribution as u128);
            let share = share as u64;
            distributions.push((*miner, share));
            distributed = distributed.saturating_add(share);
        }

        // Ensure no dust remains by adding remainder to largest contributor
        if distributed < total_reward {
            let remainder = total_reward - distributed;
            let max_idx = contributions
                .iter()
                .enumerate()
                .max_by_key(|&(_, &v)| v)
                .map(|(i, _)| i)
                .unwrap_or(0);
            distributions[max_idx].1 = distributions[max_idx].1.saturating_add(remainder);
        }

        Ok(distributions)
    }

    /// Calculate rewards with staking coherence multiplier
    pub fn calculate_staking_rewards(
        &self,
        stake_amount: u64,
        duration_seconds: u64,
        average_coherence: u64, // Fixed-point scale 1e6 (0-1e6)
    ) -> OctoResult<RewardCalculation> {
        // Validate coherence is in valid range
        if average_coherence > 1_000_000 {
            return Err(OctoError::ValidationError(
                "Coherence exceeds maximum (1.0)".to_string(),
            ));
        }

        // Base APY: 10% annually
        let seconds_per_year = 365 * 24 * 3600;
        let base_reward = (stake_amount as u128)
            .saturating_mul(10)
            .saturating_mul(duration_seconds as u128)
            / (100 * seconds_per_year as u128);

        // Coherence bonus: 0-10% additional
        let coherence_bonus_percent = (average_coherence * 10) / 1_000_000;
        let coherence_adjusted =
            (base_reward as u128).saturating_mul(100 + coherence_bonus_percent as u128) / 100u128;

        Ok(RewardCalculation {
            base_reward: base_reward as u64,
            tensor_multiplier: 1_000_000,
            coherence_bonus: coherence_bonus_percent,
            total_reward: coherence_adjusted as u64,
            currency: "SOL".to_string(),
        })
    }

    /// Validate reward calculations (sanity checks)
    pub fn validate_rewards(&self, calculation: &RewardCalculation) -> OctoResult<()> {
        if calculation.total_reward < calculation.base_reward {
            return Err(OctoError::ValidationError(
                "Total reward cannot be less than base reward".to_string(),
            ));
        }

        Ok(())
    }

    /// Set base reward rate
    pub fn set_base_reward_rate(&mut self, rate: u64) -> OctoResult<()> {
        if rate == 0 {
            return Err(OctoError::InvalidConfig(
                "Base reward rate must be greater than 0".to_string(),
            ));
        }
        self.base_reward_rate = rate;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributor_creation() {
        let distributor = RewardDistributor::new("http://localhost:8899".to_string(), 1000);
        assert!(distributor.is_ok());
    }

    #[test]
    fn test_invalid_base_reward() {
        let distributor = RewardDistributor::new("http://localhost:8899".to_string(), 0);
        assert!(distributor.is_err());
    }

    #[test]
    fn test_miner_rewards_calculation() {
        let distributor =
            RewardDistributor::new("http://localhost:8899".to_string(), 1000).unwrap();
        let miner = Pubkey::new_unique();
        let result = distributor.calculate_miner_rewards(&miner, 100, 1_200_000, 50_000);
        assert!(result.is_ok());

        let calc = result.unwrap();
        assert_eq!(calc.base_reward, 100_000); // 1000 * 100
        assert!(calc.total_reward > calc.base_reward);
    }

    #[test]
    fn test_pool_distribution() {
        let distributor =
            RewardDistributor::new("http://localhost:8899".to_string(), 1000).unwrap();
        let miners = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        let contributions = vec![60, 40];
        let result = distributor.distribute_pool_rewards(1000, miners, contributions);
        assert!(result.is_ok());

        let distribution = result.unwrap();
        let total: u64 = distribution.iter().map(|(_, amount)| amount).sum();
        assert_eq!(total, 1000);
    }

    #[test]
    fn test_staking_rewards() {
        let distributor =
            RewardDistributor::new("http://localhost:8899".to_string(), 1000).unwrap();
        let result = distributor.calculate_staking_rewards(1_000_000, 3600 * 24, 800_000);
        assert!(result.is_ok());
    }
}
