//! Miner Lifecycle Tentacle
//!
//! Manages miner registration, capability tracking, and reputation scoring.
//! This is a MUST-HAVE tentacle that provides unified miner management.

use crate::error::{OctoError, OctoResult};
use crate::types::Miner;
use ai3_lib::MinerCapabilities;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// Miner lifecycle manager
#[derive(Debug, Clone)]
pub struct MinerLifecycle {
    miners: HashMap<Pubkey, Miner>,
    rpc_endpoint: String,
}

impl MinerLifecycle {
    /// Create a new miner lifecycle manager
    pub fn new(rpc_endpoint: String) -> OctoResult<Self> {
        if rpc_endpoint.is_empty() {
            return Err(OctoError::InvalidConfig(
                "RPC endpoint cannot be empty".to_string(),
            ));
        }

        Ok(MinerLifecycle {
            miners: HashMap::new(),
            rpc_endpoint,
        })
    }

    /// Register a new miner in the network
    pub fn register_miner(
        &mut self,
        pubkey: Pubkey,
        capabilities: MinerCapabilities,
    ) -> OctoResult<Miner> {
        // Check if miner already registered
        if self.miners.contains_key(&pubkey) {
            return Err(OctoError::AlreadyExists(format!(
                "Miner {} already registered",
                pubkey
            )));
        }

        // Validate capabilities
        self.validate_capabilities(&capabilities)?;

        let miner = Miner {
            pubkey,
            capabilities,
            reputation: 100, // Starting reputation
            total_proofs: 0,
            accepted_proofs: 0,
            rejected_proofs: 0,
        };

        self.miners.insert(pubkey, miner.clone());
        Ok(miner)
    }

    /// Get miner information by pubkey
    pub fn get_miner(&self, pubkey: &Pubkey) -> OctoResult<Miner> {
        self.miners
            .get(pubkey)
            .cloned()
            .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))
    }

    /// Update miner capabilities
    pub fn update_capabilities(
        &mut self,
        pubkey: &Pubkey,
        new_capabilities: MinerCapabilities,
    ) -> OctoResult<()> {
        // First validate the new capabilities (borrows self immutably)
        self.validate_capabilities(&new_capabilities)?;

        // Then get mutable reference to update
        let miner = self
            .miners
            .get_mut(pubkey)
            .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;

        miner.capabilities = new_capabilities;
        Ok(())
    }

    /// Record proof acceptance
    pub fn record_proof_acceptance(&mut self, pubkey: &Pubkey) -> OctoResult<()> {
        // Get the miner data first
        let (total, accepted) = {
            let miner = self
                .miners
                .get_mut(pubkey)
                .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;
            miner.total_proofs += 1;
            miner.accepted_proofs += 1;
            (miner.total_proofs, miner.accepted_proofs)
        };

        // Calculate new reputation
        let new_reputation = if total == 0 {
            100
        } else {
            let acceptance_rate = accepted as f64 / total as f64;
            (50.0 + (acceptance_rate * 50.0)) as u64
        };

        // Update reputation
        let miner = self
            .miners
            .get_mut(pubkey)
            .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;
        miner.reputation = new_reputation;

        Ok(())
    }

    /// Record proof rejection
    pub fn record_proof_rejection(&mut self, pubkey: &Pubkey) -> OctoResult<()> {
        // Get the miner data first
        let (total, rejected) = {
            let miner = self
                .miners
                .get_mut(pubkey)
                .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;
            miner.total_proofs += 1;
            miner.rejected_proofs += 1;
            (miner.total_proofs, miner.rejected_proofs)
        };

        // Calculate new reputation
        let new_reputation = if total == 0 {
            100
        } else {
            let acceptance_rate = (total - rejected) as f64 / total as f64;
            (50.0 + (acceptance_rate * 50.0)) as u64
        };

        // Update reputation
        let miner = self
            .miners
            .get_mut(pubkey)
            .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;
        miner.reputation = new_reputation;

        Ok(())
    }

    /// Calculate miner reputation score (0-100)
    pub fn calculate_reputation(&self, miner: &Miner) -> u64 {
        if miner.total_proofs == 0 {
            return 100;
        }

        let acceptance_rate = (miner.accepted_proofs as f64) / (miner.total_proofs as f64);
        // Base reputation: 50-100 based on acceptance rate
        let base_reputation = 50.0 + (acceptance_rate * 50.0);
        base_reputation as u64
    }

    /// Validate miner capabilities
    fn validate_capabilities(&self, caps: &MinerCapabilities) -> OctoResult<()> {
        if caps.supported_operations.is_empty() {
            return Err(OctoError::ValidationError(
                "Must support at least one operation".to_string(),
            ));
        }

        if caps.max_tensor_size == 0 {
            return Err(OctoError::ValidationError(
                "Max tensor size must be greater than 0".to_string(),
            ));
        }

        if caps.max_computation_time == 0 {
            return Err(OctoError::ValidationError(
                "Max computation time must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Get all registered miners
    pub fn get_all_miners(&self) -> Vec<Miner> {
        self.miners.values().cloned().collect()
    }

    /// Get top miners by reputation
    pub fn get_top_miners(&self, count: usize) -> Vec<Miner> {
        let mut miners: Vec<_> = self.miners.values().cloned().collect();
        miners.sort_by(|a, b| b.reputation.cmp(&a.reputation));
        miners.into_iter().take(count).collect()
    }

    /// Deregister a miner (disable from mining)
    pub fn deregister_miner(&mut self, pubkey: &Pubkey) -> OctoResult<()> {
        self.miners
            .remove(pubkey)
            .ok_or_else(|| OctoError::NotFound(format!("Miner {} not found", pubkey)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_capabilities() -> MinerCapabilities {
        MinerCapabilities {
            supported_operations: vec!["matrix_multiply".into(), "convolution".into()],
            max_tensor_size: 64 * 64 * 4,
            is_esp_device: false,
            max_computation_time: 300,
        }
    }

    #[test]
    fn test_miner_registration() {
        let mut lifecycle = MinerLifecycle::new("http://localhost:8899".to_string()).unwrap();
        let pubkey = Pubkey::new_unique();
        let result = lifecycle.register_miner(pubkey, test_capabilities());
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_registration() {
        let mut lifecycle = MinerLifecycle::new("http://localhost:8899".to_string()).unwrap();
        let pubkey = Pubkey::new_unique();
        let _ = lifecycle.register_miner(pubkey, test_capabilities());
        let result = lifecycle.register_miner(pubkey, test_capabilities());
        assert!(result.is_err());
    }

    #[test]
    fn test_reputation_calculation() {
        let lifecycle = MinerLifecycle::new("http://localhost:8899".to_string()).unwrap();
        let mut miner = Miner {
            pubkey: Pubkey::new_unique(),
            capabilities: test_capabilities(),
            reputation: 100,
            total_proofs: 100,
            accepted_proofs: 95,
            rejected_proofs: 5,
        };

        let reputation = lifecycle.calculate_reputation(&miner);
        assert!(reputation >= 95 && reputation <= 100);
    }
}
