//! Minimal test to debug deserialization issue

use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use determinisk_core::SimulationInput;
use serde::{Deserialize, Serialize};

/// Output state after simulation (matches guest output)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationOutput {
    final_positions: Vec<(i32, i32)>,
    steps_executed: u32,
    state_hash: [u8; 32],
}

fn main() {
    // Create a minimal simulation input
    let input = SimulationInput {
        world_width: 10.0,
        world_height: 10.0,
        gravity: [0.0, -9.81],
        timestep: 0.016,
        restitution: 0.8,
        position_correction: 0.8,
        circles: vec![],
        num_steps: 1,
        record_trajectory: false,
        seed: 0,
    };

    println!("Creating minimal test proof...");
    println!("Input serialized size: {} bytes", bincode::serialize(&input).unwrap().len());
    
    // Try to serialize and deserialize locally first
    let serialized = bincode::serialize(&input).unwrap();
    let deserialized: SimulationInput = bincode::deserialize(&serialized).unwrap();
    println!("Local serialization test passed");
    
    // Create executor environment
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Generate proof
    println!("Generating proof...");
    let prover = default_prover();
    
    match prover.prove(env, PHYSICS_GUEST_ELF) {
        Ok(prove_info) => {
            println!("Proof generated successfully!");
            let output: SimulationOutput = prove_info.receipt.journal.decode().unwrap();
            println!("Steps executed: {}", output.steps_executed);
        }
        Err(e) => {
            println!("Proof generation failed: {:?}", e);
        }
    }
}