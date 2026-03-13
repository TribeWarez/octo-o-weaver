# OpenClaw Deployment Guide

This guide explains how to deploy autonomous TribeWarez agents using OpenClaw - a self-hosted agent runtime with 24/7 heartbeat scheduling, wallet signing, and more.

## What is OpenClaw?

OpenClaw is a self-hosted autonomous agent runtime that:
- Runs 24/7 as a service
- Executes triggers on schedules (time-based, event-based)
- Manages wallet signing with permission limits
- Integrates with Telegram, Discord, and other platforms

## Prerequisites

### System Requirements

- **OS**: Linux, macOS, or Windows with WSL2
- **Docker**: Docker Engine 20.10+
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 10GB for logs and data

### Required Files

All files are included in octo-O-weaver:

```
openclaw-skills/
├── tribewarez-tensor-miner.yaml
├── tribewarez-yield-farmer.yaml
└── tribewarez-staking-optimizer.yaml

openclaw-configs/
└── wallet-permissions.yaml
```

## Quick Start

### Step 1: Install OpenClaw

```bash
# Clone the repository
git clone https://github.com/openclaw/openclaw
cd openclaw

# Start with Docker
docker-compose up -d

# Check status
docker-compose ps
```

### Step 2: Add TribeWarez Skills

```bash
# Copy skill definitions
cp /path/to/octo-O-weaver/openclaw-skills/*.yaml ./skills/

# Copy wallet permissions
cp /path/to/octo-O-weaver/openclaw-configs/wallet-permissions.yaml ./config/
```

### Step 3: Configure RPC Endpoint

Edit the skill YAML to set your RPC:

```yaml
rpc:
  endpoint: "https://api.mainnet-beta.solana.com"
  timeout_ms: 30000
  fallback_endpoints:
    - "https://api.rpcpool.com"
```

### Step 4: Import Wallet

```bash
# Generate a burner wallet (for testing)
solana-keygen new --outfile ./keys/burner.json

# Import to OpenClaw
openclaw wallet import ./keys/burner.json --profile miner-burner-limited
```

### Step 5: Start Agent

```bash
# Start the tensor mining agent
openclaw agent start tribewarez-tensor-miner

# Check logs
openclaw logs -f tribewarez-tensor-miner
```

## Skill Definitions

### Tensor Miner Skill

```yaml
skill:
  id: tribewarez-tensor-miner
  name: "TribeWarez Tensor Mining Agent"
  version: "0.1.0"

provider:
  type: octo_o_weaver

triggers:
  - id: submit_proofs
    type: interval
    interval_ms: 1800000  # Every 30 minutes
    action: |
      Call ProofOrchestrator::submit_proof()
      with local ai3-lib tensor generation

  - id: claim_rewards
    type: interval
    interval_ms: 3600000  # Every hour
    action: |
      Call RewardDistributor::claim_rewards()

wallet_profile: miner-burner-limited
```

### Yield Farmer Skill

```yaml
skill:
  id: tribewarez-yield-farmer
  name: "TribeWarez Liquidity Yield Farmer"

triggers:
  - id: auto_compound
    type: interval
    interval_ms: 86400000  # Every 24 hours
    action: |
      Query pool yields
      Claim rewards
      Re-deposit

wallet_profile: farmer-burner-limited
```

### Staking Optimizer Skill

```yaml
skill:
  id: tribewarez-staking-optimizer
  name: "TribeWarez Staking Optimizer"

triggers:
  - id: rebalance_stakes
    type: interval
    interval_ms: 3600000  # Every hour
    action: |
      Query APYs
      Calculate optimal allocation
      Execute rebalancing

wallet_profile: staker-burner-limited
```

## Wallet Permission Profiles

### Miner Burner (Limited)

```yaml
miner-burner-limited:
  max_balance: 1_SOL
  rate_limit:
    txs_per_minute: 10
  
  allowed_programs:
    - pot-o-mining::submit_proof
    - pot-o-mining::claim_rewards
    - tribewarez-staking::stake
    - tribewarez-staking::unstake
  
  forbidden_programs:
    - tribewarez-governance::*
    - system_program::transfer
```

### Farmer Burner (Limited)

```yaml
farmer-burner-limited:
  max_balance: 2_SOL
  rate_limit:
    txs_per_minute: 5
  
  allowed_programs:
    - tribewarez-liquidity::add_liquidity
    - tribewarez-liquidity::remove_liquidity
    - tribewarez-staking::stake
    - tribewarez-swap::swap_exact_in
```

### Staker Burner (Limited)

```yaml
staker-burner-limited:
  max_balance: 1_SOL
  rate_limit:
    txs_per_minute: 3
  
  allowed_programs:
    - tribewarez-staking::*
    - tribewarez-vault::create_lock
    - tribewarez-vault::extend_lock
```

## Monitoring & Alerts

### Built-in Metrics

```yaml
monitoring:
  track_metrics:
    - proofs_submitted_count
    - rewards_earned_pptc
    - proof_success_rate
    - avg_proof_quality_score
    
  alerts:
    - type: failure_rate
      threshold: 10_percent
      action: notify_operator
      
    - type: reward_milestone
      threshold: 100_pptc
      action: notify_operator
```

### Viewing Metrics

```bash
# View agent metrics
openclaw metrics tribewarez-tensor-miner

# View logs
openclaw logs tribewarez-tensor-miner --level info

# View transactions
openclaw tx list --agent tribewarez-tensor-miner
```

### External Monitoring

Integrate with Prometheus, Grafana, or custom webhooks:

```yaml
monitoring:
  prometheus:
    enabled: true
    port: 9090
    
  webhook:
    url: "https://your-server.com/webhook"
    events: [failure, milestone]
```

## Emergency Procedures

### Emergency Stop

```bash
# Stop all agents
openclaw agent stop --all

# Stop specific agent
openclaw agent stop tribewarez-tensor-miner
```

### Emergency Withdrawal

```yaml
# Add to skill definition
triggers:
  - id: emergency_withdrawal
    type: manual
    action: |
      Withdraw all liquidity
      Convert to USDC
      Transfer to safe wallet
```

### Recovery

```bash
# Check agent state
openclaw agent status tribewarez-tensor-miner

# Resume from checkpoint
openclaw agent resume tribewarez-tensor-miner

# Clear state and restart
openclaw agent reset tribewarez-tensor-miner
```

## Security Best Practices

### 1. Use Burner Wallets

Never use main wallets. Generate separate burner wallets:

```bash
solana-keygen new --outfile ./keys/agent-burner.json
```

### 2. Set Rate Limits

Always set rate limits in wallet profiles:

```yaml
rate_limit:
  txs_per_minute: 10
  txs_per_hour: 100
```

### 3. Enable Alerts

```yaml
alerts:
  - type: failure_rate
    threshold: 10_percent
    action: notify_operator
```

### 4. Regular Monitoring

```bash
# Check agent health
openclaw health

# Review transactions
openclaw tx list --last 24h
```

## Troubleshooting

### Agent Not Starting

```bash
# Check logs
openclaw logs tribewarez-tensor-miner

# Common issues:
# - Invalid RPC endpoint
# - Wallet not imported
# - Permission denied
```

### RPC Timeouts

```yaml
rpc:
  timeout_ms: 60000  # Increase timeout
  fallback_endpoints:
    - "https://api.rpcpool.com"
    - "https://solana-api.projectserum.com"
```

### Insufficient Funds

```bash
# Check wallet balance
solana balance ./keys/burner.json

# Top up if needed
solana transfer ./keys/burner.json 1.0
```

### Transaction Failures

```bash
# View failed transactions
openclaw tx list --status failed

# Common causes:
# - Slippage exceeded
# - Insufficient liquidity
# - Network congestion
```

## Production Deployment

### Docker Compose

```yaml
version: '3.8'
services:
  openclaw:
    image: openclaw/openclaw:latest
    ports:
      - "8080:8080"
    volumes:
      - ./keys:/app/keys
      - ./skills:/app/skills
      - ./config:/app/config
    environment:
      - RPC_ENDPOINT=https://api.mainnet-beta.solana.com
      - LOG_LEVEL=info
```

### Systemd Service

```ini
[Unit]
Description=OpenClaw TribeWarez Agent
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/openclaw
ExecStart=/usr/local/bin/openclaw agent start --all
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

## Support

- OpenClaw GitHub: https://github.com/openclaw/openclaw
- TribeWarez Discord: https://discord.gg/tribewarez
- Emergency: Check logs with `openclaw logs -f`
