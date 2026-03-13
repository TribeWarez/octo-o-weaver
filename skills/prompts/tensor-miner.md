# PoT-O Tensor Mining Agent Generator

Using octo-O-weaver and ai3-lib, generate a PoT-O miner agent that:

## Functionality

1. **Generates** tensor proofs locally using ai3-lib
2. **Submits** proofs via ProofOrchestrator to pot-o-mining
3. **Monitors** tensor coherence and entropy metrics
4. **Optimizes** proof paths for higher rewards
5. **Collects** mining rewards via RewardDistributor

## Requirements

- Use `ProofOrchestrator` for proof submission
- Integrate `TensorNetworkManager` for metrics
- Query `ai3_lib` for best tensor paths
- Support ESP32 device detection for optimizations
- Include proof validation with error recovery
- Generate full agent binary

## Constraints

- Proof generation: max 30 seconds per proof
- Memory: must work on <512MB devices
- Network: fallback to local tensor computation if RPC unavailable

## Output Files

1. `src/main.rs` - Complete mining loop
2. `src/tensor_optimizer.rs` - Tensor path optimization logic
3. `src/proof_batcher.rs` - Proof batching (1-10 proofs per submission)
4. `tests/mock_tensor_network.rs` - Tests with mock data
5. `Cargo.toml` - Dependencies

## Key Types

```rust
use octo_o_weaver::prelude::*;
use octo_o_weaver::proof_orchestration::ProofOrchestrator;
use octo_o_weaver::miner_lifecycle::MinerLifecycle;
use octo_o_weaver::tensor_network::TensorNetworkManager;
use octo_o_weaver::reward_distribution::RewardDistributor;
use ai3_lib::{MinerCapabilities, TensorEngine};
```

Generate production-ready, well-documented code following Rust best practices.
