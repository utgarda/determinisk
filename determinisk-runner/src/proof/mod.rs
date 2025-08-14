//! Proof backend abstraction

use determinisk_core::SimulationInput;
use serde::{Deserialize, Serialize};

pub mod mock;

// RISC Zero and SP1 backends are integrated directly in runner.rs
// They could be refactored into separate modules later

/// Proof metrics for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetrics {
    pub total_cycles: u64,
    pub user_cycles: Option<u64>,
    pub segments: u32,
    pub proof_size_bytes: usize,
    pub proving_time_ms: u128,
    pub verification_time_ms: Option<u128>,
    pub zkvm_backend: String,
}

/// Trait for proof backends
pub trait ProofBackend {
    /// Generate a proof for the simulation
    fn prove(&self, input: &SimulationInput) -> Result<ProofMetrics, String>;
    
    /// Verify a proof
    fn verify(&self, proof: &[u8]) -> Result<bool, String>;
}