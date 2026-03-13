//! Error types for Octo-Weaver PaaS layer

use thiserror::Error;

/// Result type alias for Octo-Weaver operations
pub type OctoResult<T> = Result<T, OctoError>;

/// Errors that can occur in Octo-Weaver operations
#[derive(Error, Debug)]
pub enum OctoError {
    #[error("Proof orchestration error: {0}")]
    ProofOrchestration(String),

    #[error("Miner lifecycle error: {0}")]
    MinerLifecycle(String),

    #[error("Tensor network error: {0}")]
    TensorNetwork(String),

    #[error("Reward distribution error: {0}")]
    RewardDistribution(String),

    #[error("Pool strategy error: {0}")]
    PoolStrategy(String),

    #[error("Cross-chain bridge error: {0}")]
    CrossChainBridge(String),

    #[error("Governance error: {0}")]
    Governance(String),

    #[error("Liquidity error: {0}")]
    Liquidity(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Account parsing error: {0}")]
    AccountParsingError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),
}

impl From<anyhow::Error> for OctoError {
    fn from(err: anyhow::Error) -> Self {
        OctoError::InternalError(err.to_string())
    }
}

impl From<serde_json::Error> for OctoError {
    fn from(err: serde_json::Error) -> Self {
        OctoError::SerializationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = OctoError::ProofOrchestration("test error".to_string());
        assert_eq!(err.to_string(), "Proof orchestration error: test error");
    }
}
