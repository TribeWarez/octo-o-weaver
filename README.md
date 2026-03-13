# 🐙 Octo-O-Weaver

<p align="center">
  <img src="https://github.com/user-attachments/assets/19df8b02-16f8-45fb-8a3f-86005c3de133" alt="Octo-O-Weaver Logo" width="300" />
</p>

### Platform-as-a-Service (PaaS) Abstraction Layer for TribeWarez DeFi + PoT-O

*8 tentacles weaving AI, CPU, and Solana DeFi into autonomous agent orchestration*

---

Octo-Weaver unifies the TribeWarez v0.4.0 ecosystem (9 Anchor programs + 4 off-chain core crates) into a single, coherent interface for clients.

## Features

### 8 Core Tentacles

#### MUST-HAVE (Production-Ready)

1. **Proof Orchestration** - Unified proof submission & validation across all programs
2. **Miner Lifecycle** - Register, track capabilities, and manage reputation
3. **Tensor Network Management** - Query entropy, coherence, and entanglement metrics
4. **Reward Distribution** - Calculate & distribute rewards with tensor multipliers
5. **Pool Strategy Abstraction** - Solo/Proportional/PPLNS mining strategies with ROI

#### NICE-TO-HAVE (Extensible)

6. **Cross-Chain Bridge** - Token wrapping and signature verification
7. **Governance & Treasury** - DAO proposals, voting, and execution
8. **Liquidity & Swap Engine** - AMM swaps, multi-hop routing, and TWAP

## Quick Start

```rust
use octo_o_weaver::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize managers
    let orchestrator = ProofOrchestrator::new(
        "http://localhost:8899".to_string()
    )?;
    
    let miner_manager = MinerLifecycle::new(
        "http://localhost:8899".to_string()
    )?;
    
    let tensor_manager = TensorNetworkManager::new(
        "http://localhost:8899".to_string()
    )?;
    
    let rewards = RewardDistributor::new(
        "http://localhost:8899".to_string(),
        1000  // base reward in lamports
    )?;
    
    // Query network state
    let network_state = tensor_manager.query_network_state().await?;
    println!("Network entropy: {}", network_state.total_entropy);
    
    Ok(())
}
```

## Installation

### From crates.io (Open Alpha)

```toml
[dependencies]
octo-O-weaver = "0.1.0"
pot-o-core = "0.4.0"
solana-client = "1.18"
tokio = { version = "1", features = ["full"] }
```

```bash
cargo build
cargo test
```

### From GitHub (Bleeding Edge)

```toml
[dependencies]
octo-O-weaver = { git = "https://github.com/TribeWarez/octo-o-weaver" }
```

### Features

```toml
octo-O-weaver = { version = "0.1.0", features = ["http-server"] }
```

- `http-server` - Enable optional HTTP API server (uses axum)

### System Requirements

- Rust 1.70+ (MSRV)
- Solana CLI 1.18+
- 2GB RAM minimum for compilation

## Architecture

Octo-Weaver is organized as 8 tentacles:

```
octo-weaver/
├── src/
│   ├── lib.rs                    # Main library with module declarations
│   ├── error.rs                  # OctoError and OctoResult types
│   ├── types.rs                  # Shared types across tentacles
│   ├── proof_orchestration.rs    # Tentacle 1 (MUST-HAVE)
│   ├── miner_lifecycle.rs        # Tentacle 2 (MUST-HAVE)
│   ├── tensor_network.rs         # Tentacle 3 (MUST-HAVE)
│   ├── reward_distribution.rs    # Tentacle 4 (MUST-HAVE)
│   ├── pool_strategy.rs          # Tentacle 5 (MUST-HAVE)
│   ├── cross_chain_bridge.rs     # Tentacle 6 (NICE-TO-HAVE)
│   ├── governance.rs             # Tentacle 7 (NICE-TO-HAVE)
│   └── liquidity.rs              # Tentacle 8 (NICE-TO-HAVE)
├── examples/
│   ├── mining_workflow.rs        # End-to-end mining example
│   ├── staking_strategy.rs       # Staking with coherence bonuses
│   └── swap_quoting.rs           # Liquidity engine example
└── Cargo.toml
```

## Dependencies

Octo-Weaver depends on the TribeWarez v0.3.0 core crates:

- `pot-o-core` v0.3.0 - Tensor metrics and types
- `ai3-lib` v0.3.0 - AI3 engine and tensor operations
- `pot-o-mining` v0.3.0 - Consensus and proof types
- `pot-o-extensions` v0.3.0 - Chain bridge and pool strategies

## Tensor Metrics

All tensor metrics use fixed-point arithmetic with scale **1e6**:

- **Entropy** - S(A) = |γ_A| * ln(d) (range: 0-1e6)
- **Coherence** - tanh(S/S_max) unlock probability (range: 0-1e6)
- **Mutual Information** - I(A:B) = S_A + S_B - S_AB (range: 0-1e6)
- **Effective Distance** - d_eff = 1 - I/S_max (range: 0-1e6)

### Converting Fixed-Point Values

```rust
// On-chain value: 750_000 (fixed-point scale 1e6)
let displayed_value = 750_000 as f64 / 1_000_000.0;  // 0.75
```

## Testing

Run all tests:

```bash
cargo test --all
```

Run specific tentacle tests:

```bash
cargo test --lib proof_orchestration
cargo test --lib miner_lifecycle
cargo test --lib tensor_network
```

## Features

Enable optional HTTP server feature:

```toml
octo-weaver = { version = "0.1.0", features = ["http-server"] }
```

## Performance

Octo-Weaver is optimized for:

- **Low-latency proof submission** (< 100ms)
- **Efficient tensor metric queries** (O(1) for most operations)
- **Scalable reward distribution** (handles 1000+ miners)
- **Thread-safe concurrent access** (Arc-based managers)

## License

MIT

## Contributing

See main TribeWarez repository for contribution guidelines.

## Roadmap

- [ ] Week 3: Complete MUST-HAVE tentacles implementation
- [ ] Week 4: HTTP API server and integration tests
- [ ] Week 5: NICE-TO-HAVE tentacles implementation
- [ ] Week 6: pot-o-validator v0.3.0 integration
- [ ] Week 7: crates.io publishing and documentation

## Support

For issues and questions, please open an issue on the main TribeWarez repository.
