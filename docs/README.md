# 🐙 Octo-O-Weaver Documentation

Welcome to the Octo-O-Weaver documentation! This guide will help you get started with the PaaS orchestration layer for TribeWarez DeFi + PoT-O autonomous agents.

## What is Octo-O-Weaver?

Octo-O-Weaver is a **Platform-as-a-Service (PaaS)** abstraction layer that unifies the TribeWarez v0.4.0 ecosystem into a single interface for building autonomous agents on Solana.

### Key Features

- **8 Tentacle Modules** - Production-ready abstractions for all TribeWarez programs
- **PoT-O Integration** - Native support for tensor proof mining and coherence-based rewards
- **OpenCode Ready** - AI-powered code generation for agent strategies
- **OpenClaw Compatible** - Self-hosted 24/7 autonomous agents
- **Off-chain Only** - No on-chain contract deployments required

## Quick Start

### Installation

```toml
[dependencies]
octo-O-weaver = "0.1.0"
tokio = { version = "1", features = ["full"] }
solana-client = "1.18"
```

```bash
cargo build
cargo test
```

### Your First Agent

```rust
use octo_o_weaver::prelude::*;
use solana_sdk::pubkey::Pubkey;
use ai3_lib::MinerCapabilities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize managers
    let rpc_endpoint = "https://api.mainnet-beta.solana.com";
    
    let mut miner_manager = MinerLifecycle::new(rpc_endpoint.to_string())?;
    let tensor_manager = TensorNetworkManager::new(rpc_endpoint.to_string())?;
    let rewards = RewardDistributor::new(rpc_endpoint.to_string(), 1000)?;
    
    // Register a miner
    let pubkey = Pubkey::new_unique();
    let caps = MinerCapabilities {
        supported_operations: vec!["matrix_multiply".into()],
        max_tensor_size: 64 * 64 * 4,
        is_esp_device: false,
        max_computation_time: 300,
    };
    
    let miner = miner_manager.register_miner(pubkey, caps)?;
    println!("Registered miner: {}", miner.pubkey);
    
    // Query network state
    let network_state = tensor_manager.query_network_state().await?;
    println!("Network coherence: {}", network_state.coherence);
    
    // Calculate rewards
    let calculation = rewards.calculate_miner_rewards(
        &pubkey,
        10,         // 10 accepted proofs
        1_150_000,  // 1.15x tensor multiplier
        50_000,     // 0.05 coherence bonus
    )?;
    
    println!("Total reward: {} lamports", calculation.total_reward);
    
    Ok(())
}
```

## The 8 Tentacles

### MUST-HAVE (Production-Ready)

1. **ProofOrchestrator** - Submit & validate PoT-O tensor proofs
2. **MinerLifecycle** - Register miners, track capabilities, manage reputation
3. **TensorNetworkManager** - Query entropy, coherence, entanglement metrics
4. **RewardDistributor** - Calculate mining rewards with multipliers
5. **PoolStrategyManager** - Solo/Proportional/PPLNS pool strategies

### NICE-TO-HAVE (Extensible)

6. **CrossChainBridge** - Token wrapping via tribewarez-bridge
7. **GovernanceManager** - DAO proposals & voting
8. **LiquidityEngine** - Swap routing, multi-hop arbitrage

## Integration Options

### Option 1: OpenCode (AI Code Generation)

Generate complete agents by pasting prompt templates into OpenCode:

- **Yield Farmer** - Liquidity provision & auto-compounding
- **Tensor Miner** - PoT-O proof generation & submission
- **Liquidity Arbitrage** - Multi-hop swap optimization
- **Staking Optimizer** - Coherence-based reward maximization

See `skills/prompts/` for all templates.

### Option 2: OpenClaw (Self-Hosted Agents)

Deploy 24/7 autonomous agents using OpenClaw:

```bash
# Install OpenClaw
git clone https://github.com/openclaw/openclaw
cd openclaw
docker-compose up -d

# Add TribeWarez skill
cp octo-O-weaver/openclaw-skills/tribewarez-tensor-miner.yaml ./skills/
```

See `openclaw-skills/` for YAML configurations.

### Option 3: Direct Library Usage

Build custom agents using octo-O-Weaver as a Rust library:

```rust
use octo_o_weaver::{
    proof_orchestration::ProofOrchestrator,
    miner_lifecycle::MinerLifecycle,
    tensor_network::TensorNetworkManager,
    reward_distribution::RewardDistributor,
    pool_strategy::PoolStrategyManager,
    prelude::*,
};
```

## Examples

Run the included examples:

```bash
# Mining workflow
cargo run --example mining_workflow

# Staking strategy
cargo run --example staking_strategy

# Swap quoting
cargo run --example swap_quoting
```

## API Reference

For detailed API documentation, see:

- [ProofOrchestrator](src/proof_orchestration.rs.html)
- [MinerLifecycle](src/miner_lifecycle.rs.html)
- [TensorNetworkManager](src/tensor_network.rs.html)
- [RewardDistributor](src/reward_distribution.rs.html)
- [PoolStrategyManager](src/pool_strategy.rs.html)

## System Requirements

- **Rust** 1.70+ (MSRV)
- **Solana CLI** 1.18+
- **RAM** 2GB minimum for compilation

## Support

- **GitHub**: https://github.com/TribeWarez/octo-o-weaver
- **Documentation**: https://docs.rs/octo-O-weaver
- **Discord**: Join the TribeWarez community

## License

MIT or Apache-2.0
