# OpenCode Integration Guide

This guide explains how to use OpenCode (opencode.ai) to generate TribeWarez agents using the octo-O-weaver crate.

## What is OpenCode?

OpenCode is an AI-powered coding agent that can generate complete Rust code from natural language prompts. By using the octo-O-weaver prompt templates, you can generate production-ready DeFi agents in minutes.

## Getting Started

### Step 1: Choose a Prompt Template

Select one of the pre-built templates in `skills/prompts/`:

| Template | Use Case |
|----------|----------|
| `yield-farmer.md` | Liquidity provision & auto-compounding |
| `tensor-miner.md` | PoT-O tensor proof generation & mining |
| `liquidity-arbitrage.md` | Multi-hop swap arbitrage detection |
| `staking-optimizer.md` | Staking with coherence-based rewards |
| `custom-strategy.md` | Build your own custom strategy |

### Step 2: Paste into OpenCode

1. Go to [opencode.ai](https://opencode.ai)
2. Start a new conversation
3. Copy the prompt template content
4. Paste it into OpenCode
5. Wait for code generation

### Step 3: Review & Customize

The generated code will include:
- Complete `main.rs` with agent logic
- `Cargo.toml` with all dependencies
- Integration tests
- README with setup instructions

## Prompt Templates

### Yield Farmer Agent

```markdown
# TribeWarez Yield Farmer Agent Generator

Using the octo-O-weaver crate (https://crates.io/crates/octo-O-weaver),
generate a complete Solana agent that:

1. **Monitors** tribewarez-liquidity pools (PPTC/AUMCOIN pairs)
2. **Queries** current yields via TensorNetworkManager RPC
3. **Calculates** optimal deposit amounts using PoolStrategyManager
4. **Executes** deposits via CPI to tribewarez-staking
5. **Re-balances** hourly based on coherence changes

Requirements:
- Use octo_o_weaver::pool_strategy_manager for strategy selection
- Integrate ai3_lib tensor metrics for yield optimization
- Include error handling for RPC failures
- Generate full Cargo.toml with dependencies
- Include 3 integration tests
- Output as ready-to-deploy Rust binary
...
```

### Tensor Miner Agent

```markdown
# PoT-O Tensor Mining Agent Generator

Using octo-O-weaver and ai3-lib, generate a PoT-O miner agent that:

1. **Generates** tensor proofs locally using ai3-lib
2. **Submits** proofs via ProofOrchestrator to pot-o-mining
3. **Monitors** tensor coherence and entropy metrics
4. **Optimizes** proof paths for higher rewards
5. **Collects** mining rewards via RewardDistributor
...
```

### Liquidity Arbitrage Bot

```markdown
# TribeWarez Liquidity Arbitrage Bot Generator

Using octo-O-weaver, generate a bot that:

1. **Monitors** multi-hop swap prices via tribewarez-router
2. **Identifies** arbitrage opportunities across pools
3. **Executes** multi-leg swaps via LiquidityEngine
4. **Tracks** profit/loss in PPTC
...
```

### Staking Optimizer

```markdown
# TribeWarez Staking Reward Maximizer

Using octo-O-weaver, generate an agent that:

1. **Monitors** staking pools and reward rates
2. **Calculates** optimal stake amounts based on PoT-O coherence
3. **Manages** lock-ups via tribewarez-vault
4. **Auto-compounds** rewards daily
5. **Rebalances** across pools for max APY
...
```

## Customizing Generated Code

### Adding Custom Logic

After generation, you can add custom logic:

```rust
// In your main.rs
async fn custom_strategy(
    tensor_manager: &TensorNetworkManager,
    rewards: &RewardDistributor,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = tensor_manager.query_network_state().await?;
    
    // Add your custom threshold logic
    if state.coherence > 800_000 {  // > 0.8 coherence
        // High coherence: maximize proof submission
        submit_high_priority_proofs().await?;
    } else {
        // Low coherence: wait for better conditions
        wait_for_better_conditions().await?;
    }
    
    Ok(())
}
```

### Adding New Triggers

```rust
// Custom trigger based on tensor metrics
async fn coherence_trigger(
    threshold: u64,
    action: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let manager = TensorNetworkManager::new(rpc)?;
    let state = manager.query_network_state().await?;
    
    if state.coherence > threshold {
        action()?;
    }
    
    Ok(())
}
```

## Best Practices

### 1. Start with Testnet

```rust
let rpc_endpoint = "https://api.devnet.solana.com";
```

### 2. Use Burner Wallets

```yaml
# In OpenClaw config
wallet_profile: miner-burner-limited
```

### 3. Implement Error Handling

```rust
match result {
    Ok(calculation) => {
        println!("Reward: {}", calculation.total_reward);
    }
    Err(OctoError::NetworkError(e)) => {
        // Retry with exponential backoff
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### 4. Monitor Metrics

```rust
// Track proof success rate
let success_rate = (accepted_proofs as f64 / total_proofs as f64) * 100.0;
println!("Success rate: {:.2}%", success_rate);
```

## Troubleshooting

### "Module not found"

Make sure your `Cargo.toml` includes:

```toml
[dependencies]
octo-O-weaver = "0.1.0"
```

### "RPC timeout"

Increase timeout or use fallback endpoints:

```rust
let client = RpcClient::new_with_timeout(
    "https://api.mainnet-beta.solana.com",
    Duration::from_secs(60),
);
```

### "Insufficient funds"

Use a wallet with more SOL, or reduce operation frequency.

## Examples Generated

### Yield Farmer Output Structure

```
yield-farmer/
├── Cargo.toml
├── src/
│   ├── main.rs       # Agent loop
│   ├── strategy.rs   # Pool selection logic
│   ├── executor.rs   # Deposit/withdraw
│   └── metrics.rs    # Tracking
├── tests/
│   └── integration.rs
└── README.md
```

### Tensor Miner Output Structure

```
tensor-miner/
├── Cargo.toml
├── src/
│   ├── main.rs       # Mining loop
│   ├── generator.rs  # ai3-lib tensor proof generation
│   ├── submitter.rs  # ProofOrchestrator integration
│   └── optimizer.rs  # Path optimization
├── tests/
│   └── mock_network.rs
└── README.md
```

## Advanced Usage

### Multi-Wallet Strategies

```rust
async fn multi_wallet_strategy(
    wallets: Vec<Wallet>,
    pool_manager: &PoolStrategyManager,
) -> Result<()> {
    for wallet in wallets {
        let allocation = pool_manager.get_optimal_strategy(
            wallet.pubkey(),
            wallet.balance(),
            5,
        )?;
        
        execute_allocation(&wallet, &allocation).await?;
    }
    
    Ok(())
}
```

### Custom Pool Strategies

```rust
pub enum CustomStrategy {
    CoherenceWeighted,
    VolatilityAdjusted,
    TimeBased,
}

impl PoolStrategyManager {
    pub fn with_custom_strategy(
        &self,
        strategy: CustomStrategy,
    ) -> PoolStrategyConfig {
        match strategy {
            CustomStrategy::CoherenceWeighted => {
                // Weight by tensor coherence
            }
            // ...
        }
    }
}
```

## Next Steps

1. **Try a Template** - Start with yield-farmer.md
2. **Run Locally** - Test on devnet first
3. **Deploy to OpenClaw** - Use the YAML skill definitions
4. **Monitor** - Track metrics and adjust parameters

## Support

- GitHub Issues: https://github.com/TribeWarez/octo-o-weaver/issues
- OpenCode Discord: https://discord.gg/opencode
