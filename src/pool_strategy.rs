//! Pool Strategy Abstraction Tentacle
//!
//! Provides unified interface for Solo, Proportional, and PPLNS pool strategies.
//! This is a MUST-HAVE tentacle that abstracts pool management and ROI calculations.

use crate::error::{OctoError, OctoResult};
use crate::types::{PoolROI, PoolStrategyConfig};
use solana_sdk::pubkey::Pubkey;

/// Pool strategy manager
#[derive(Debug, Clone)]
pub struct PoolStrategyManager {
    rpc_endpoint: String,
}

impl PoolStrategyManager {
    /// Create a new pool strategy manager
    pub fn new(rpc_endpoint: String) -> OctoResult<Self> {
        if rpc_endpoint.is_empty() {
            return Err(OctoError::InvalidConfig(
                "RPC endpoint cannot be empty".to_string(),
            ));
        }

        Ok(PoolStrategyManager { rpc_endpoint })
    }

    /// Create a solo mining strategy
    pub fn create_solo_strategy(&self, miner_pubkey: Pubkey) -> OctoResult<PoolStrategyConfig> {
        Ok(PoolStrategyConfig::Solo { miner_pubkey })
    }

    /// Create a proportional pool strategy
    pub fn create_proportional_strategy(
        &self,
        pool_id: Pubkey,
        fee_percent: u8,
    ) -> OctoResult<PoolStrategyConfig> {
        if fee_percent > 100 {
            return Err(OctoError::ValidationError(
                "Fee percent cannot exceed 100".to_string(),
            ));
        }

        Ok(PoolStrategyConfig::Proportional {
            pool_id,
            fee_percent,
        })
    }

    /// Create a PPLNS pool strategy
    pub fn create_pplns_strategy(
        &self,
        pool_id: Pubkey,
        fee_percent: u8,
        share_multiplier: u64,
    ) -> OctoResult<PoolStrategyConfig> {
        if fee_percent > 100 {
            return Err(OctoError::ValidationError(
                "Fee percent cannot exceed 100".to_string(),
            ));
        }

        if share_multiplier == 0 {
            return Err(OctoError::ValidationError(
                "Share multiplier must be greater than 0".to_string(),
            ));
        }

        Ok(PoolStrategyConfig::PPLNS {
            pool_id,
            fee_percent,
            share_multiplier,
        })
    }

    /// Calculate ROI for a given strategy
    pub fn calculate_roi(
        &self,
        strategy: &PoolStrategyConfig,
        daily_proofs: u64,
        reward_per_proof: u64,
    ) -> OctoResult<PoolROI> {
        self.validate_inputs(daily_proofs, reward_per_proof)?;

        let (base_daily, fee_percent) = match strategy {
            PoolStrategyConfig::Solo { .. } => (daily_proofs * reward_per_proof, 0u8),
            PoolStrategyConfig::Proportional { fee_percent, .. } => {
                (daily_proofs * reward_per_proof, *fee_percent)
            }
            PoolStrategyConfig::PPLNS {
                fee_percent,
                share_multiplier,
                ..
            } => {
                // PPLNS: rewards based on share history (simplified)
                let adjusted_daily =
                    (daily_proofs as u128).saturating_mul(*share_multiplier as u128) / 100u128;
                (adjusted_daily as u64 * reward_per_proof, *fee_percent)
            }
        };

        // Calculate net after fees
        let daily_return = (base_daily as f64) * (1.0 - (fee_percent as f64 / 100.0));
        let monthly_return = daily_return * 30.0;
        let annual_return = daily_return * 365.0;

        Ok(PoolROI {
            strategy: format!("{:?}", strategy),
            daily_return,
            monthly_return,
            annual_return,
            projected_revenue: base_daily,
        })
    }

    /// Compare multiple strategies
    pub fn compare_strategies(
        &self,
        strategies: Vec<PoolStrategyConfig>,
        daily_proofs: u64,
        reward_per_proof: u64,
    ) -> OctoResult<Vec<PoolROI>> {
        let mut results = Vec::new();
        for strategy in strategies {
            match self.calculate_roi(&strategy, daily_proofs, reward_per_proof) {
                Ok(roi) => results.push(roi),
                Err(e) => {
                    return Err(OctoError::PoolStrategy(format!(
                        "Failed to calculate ROI: {}",
                        e
                    )))
                }
            }
        }
        Ok(results)
    }

    /// Get optimal strategy based on miner metrics
    pub fn get_optimal_strategy(
        &self,
        miner_pubkey: Pubkey,
        hashrate: u64,
        pool_fee_percent: u8,
    ) -> OctoResult<PoolStrategyConfig> {
        // Simple heuristic: High hashrate miners benefit from proportional pools
        if hashrate > 1_000_000 {
            // Create hypothetical pool ID (in real implementation, query actual pools)
            let pool_id = Pubkey::new_unique();
            Ok(PoolStrategyConfig::Proportional {
                pool_id,
                fee_percent: pool_fee_percent,
            })
        } else {
            // Low hashrate miners benefit from solo mining
            Ok(PoolStrategyConfig::Solo { miner_pubkey })
        }
    }

    /// Validate mining inputs
    fn validate_inputs(&self, daily_proofs: u64, reward_per_proof: u64) -> OctoResult<()> {
        if daily_proofs == 0 {
            return Err(OctoError::ValidationError(
                "Daily proofs must be greater than 0".to_string(),
            ));
        }

        if reward_per_proof == 0 {
            return Err(OctoError::ValidationError(
                "Reward per proof must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string());
        assert!(manager.is_ok());
    }

    #[test]
    fn test_create_solo_strategy() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string()).unwrap();
        let pubkey = Pubkey::new_unique();
        let strategy = manager.create_solo_strategy(pubkey);
        assert!(strategy.is_ok());
    }

    #[test]
    fn test_create_proportional_strategy() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string()).unwrap();
        let pool_id = Pubkey::new_unique();
        let strategy = manager.create_proportional_strategy(pool_id, 5);
        assert!(strategy.is_ok());
    }

    #[test]
    fn test_invalid_fee_percent() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string()).unwrap();
        let pool_id = Pubkey::new_unique();
        let strategy = manager.create_proportional_strategy(pool_id, 150);
        assert!(strategy.is_err());
    }

    #[test]
    fn test_roi_calculation() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string()).unwrap();
        let pubkey = Pubkey::new_unique();
        let strategy = manager.create_solo_strategy(pubkey).unwrap();
        let roi = manager.calculate_roi(&strategy, 100, 1000);
        assert!(roi.is_ok());

        let roi = roi.unwrap();
        assert_eq!(roi.daily_return, 100_000.0); // 100 * 1000
    }

    #[test]
    fn test_compare_strategies() {
        let manager = PoolStrategyManager::new("http://localhost:8899".to_string()).unwrap();
        let pubkey = Pubkey::new_unique();
        let pool_id = Pubkey::new_unique();
        let strategies = vec![
            manager.create_solo_strategy(pubkey).unwrap(),
            manager.create_proportional_strategy(pool_id, 5).unwrap(),
        ];
        let comparison = manager.compare_strategies(strategies, 100, 1000);
        assert!(comparison.is_ok());
        assert_eq!(comparison.unwrap().len(), 2);
    }
}
