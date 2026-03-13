# TribeWarez Yield Farmer Agent Generator

Using the octo-O-weaver crate (https://crates.io/crates/octo-O-weaver), generate a complete Solana agent that:

## Functionality

1. **Monitors** tribewarez-liquidity pools (PPTC/AUMCOIN pairs)
2. **Queries** current yields via TensorNetworkManager RPC
3. **Calculates** optimal deposit amounts using PoolStrategyManager
4. **Executes** deposits via CPI to tribewarez-staking
5. **Re-balances** hourly based on coherence changes

## Requirements

- Use `octo_o_weaver::pool_strategy_manager` for strategy selection
- Integrate `ai3_lib` tensor metrics for yield optimization
- Include error handling for RPC failures
- Generate full Cargo.toml with dependencies
- Include 3 integration tests
- Output as ready-to-deploy Rust binary

## Constraints

- Max transaction size: 1.2 MB
- Gas budget: 500K lamports per operation
- Min. pool liquidity: 10K PPTC

## Output Files

1. `src/main.rs` - Agent loop + logic
2. `src/lib.rs` - Reusable components  
3. `Cargo.toml` - All dependencies
4. `tests/integration_tests.rs` - Integration tests
5. `README.md` - Setup instructions

## Key Types

```rust
use octo_o_weaver::prelude::*;
use octo_o_weaver::pool_strategy::{PoolStrategyManager, PoolROI};
use octo_o_weaver::tensor_network::TensorNetworkManager;
use octo_o_weaver::reward_distribution::RewardDistributor;
```

## Example Usage

```rust
let pool_manager = PoolStrategyManager::new(rpc_endpoint)?;
let tensor_manager = TensorNetworkManager::new(rpc_endpoint)?;
let rewards = RewardDistributor::new(rpc_endpoint, base_reward_rate)?;
```

Generate production-ready, well-documented code following Rust best practices.
