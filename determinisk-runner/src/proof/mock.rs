//! Mock proof backend for testing

use super::{ProofBackend, ProofMetrics};
use determinisk_core::SimulationInput;
use std::thread;
use std::time::Duration;

pub struct MockBackend;

impl ProofBackend for MockBackend {
    fn prove(&self, input: &SimulationInput) -> Result<ProofMetrics, String> {
        // Simulate proof generation delay
        thread::sleep(Duration::from_secs(5));
        
        // Generate mock metrics based on input
        let total_cycles = (input.num_steps as u64) * (input.circles.len() as u64) * 1000;
        
        Ok(ProofMetrics {
            total_cycles,
            user_cycles: Some(total_cycles * 8 / 10),
            segments: 1,
            proof_size_bytes: 4200,
            proving_time_ms: 5000,
            verification_time_ms: Some(10),
            zkvm_backend: "Mock".to_string(),
        })
    }
    
    fn verify(&self, _proof: &[u8]) -> Result<bool, String> {
        Ok(true)
    }
}