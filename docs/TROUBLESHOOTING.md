# Octo-O-Weaver Troubleshooting Guide

This guide covers common issues and their solutions when using octo-O-weaver.

## Compilation Errors

### "cannot find crate `octo_O_weaver`"

**Problem**: Crate name not recognized in imports.

**Solution**: Use the correct crate name:

```rust
// Use this (snake_case)
use octo_o_weaver::prelude::*;

// Not this (kebab-case won't work in Rust)
// use octo-O-weaver::prelude::*; // WRONG
```

### "module not found: proof_orchestration"

**Problem**: Module not exported.

**Solution**: Check that you're importing from the correct path:

```rust
use octo_o_weaver::proof_orchestration::ProofOrchestrator;
// Or use prelude
use octo_o_weaver::prelude::*;
```

### "struct has no field X"

**Problem**: Struct field name mismatch.

**Solution**: Use the correct field names from the actual types:

```rust
// MinerCapabilities has these fields:
MinerCapabilities {
    supported_operations: Vec<String>,  // NOT "algorithms"
    max_tensor_size: usize,             // NOT "max_memory"
    is_esp_device: bool,                // NOT "device_type"
    max_computation_time: u64,           // NOT "compute_units"
}
```

### "missing argument: rpc_endpoint"

**Problem**: Constructor requires RPC endpoint.

**Solution**: Pass the RPC URL:

```rust
let manager = MinerLifecycle::new(
    "https://api.mainnet-beta.solana.com".to_string()
)?;
```

## Runtime Errors

### "RPC endpoint cannot be empty"

**Problem**: Empty string passed as RPC endpoint.

**Solution**: Use a valid Solana RPC URL:

```rust
// Valid endpoints
let rpc = "https://api.mainnet-beta.solana.com";
let rpc = "https://api.devnet.solana.com";
let rpc = "https://api.rpcpool.com";
```

### "Miner not found"

**Problem**: Trying to access unregistered miner.

**Solution**: Register the miner first:

```rust
let miner = miner_manager.register_miner(pubkey, capabilities)?;
```

Or check if miner exists:

```rust
if let Ok(miner) = miner_manager.get_miner(&pubkey) {
    // Miner exists
}
```

### "Proof not found"

**Problem**: Proof ID doesn't exist in orchestrator.

**Solution**: Submit proof first:

```rust
let result = orchestrator.submit_proof(proof).await?;
let proof_id = result.proof_id;
```

## Network Errors

### "connection timeout"

**Problem**: RPC endpoint not responding.

**Solution**: 
1. Check endpoint URL is correct
2. Try fallback endpoint
3. Increase timeout

```rust
use std::time::Duration;
use solana_client::rpc_client::RpcClient;

let client = RpcClient::new_with_timeout(
    "https://api.mainnet-beta.solana.com",
    Duration::from_secs(60),  // Increase from default 30s
);
```

### "rate limited"

**Problem**: Too many requests to RPC.

**Solution**:
1. Add delay between requests
2. Use multiple RPC endpoints
3. Use premium RPC service

```rust
tokio::time::sleep(Duration::from_millis(100)).await;
```

### "invalid account data"

**Problem**: Account doesn't contain expected data.

**Solution**: Verify account address and data type:

```rust
// Check account exists
let account = client.get_account(&pubkey)?;
println!("Lamports: {}", account.lamports);
```

## Logic Errors

### "reputation calculation wrong"

**Problem**: Reputation not updating correctly.

**Solution**: Call reputation calculation after proof events:

```rust
// After proof acceptance
miner_manager.record_proof_acceptance(&pubkey)?;

// After proof rejection  
miner_manager.record_proof_rejection(&pubkey)?;
```

### "reward calculation returns 0"

**Problem**: Zero rewards calculated.

**Solution**: Check parameters are valid:

```rust
// Must have accepted proofs
assert!(accepted_proofs > 0);

// Multiplier must be > 0 (fixed-point 1e6)
assert!(tensor_multiplier >= 1_000_000);

// Coherence must be in range 0-1e6
assert!(average_coherence <= 1_000_000);
```

### "pool strategy not optimal"

**Problem**: Getting wrong strategy type.

**Solution**: The optimal strategy depends on hashrate:

```rust
// Low hashrate -> Solo
// Medium hashrate -> Proportional  
// High hashrate -> PPLNS
let strategy = pool_manager.get_optimal_strategy(
    pubkey,
    hashrate,
    5,  // pool_count
)?;
```

## OpenClaw Issues

### "wallet not imported"

**Problem**: Wallet keyfile not found.

**Solution**:

```bash
# Import wallet
openclaw wallet import ./keys/burner.json --profile miner-burner-limited

# List wallets
openclaw wallet list
```

### "permission denied"

**Problem**: Wallet lacks required permissions.

**Solution**: Use correct wallet profile:

```yaml
# For mining, use miner-burner-limited
wallet_profile: miner-burner-limited

# Not farmer or staker profiles
```

### "agent not running"

**Problem**: Agent process stopped.

**Solution**:

```bash
# Check status
openclaw agent status tribewarez-tensor-miner

# Restart agent
openclaw agent restart tribewarez-tensor-miner

# Check logs for errors
openclaw logs tribewarez-tensor-miner
```

### "trigger not firing"

**Problem**: Scheduled action not executing.

**Solution**:

```yaml
# Check interval is correct (in milliseconds)
triggers:
  - id: submit_proofs
    type: interval
    interval_ms: 1800000  # 30 minutes = 30 * 60 * 1000

# Check enabled flag
enabled: true
```

## Common Pitfalls

### 1. Forgetting to Handle Errors

```rust
// Bad
let miner = miner_manager.register_miner(pubkey, caps)?;

// Good
match miner_manager.register_miner(pubkey, caps) {
    Ok(miner) => println!("Registered: {}", miner.pubkey),
    Err(OctoError::AlreadyExists(_)) => {
        println!("Miner already registered");
    }
    Err(e) => return Err(e),
}
```

### 2. Using Wrong Multiplier Scale

```rust
// Bad: Using decimal
let multiplier = 1.15; // WRONG

// Good: Using fixed-point 1e6
let multiplier = 1_150_000; // = 1.15x

// Bad: Using percentage  
let bonus = 5; // WRONG - treated as 5x, not 5%

// Good: Using fixed-point 1e6
let bonus = 50_000; // = 5%
```

### 3. Not Awaiting Async Functions

```rust
// Bad
let state = tensor_manager.query_network_state(); // Missing .await!

// Good
let state = tensor_manager.query_network_state().await?;
```

### 4. Mutability Issues

```rust
// Some methods require &mut self
// Make sure to declare as mut
let mut orchestrator = ProofOrchestrator::new(rpc)?;
orchestrator.submit_proof(proof).await?; // Works

let orchestrator = ProofOrchestrator::new(rpc)?;
orchestrator.submit_proof(proof).await?; // Error!
```

## Debugging Tips

### Enable Logging

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_subscriber::EnvFilter::new("debug"))
    .init();
```

### Print State

```rust
// Debug print
println!("{:?}", network_state);

// Specific fields
println!("Coherence: {}", network_state.coherence);
println!("Entropy: {}", network_state.total_entropy);
```

### Check Types

```rust
// Print type info
println!("{:?}", std::any::type_name::<MinerCapabilities>());
```

## Getting Help

1. **Check GitHub Issues**: https://github.com/TribeWarez/octo-o-weaver/issues
2. **Join Discord**: https://discord.gg/tribewarez
3. **Review Examples**: See `examples/` folder
4. **Read Tests**: Check `src/*/tests.rs` for usage patterns

## Error Code Reference

| Code | Meaning |
|------|---------|
| 0x01 | InvalidConfig - Check constructor parameters |
| 0x02 | NotFound - Resource doesn't exist |
| 0x03 | AlreadyExists - Resource already registered |
| 0x04 | ValidationError - Input validation failed |
| 0x05 | ProofOrchestration - Proof submission failed |
| 0x06 | NetworkError - RPC/network issue |
