#![no_main]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use determinisk_core::{World, SimulationInput};
use serde::{Deserialize, Serialize};

/// Output state after simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationOutput {
    /// Final positions of all circles
    final_positions: Vec<(i32, i32)>, // Fixed-point bit representation
    /// Number of steps executed
    steps_executed: u32,
    /// Hash of final world state
    state_hash: [u8; 32],
}

fn main() {
    // Read simulation input
    let input: SimulationInput = env::read();
    
    // Initialize world from input using the unified constructor
    let mut world = World::from_input(&input);
    
    // Run simulation for specified steps
    for _ in 0..input.num_steps {
        world.step();
    }
    
    // Collect final positions (as fixed-point bit representations for determinism)
    let final_positions: Vec<(i32, i32)> = world.circles
        .iter()
        .map(|circle| (
            circle.position.x.to_bits(),
            circle.position.y.to_bits(),
        ))
        .collect();
    
    // Compute state hash for verification
    use risc0_zkvm::sha::{Impl, Sha256};
    let mut hasher = Impl::hash_bytes(&[]);
    
    for circle in &world.circles {
        let x_bytes = circle.position.x.to_bits().to_le_bytes();
        let y_bytes = circle.position.y.to_bits().to_le_bytes();
        hasher = Impl::hash_bytes(&[hasher.as_bytes(), &x_bytes, &y_bytes].concat());
    }
    
    let mut state_hash = [0u8; 32];
    state_hash.copy_from_slice(hasher.as_bytes());
    
    // Prepare output
    let output = SimulationOutput {
        final_positions,
        steps_executed: input.num_steps,
        state_hash,
    };
    
    // Commit output to journal for verification
    env::commit(&output);
}