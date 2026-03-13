//! TribeWarez Yield Farmer Agent Library
//!
//! This module provides the core yield farming logic for the TribeWarez DeFi ecosystem.
//! It monitors liquidity pools, optimizes allocations, and auto-compounds rewards.

use octo_O_weaver::{
    error::OctoResult,
    liquidity::LiquidityEngine,
    pool_strategy::{PoolROI, PoolStrategyConfig, PoolStrategyManager},
    prelude::*,
    reward_distribution::RewardDistributor,
    tensor_network::TensorNetworkManager,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Configuration for the yield farmer agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldFarmerConfig {
    /// RPC endpoint for Solana
    pub rpc_endpoint: String,
    /// Minimum liquidity to consider a pool (in lamports)
    pub min_pool_liquidity: u64,
    /// Minimum APY to consider (as percentage * 100, e.g., 1500 = 15%)
    pub min_apy: u64,
    /// Rebalance interval in seconds
    pub rebalance_interval_secs: u64,
    /// Compounding interval in seconds
    pub compound_interval_secs: u64,
    /// Maximum slippage tolerance (as percentage * 100, e.g., 100 = 1%)
    pub max_slippage_tolerance: u64,
    /// Wallet public key (base58)
    pub wallet_pubkey: String,
}

impl Default for YieldFarmerConfig {
    fn default() -> Self {
        Self {
            rpc_endpoint: "https://api.mainnet-beta.solana.com".to_string(),
            min_pool_liquidity: 10_000_000, // 10M lamports = 0.01 SOL
            min_apy: 500,                   // 5% minimum APY
            rebalance_interval_secs: 3600,   // 1 hour
            compound_interval_secs: 86400,   // 24 hours
            max_slippage_tolerance: 100,    // 1%
            wallet_pubkey: "".to_string(),
        }
    }
}

/// Pool information with yield metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolWithYield {
    pub pool_id: String,
    pub token_a: String,
    pub token_b: String,
    pub liquidity: u64,
    pub apy: f64,
    pub daily_volume: u64,
    pub fee_tier: u16,
}

/// State of the yield farmer
#[derive(Debug)]
pub struct YieldFarmerState {
    pub config: YieldFarmerConfig,
    pub pools: Vec<PoolWithYield>,
    pub current_allocation: Vec<(String, u64)>, // pool_id -> amount
    pub total_value_locked: u64,
    pub total_rewards_earned: u64,
    pub last_rebalance: i64,
    pub last_compound: i64,
}

impl YieldFarmerState {
    pub fn new(config: YieldFarmerConfig) -> Self {
        Self {
            config,
            pools: Vec::new(),
            current_allocation: Vec::new(),
            total_value_locked: 0,
            total_rewards_earned: 0,
            last_rebalance: 0,
            last_compound: 0,
        }
    }
}

/// Main yield farmer agent
pub struct YieldFarmer {
    state: Arc<RwLock<YieldFarmerState>>,
    pool_manager: PoolStrategyManager,
    tensor_manager: TensorNetworkManager,
    liquidity_engine: LiquidityEngine,
    reward_distributor: RewardDistributor,
}

impl YieldFarmer {
    /// Create a new yield farmer agent
    pub async fn new(config: YieldFarmerConfig) -> OctoResult<Self> {
        info!("Initializing Yield Farmer Agent");
        
        let pool_manager = PoolStrategyManager::new(config.rpc_endpoint.clone())?;
        let tensor_manager = TensorNetworkManager::new(config.rpc_endpoint.clone())?;
        let liquidity_engine = LiquidityEngine::new(config.rpc_endpoint.clone())?;
        let reward_distributor = RewardDistributor::new(
            config.rpc_endpoint.clone(),
            1000, // Base reward rate
        )?;

        let state = YieldFarmerState::new(config);

        Ok(Self {
            state: Arc::new(RwLock::new(state)),
            pool_manager,
            tensor_manager,
            liquidity_engine,
            reward_distributor,
        })
    }

    /// Discover available pools with yields
    pub async fn discover_pools(&self) -> OctoResult<Vec<PoolWithYield>> {
        info!("Discovering liquidity pools");
        
        // Get network state for coherence-based yield adjustments
        let network_state = self.tensor_manager.query_network_state().await?;
        let coherence_multiplier = network_state.coherence as f64 / 1_000_000.0;
        
        debug!("Network coherence: {:.4}", coherence_multiplier);

        // In production, this would query actual pool data from Solana
        // For now, return mock pool data with coherence adjustments
        let pools = vec![
            PoolWithYield {
                pool_id: "PPTC-AUMCOIN-001".to_string(),
                token_a: "PPTC".to_string(),
                token_b: "AUMCOIN".to_string(),
                liquidity: 50_000_000_000,
                apy: 12.5 * coherence_multiplier,
                daily_volume: 5_000_000_000,
                fee_tier: 30,
            },
            PoolWithYield {
                pool_id: "PPTC-SOL-001".to_string(),
                token_a: "PPTC".to_string(),
                token_b: "SOL".to_string(),
                liquidity: 100_000_000_000,
                apy: 8.3 * coherence_multiplier,
                daily_volume: 15_000_000_000,
                fee_tier: 25,
            },
            PoolWithYield {
                pool_id: "PPTC-USDC-001".to_string(),
                token_a: "PPTC".to_string(),
                token_b: "USDC".to_string(),
                liquidity: 25_000_000_000,
                apy: 6.2 * coherence_multiplier,
                daily_volume: 2_000_000_000,
                fee_tier: 20,
            },
        ];

        let config = self.state.read().await.config.clone();
        let filtered: Vec<PoolWithYield> = pools
            .into_iter()
            .filter(|p| {
                p.liquidity >= config.min_pool_liquidity && 
                p.apy >= config.min_apy as f64 / 100.0
            })
            .collect();

        info!("Found {} viable pools", filtered.len());
        
        // Update state
        {
            let mut state = self.state.write().await;
            state.pools = filtered.clone();
        }

        Ok(filtered)
    }

    /// Calculate optimal allocation across pools
    pub async fn calculate_optimal_allocation(
        &self,
        total_capital: u64,
    ) -> OctoResult<Vec<(String, u64)>> {
        info!("Calculating optimal allocation for {} lamports", total_capital);
        
        let pools = self.state.read().await.pools.clone();
        
        if pools.is_empty() {
            warn!("No pools available for allocation");
            return Ok(Vec::new());
        }

        // Simple allocation strategy: weighted by APY
        let total_apy: f64 = pools.iter().map(|p| p.apy).sum();
        
        let allocation: Vec<(String, u64)> = pools
            .iter()
            .map(|pool| {
                let weight = pool.apy / total_apy;
                let amount = (total_capital as f64 * weight) as u64;
                (pool.pool_id.clone(), amount)
            })
            .collect();

        debug!("Optimal allocation: {:?}", allocation);
        
        // Update state
        {
            let mut state = self.state.write().await;
            state.current_allocation = allocation.clone();
            state.total_value_locked = total_capital;
        }

        Ok(allocation)
    }

    /// Execute allocation (in production, this would submit actual transactions)
    pub async fn execute_allocation(
        &self,
        allocation: Vec<(String, u64)>,
    ) -> OctoResult<()> {
        info!("Executing allocation across {} pools", allocation.len());
        
        for (pool_id, amount) in &allocation {
            if *amount > 0 {
                info!("  Adding {} lamports to pool {}", amount, pool_id);
                // In production: submit add_liquidity transaction
            }
        }

        Ok(())
    }

    /// Compound rewards (claim and re-stake)
    pub async fn compound_rewards(&self) -> OctoResult<u64> {
        info!("Compounding rewards");
        
        let state = self.state.read().await;
        let total_vl = state.total_value_locked;
        drop(state);

        // Calculate rewards based on current allocation
        let rewards = self.reward_distributor.calculate_miner_rewards(
            &solana_sdk::pubkey::Pubkey::new_unique(),
            10,         // Simulated proofs
            1_100_000, // 1.1x tensor multiplier
            50_000,    // 5% coherence bonus
        )?;

        let total_rewards = rewards.total_reward;
        
        // Update state
        {
            let mut state = self.state.write().await;
            state.total_rewards_earned += total_rewards;
            state.last_compound = chrono::Utc::now().timestamp();
            state.total_value_locked += total_rewards;
        }

        info!("Compounded {} lamports in rewards", total_rewards);
        Ok(total_rewards)
    }

    /// Check if rebalancing is needed
    pub async fn needs_rebalance(&self) -> bool {
        let state = self.state.read().await;
        let now = chrono::Utc::now().timestamp();
        let interval = state.config.rebalance_interval_secs as i64;
        
        (now - state.last_rebalance) >= interval
    }

    /// Check if compounding is needed
    pub async fn needs_compound(&self) -> bool {
        let state = self.state.read().await;
        let now = chrono::Utc::now().timestamp();
        let interval = state.config.compound_interval_secs as i64;
        
        (now - state.last_compound) >= interval
    }

    /// Get current state
    pub async fn get_state(&self) -> YieldFarmerState {
        self.state.read().await.clone()
    }

    /// Run the main agent loop
    pub async fn run(&self) -> OctoResult<()> {
        info!("Starting Yield Farmer Agent main loop");
        
        // Initial pool discovery
        self.discover_pools().await?;

        loop {
            // Check if we need to rebalance
            if self.needs_rebalance().await {
                info!("Rebalance needed");
                
                let state = self.get_state().await;
                let allocation = self.calculate_optimal_allocation(state.total_value_locked).await?;
                self.execute_allocation(allocation).await?;
                
                let mut state = self.state.write().await;
                state.last_rebalance = chrono::Utc::now().timestamp();
            }

            // Check if we need to compound
            if self.needs_compound().await {
                info!("Compounding rewards");
                self.compound_rewards().await?;
            }

            // Get updated state
            let state = self.get_state().await;
            info!(
                "TVL: {} | Rewards: {} | Pools: {}",
                state.total_value_locked,
                state.total_rewards_earned,
                state.pools.len()
            );

            // Sleep before next iteration (check every 5 minutes)
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_yield_farmer_creation() {
        let config = YieldFarmerConfig::default();
        let farmer = YieldFarmer::new(config).await;
        assert!(farmer.is_ok());
    }

    #[tokio::test]
    async fn test_pool_discovery() {
        let config = YieldFarmerConfig::default();
        let farmer = YieldFarmer::new(config).await.unwrap();
        let pools = farmer.discover_pools().await;
        assert!(pools.is_ok());
    }

    #[tokio::test]
    async fn test_allocation_calculation() {
        let config = YieldFarmerConfig::default();
        let farmer = YieldFarmer::new(config).await.unwrap();
        
        // Need to discover pools first
        farmer.discover_pools().await.unwrap();
        
        let allocation = farmer.calculate_optimal_allocation(10_000_000).await;
        assert!(allocation.is_ok());
    }
}
