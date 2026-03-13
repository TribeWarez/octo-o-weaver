# Octo-O-Weaver API Reference

This document provides detailed API documentation for all octo-O-weaver types, functions, and modules.

## Module Index

- [error](#module-error) - Error types
- [types](#module-types) - Shared types
- [proof_orchestration](#module-proof-orchestration) - Proof submission
- [miner_lifecycle](#module-miner-lifecycle) - Miner management
- [tensor_network](#module-tensor-network) - Network metrics
- [reward_distribution](#module-reward-distribution) - Reward calculations
- [pool_strategy](#module-pool-strategy) - Pool strategies
- [cross_chain_bridge](#module-cross-chain-bridge) - Token bridging
- [governance](#module-governance) - DAO operations
- [liquidity](#module-liquidity) - Swap routing

---

## Module: error

```rust
use octo_o_weaver::error::{OctoError, OctoResult};
```

### OctoError

```rust
pub enum OctoError {
    /// Invalid configuration parameter
    InvalidConfig(String),
    
    /// Resource not found
    NotFound(String),
    
    /// Resource already exists
    AlreadyExists(String),
    
    /// Validation failed
    ValidationError(String),
    
    /// Proof orchestration error
    ProofOrchestration(String),
    
    /// Network/RPC error
    NetworkError(String),
}
```

### OctoResult

```rust
pub type OctoResult<T> = Result<T, OctoError>;
```

---

## Module: types

```rust
use octo_o_weaver::types::*;
```

### Miner

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miner {
    pub pubkey: Pubkey,
    pub capabilities: MinerCapabilities,
    pub reputation: u64,
    pub total_proofs: u64,
    pub accepted_proofs: u64,
    pub rejected_proofs: u64,
}
```

### TensorNetworkSnapshot

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorNetworkSnapshot {
    /// Total entropy (fixed-point 1e6)
    pub total_entropy: u64,
    /// Network coherence (fixed-point 1e6, 0-1e6)
    pub coherence: u64,
    /// Number of entanglement pairs
    pub entanglement_pairs: u64,
    /// Network efficiency (fixed-point 1e6)
    pub network_efficiency: u64,
    /// Unix timestamp
    pub timestamp: u64,
}
```

### RewardCalculation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculation {
    /// Base reward in lamports
    pub base_reward: u64,
    /// Tensor multiplier (fixed-point 1e6, e.g., 1150000 = 1.15x)
    pub tensor_multiplier: u64,
    /// Coherence bonus (fixed-point 1e6, e.g., 50000 = 5%)
    pub coherence_bonus: u64,
    /// Total reward in lamports
    pub total_reward: u64,
    /// Currency/token symbol
    pub currency: String,
}
```

### PoolROI

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolROI {
    /// Daily return in lamports
    pub daily_return: f64,
    /// Monthly return in lamports
    pub monthly_return: f64,
    /// Annual return in lamports
    pub annual_return: f64,
}
```

### SwapQuote

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuote {
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    /// Price impact (fixed-point 1e6)
    pub price_impact: u64,
    pub swap_fee: u64,
    pub execution_price: f64,
}
```

### PoolInfo

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub pool_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_supply: u64,
    /// Fee in basis points (e.g., 30 = 0.30%)
    pub fee_tier: u16,
}
```

### ProofStatus

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProofStatus {
    Pending,
    Submitted,
    Validated,
    Rejected,
    Expired,
}
```

### ProposalStatus

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Failed,
    Executed,
}
```

---

## Module: proof_orchestration

```rust
use octo_o_weaver::proof_orchestration::ProofOrchestrator;
```

### ProofOrchestrator

```rust
pub struct ProofOrchestrator {
    proofs: HashMap<String, ProofSubmissionResult>,
    rpc_endpoint: String,
}

impl ProofOrchestrator {
    /// Create a new proof orchestrator
    pub fn new(rpc_endpoint: String) -> OctoResult<Self>
    
    /// Submit a proof to the network
    pub async fn submit_proof(
        &mut self,
        proof: ProofPayload,
    ) -> OctoResult<ProofSubmissionResult>
    
    /// Validate a proof structure
    pub fn validate_proof(&self, proof: &ProofPayload) -> OctoResult<()>
    
    /// Get proof submission status
    pub fn get_proof_status(
        &self,
        proof_id: &str,
    ) -> OctoResult<ProofStatus>
    
    /// Batch submit multiple proofs
    pub async fn submit_batch(
        &mut self,
        proofs: Vec<ProofPayload>,
    ) -> OctoResult<Vec<ProofSubmissionResult>>
    
    /// Verify proof against challenge
    pub fn verify_proof_challenge(
        &self,
        proof: &ProofPayload,
        challenge: &Challenge,
    ) -> OctoResult<bool>
    
    /// Get all submitted proofs
    pub fn get_all_proofs(&self) -> Vec<ProofSubmissionResult>
    
    /// Archive old proofs
    pub fn archive_old_proofs(
        &mut self,
        before_timestamp: u64,
    ) -> OctoResult<u64>
}
```

---

## Module: miner_lifecycle

```rust
use octo_o_weaver::miner_lifecycle::MinerLifecycle;
```

### MinerLifecycle

```rust
pub struct MinerLifecycle {
    miners: HashMap<Pubkey, Miner>,
    rpc_endpoint: String,
}

impl MinerLifecycle {
    /// Create a new miner lifecycle manager
    pub fn new(rpc_endpoint: String) -> OctoResult<Self>
    
    /// Register a new miner
    pub fn register_miner(
        &mut self,
        pubkey: Pubkey,
        capabilities: MinerCapabilities,
    ) -> OctoResult<Miner>
    
    /// Get miner by pubkey
    pub fn get_miner(&self, pubkey: &Pubkey) -> OctoResult<Miner>
    
    /// Update miner capabilities
    pub fn update_capabilities(
        &mut self,
        pubkey: &Pubkey,
        new_capabilities: MinerCapabilities,
    ) -> OctoResult<()>
    
    /// Record proof acceptance
    pub fn record_proof_acceptance(&mut self, pubkey: &Pubkey) -> OctoResult<()>
    
    /// Record proof rejection
    pub fn record_proof_rejection(&mut self, pubkey: &Pubkey) -> OctoResult<()>
    
    /// Calculate reputation score (0-100)
    pub fn calculate_reputation(&self, miner: &Miner) -> u64
    
    /// Get all registered miners
    pub fn get_all_miners(&self) -> Vec<Miner>
    
    /// Get top miners by reputation
    pub fn get_top_miners(&self, count: usize) -> Vec<Miner>
    
    /// Deregister a miner
    pub fn deregister_miner(&mut self, pubkey: &Pubkey) -> OctoResult<()>
}
```

---

## Module: tensor_network

```rust
use octo_o_weaver::tensor_network::TensorNetworkManager;
```

### TensorNetworkManager

```rust
pub struct TensorNetworkManager {
    rpc_endpoint: String,
}

impl TensorNetworkManager {
    /// Create a new tensor network manager
    pub fn new(rpc_endpoint: String) -> OctoResult<Self>
    
    /// Query complete network state
    pub async fn query_network_state(
        &self,
    ) -> OctoResult<TensorNetworkSnapshot>
    
    /// Get entropy for a specific partition
    pub fn get_partition_entropy(&self, partition_id: &str) -> OctoResult<u64>
    
    /// Get mutual information between two partitions
    pub fn get_mutual_information(
        &self,
        partition_a: &str,
        partition_b: &str,
    ) -> OctoResult<u64>
    
    /// Get effective distance between partitions
    pub fn get_effective_distance(
        &self,
        partition_a: &str,
        partition_b: &str,
    ) -> OctoResult<u64>
    
    /// Get coherence probability for a partition
    pub fn get_coherence_probability(
        &self,
        partition_id: &str,
    ) -> OctoResult<u64>
    
    /// Get minimal cut for network partitioning
    pub fn get_minimal_cut(&self, region_id: &str) -> OctoResult<Vec<String>>
    
    /// Get entanglement state between two accounts
    pub fn get_entanglement(
        &self,
        account_a: &Pubkey,
        account_b: &Pubkey,
    ) -> OctoResult<EntanglementState>
    
    /// Calculate network efficiency score
    pub fn calculate_efficiency(
        &self,
        entropy: u64,
        coherence: u64,
        pairs: u64,
    ) -> OctoResult<u64>
}
```

---

## Module: reward_distribution

```rust
use octo_o_weaver::reward_distribution::RewardDistributor;
```

### RewardDistributor

```rust
pub struct RewardDistributor {
    rpc_endpoint: String,
    /// Base reward in lamports per proof
    base_reward_rate: u64,
}

impl RewardDistributor {
    /// Create a new reward distributor
    pub fn new(
        rpc_endpoint: String,
        base_reward_rate: u64,
    ) -> OctoResult<Self>
    
    /// Calculate miner rewards with multipliers
    pub fn calculate_miner_rewards(
        &self,
        miner: &Pubkey,
        accepted_proofs: u64,
        /// Tensor multiplier (fixed-point 1e6)
        tensor_multiplier: u64,
        /// Coherence bonus (fixed-point 1e6)
        coherence_bonus: u64,
    ) -> OctoResult<RewardCalculation>
    
    /// Distribute rewards to pool participants
    pub fn distribute_pool_rewards(
        &self,
        total_reward: u64,
        miners: Vec<Pubkey>,
        contributions: Vec<u64>,
    ) -> OctoResult<Vec<(Pubkey, u64)>>
    
    /// Calculate staking rewards with lock duration
    pub fn calculate_staking_rewards(
        &self,
        stake_amount: u64,
        duration_seconds: u64,
        /// Average coherence (fixed-point 1e6)
        average_coherence: u64,
    ) -> OctoResult<RewardCalculation>
}
```

---

## Module: pool_strategy

```rust
use octo_o_weaver::pool_strategy::PoolStrategyManager;
```

### PoolStrategyManager

```rust
pub struct PoolStrategyManager {
    rpc_endpoint: String,
}

impl PoolStrategyManager {
    /// Create a new pool strategy manager
    pub fn new(rpc_endpoint: String) -> OctoResult<Self>
    
    /// Create a solo mining strategy
    pub fn create_solo_strategy(
        &self,
        miner_pubkey: Pubkey,
    ) -> OctoResult<PoolStrategyConfig>
    
    /// Create a proportional pool strategy
    pub fn create_proportional_strategy(
        &self,
        pool_id: Pubkey,
        fee_percent: u64,
    ) -> OctoResult<PoolStrategyConfig>
    
    /// Create a PPLNS pool strategy
    pub fn create_pplns_strategy(
        &self,
        pool_id: Pubkey,
        share_multiplier: u64,
    ) -> OctoResult<PoolStrategyConfig>
    
    /// Get optimal strategy based on miner hashrate
    pub fn get_optimal_strategy(
        &self,
        miner_pubkey: Pubkey,
        miner_hashrate: u64,
        pool_count: usize,
    ) -> OctoResult<PoolStrategyConfig>
    
    /// Calculate ROI for a strategy
    pub fn calculate_roi(
        &self,
        strategy: &PoolStrategyConfig,
        stake_amount: u64,
        reward_per_proof: u64,
    ) -> OctoResult<PoolROI>
    
    /// Compare multiple strategies
    pub fn compare_strategies(
        &self,
        strategies: Vec<PoolStrategyConfig>,
        stake_amount: u64,
        reward_per_proof: u64,
    ) -> OctoResult<Vec<(PoolStrategyConfig, PoolROI)>>
}
```

### PoolStrategyConfig

```rust
pub enum PoolStrategyConfig {
    Solo {
        miner_pubkey: Pubkey,
    },
    Proportional {
        pool_id: Pubkey,
        fee_percent: u64,
    },
    PPLNS {
        pool_id: Pubkey,
        share_multiplier: u64,
    },
}
```

---

## Module: liquidity

```rust
use octo_o_weaver::liquidity::LiquidityEngine;
```

### LiquidityEngine

```rust
pub struct LiquidityEngine {
    rpc_endpoint: String,
}

impl LiquidityEngine {
    /// Create a new liquidity engine
    pub fn new(rpc_endpoint: String) -> OctoResult<Self>
    
    /// Quote a swap between two tokens
    pub fn quote_swap(
        &self,
        input_token: Pubkey,
        output_token: Pubkey,
        input_amount: u64,
    ) -> OctoResult<SwapQuote>
    
    /// Get pool information
    pub fn get_pool_info(
        &self,
        pool_id: &Pubkey,
    ) -> OctoResult<PoolInfo>
    
    /// Calculate liquidity provider fees
    pub fn calculate_lp_fees(
        &self,
        pool_info: &PoolInfo,
        lp_tokens: u64,
    ) -> OctoResult<u64>
}
```

---

## Prelude

For convenience, import all common types:

```rust
use octo_o_weaver::prelude::*;
```

The prelude includes:
- All tentacle managers
- Common types (Miner, RewardCalculation, etc.)
- Error types (OctoError, OctoResult)
- MinerCapabilities from ai3-lib
- ProofPayload, Challenge from pot_o_mining
