#![no_main]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use determinisk_core::{World, Circle, Vec2, Scalar};
use serde::{Deserialize, Serialize};

/// Input configuration for physics simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationInput {
    /// World dimensions
    width: f32,
    height: f32,
    /// Initial circles configuration
    circles: Vec<CircleConfig>,
    /// Number of simulation steps
    steps: u32,
}

/// Circle configuration for initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CircleConfig {
    position: (f32, f32),
    velocity: (f32, f32),
    radius: f32,
    mass: f32,
}

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
    
    // Initialize world
    let mut world = World::new(input.width, input.height);
    
    // Add circles to world
    for config in input.circles {
        let mut circle = Circle::new(
            Vec2::new(config.position.0, config.position.1),
            Scalar::from_float(config.radius),
            Scalar::from_float(config.mass),
        );
        
        // Set initial velocity
        circle.set_velocity(
            Vec2::new(config.velocity.0, config.velocity.1),
            world.timestep,
        );
        
        world.add_circle(circle);
    }
    
    // Run simulation for specified steps
    for _ in 0..input.steps {
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
        steps_executed: input.steps,
        state_hash,
    };
    
    // Commit output to journal for verification
    env::commit(&output);
}