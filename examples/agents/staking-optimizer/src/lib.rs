//! TribeWarez Staking Optimizer Agent
//!
//! This agent optimizes staking rewards using PoT-O coherence bonuses.
//! It manages lock-ups, rebalances across pools, and auto-compounds rewards.

use octo_O_weaver::{
    error::OctoResult,
    prelude::*,
    reward_distribution::RewardDistributor,
    tensor_network::TensorNetworkManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Lock duration options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LockDuration {
    Short,   // 7 days
    Medium,  // 30 days  
    Long,    // 90 days
}

impl LockDuration {
    pub fn seconds(&self) -> u64 {
        match self {
            LockDuration::Short => 7 * 24 * 3600,
            LockDuration::Medium => 30 * 24 * 3600,
            LockDuration::Long => 90 * 24 * 3600,
        }
    }

    pub fn multiplier(&self) -> u64 {
        match self {
            LockDuration::Short => 1_100_000,   // 1.1x
            LockDuration::Medium => 1_250_000,  // 1.25x
            LockDuration::Long => 1_500_000,    // 1.5x
        }
    }
}

/// Stake position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakePosition {
    pub pool_id: String,
    pub amount: u64,
    pub lock_duration: LockDuration,
    pub start_time: i64,
    pub coherence_at_stake: u64,
}

/// Configuration for staking optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingOptimizerConfig {
    pub rpc_endpoint: String,
    pub total_stake_amount: u64,
    pub min_stake: u64,
    pub rebalance_interval_secs: u64,
    pub auto_compound: bool,
    pub preferred_lock_duration: LockDuration,
    pub wallet_pubkey: String,
}

impl Default for StakingOptimizerConfig {
    fn default() -> Self {
        Self {
            rpc_endpoint: "https://api.mainnet-beta.solana.com".to_string(),
            total_stake_amount: 10_000_000_000, // 10 SOL
            min_stake: 100_000_000,              // 0.1 SOL
            rebalance_interval_secs: 3600,        // 1 hour
            auto_compound: true,
            preferred_lock_duration: LockDuration::Long,
            wallet_pubkey: "".to_string(),
        }
    }
}

/// Staking optimizer state
#[derive(Debug)]
pub struct StakingOptimizerState {
    pub config: StakingOptimizerConfig,
    pub positions: Vec<StakePosition>,
    pub total_staked: u64,
    pub total_rewards: u64,
    pub last_rebalance: i64,
    pub weighted_apr: f64,
}

impl StakingOptimizerState {
    pub fn new(config: StakingOptimizerConfig) -> Self {
        Self {
            config,
            positions: Vec::new(),
            total_staked: 0,
            total_rewards: 0,
            last_rebalance: 0,
            weighted_apr: 0.0,
        }
    }
}

/// Main staking optimizer agent
pub struct StakingOptimizer {
    state: Arc<RwLock<StakingOptimizerState>>,
    tensor_manager: TensorNetworkManager,
    reward_distributor: RewardDistributor,
}

impl StakingOptimizer {
    /// Create a new staking optimizer
    pub async fn new(config: StakingOptimizerConfig) -> OctoResult<Self> {
        info!("Initializing Staking Optimizer Agent");

        let tensor_manager = TensorNetworkManager::new(config.rpc_endpoint.clone())?;
        let reward_distributor = RewardDistributor::new(
            config.rpc_endpoint.clone(),
            1000,
        )?;

        let state = StakingOptimizerState::new(config);

        Ok(Self {
            state: Arc::new(RwLock::new(state)),
            tensor_manager,
            reward_distributor,
        })
    }

    /// Get current network coherence
    pub async fn get_coherence(&self) -> OctoResult<u64> {
        let network_state = self.tensor_manager.query_network_state().await?;
        Ok(network_state.coherence)
    }

    /// Calculate optimal lock duration based on coherence
    pub async fn calculate_optimal_lock(&self) -> OctoResult<LockDuration> {
        let coherence = self.get_coherence().await?;
        let config = self.state.read().await.config.clone();

        // Higher coherence = longer lock for better multipliers
        if coherence > 800_000 {
            Ok(LockDuration::Long)
        } else if coherence > 500_000 {
            match config.preferred_lock_duration {
                LockDuration::Short => LockDuration::Medium,
                _ => config.preferred_lock_duration,
            }
        } else {
            Ok(LockDuration::Short)
        }
    }

    /// Calculate rewards for current positions
    pub async fn calculate_rewards(&self) -> OctoResult<u64> {
        let state = self.state.read().await;
        let coherence = self.get_coherence().await.unwrap_or(500_000);

        let mut total_rewards = 0u64;

        for position in &state.positions {
            let calculation = self.reward_distributor.calculate_staking_rewards(
                position.amount,
                position.lock_duration.seconds(),
                coherence,
            )?;
            total_rewards += calculation.total_reward;
        }

        Ok(total_rewards)
    }

    /// Create a new stake position
    pub async fn create_stake(&self, pool_id: String, amount: u64) -> OctoResult<()> {
        let config = self.state.read().await.config.clone();

        if amount < config.min_stake {
            return Err(OctoError::ValidationError(
                format!("Amount {} below minimum {}", amount, config.min_stake)
            ));
        }

        let lock_duration = self.calculate_optimal_lock().await?;
        let coherence = self.get_coherence().await?;

        let position = StakePosition {
            pool_id,
            amount,
            lock_duration: lock_duration.clone(),
            start_time: chrono::Utc::now().timestamp(),
            coherence_at_stake: coherence,
        };

        {
            let mut state = self.state.write().await;
            state.positions.push(position.clone());
            state.total_staked += amount;
        }

        info!("Created {} stake: {} lamports for {} days", 
            lock_duration.name(), amount, lock_duration.seconds() / 86400);

        Ok(())
    }

    /// Rebalance stakes based on current coherence
    pub async fn rebalance(&self) -> OctoResult<()> {
        info!("Rebalancing stakes");

        let coherence = self.get_coherence().await?;
        let state = self.state.read().await;
        
        // Calculate weighted APR
        let mut weighted_sum = 0.0;
        let mut total = 0u64;

        for position in &state.positions {
            let lock_mult = position.lock_duration.multiplier() as f64 / 1_000_000.0;
            let coherence_mult = coherence as f64 / 1_000_000.0;
            let apr = 10.0 * lock_mult * coherence_mult; // Base 10% APY
            weighted_sum += apr * position.amount as f64;
            total += position.amount;
        }

        let weighted_apr = if total > 0 {
            weighted_sum / total as f64
        } else {
            0.0
        };

        drop(state);

        {
            let mut state = self.state.write().await;
            state.last_rebalance = chrono::Utc::now().timestamp();
            state.weighted_apr = weighted_apr;
        }

        info!("Rebalanced. Weighted APR: {:.2}%", weighted_apr);
        Ok(())
    }

    /// Auto-compound rewards
    pub async fn compound(&self) -> OctoResult<u64> {
        info!("Compounding rewards");

        let rewards = self.calculate_rewards().await?;

        if rewards > 0 {
            let config = self.state.read().await.config.clone();
            
            // Add rewards to stake (in production, submit actual transaction)
            {
                let mut state = self.state.write().await;
                state.total_rewards += rewards;
                state.total_staked += rewards;
            }

            info!("Compounded {} lamports in rewards", rewards);
        }

        Ok(rewards)
    }

    /// Check if rebalancing is needed
    pub async fn needs_rebalance(&self) -> bool {
        let state = self.state.read().await;
        let now = chrono::Utc::now().timestamp();
        (now - state.last_rebalance) >= state.config.rebalance_interval_secs as i64
    }

    /// Get current state
    pub async fn get_state(&self) -> StakingOptimizerState {
        self.state.read().await.clone()
    }

    /// Run the main staking optimizer loop
    pub async fn run(&self) -> OctoResult<()> {
        info!("Starting Staking Optimizer Agent main loop");

        // Initial stake setup
        {
            let config = self.state.read().await.config.clone();
            if config.total_stake_amount > 0 {
                self.create_stake(
                    "tribewarez-staking-pool-001".to_string(),
                    config.total_stake_amount,
                ).await?;
            }
        }

        loop {
            // Display current state
            let state = self.get_state().await;
            let coherence = self.get_coherence().await.unwrap_or(0);

            info!(
                "Staked: {} | Rewards: {} | APR: {:.2}% | Coherence: {:.4}",
                state.total_staked,
                state.total_rewards,
                state.weighted_apr,
                coherence as f64 / 1_000_000.0
            );

            // Check if we need to rebalance
            if self.needs_rebalance().await {
                self.rebalance().await?;
            }

            // Auto-compound if enabled
            let config = self.state.read().await.config.clone();
            if config.auto_compound {
                self.compound().await?;
            }

            // Sleep before next iteration
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // 5 minutes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_staking_optimizer_creation() {
        let config = StakingOptimizerConfig::default();
        let optimizer = StakingOptimizer::new(config).await;
        assert!(optimizer.is_ok());
    }

    #[tokio::test]
    async fn test_lock_duration_calculations() {
        assert_eq!(LockDuration::Short.seconds(), 7 * 24 * 3600);
        assert_eq!(LockDuration::Medium.seconds(), 30 * 24 * 3600);
        assert_eq!(LockDuration::Long.seconds(), 90 * 24 * 3600);
    }
}
