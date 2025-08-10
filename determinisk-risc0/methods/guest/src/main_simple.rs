#![no_main]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

/// Fixed-point arithmetic using Q16.16 format
type Fixed = i32;

const FIXED_POINT_SHIFT: i32 = 16;
const GRAVITY: Fixed = -10 << FIXED_POINT_SHIFT; // -10.0 in fixed-point

fn to_fixed(x: f32) -> Fixed {
    (x * (1 << FIXED_POINT_SHIFT) as f32) as Fixed
}

fn from_fixed(x: Fixed) -> f32 {
    x as f32 / (1 << FIXED_POINT_SHIFT) as f32
}

fn mul_fixed(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FIXED_POINT_SHIFT) as Fixed
}

/// Simple physics simulation input
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsInput {
    /// Initial position (x, y) in floating point
    initial_position: (f32, f32),
    /// Initial velocity (vx, vy) in floating point
    initial_velocity: (f32, f32),
    /// Number of timesteps to simulate
    timesteps: u32,
    /// Timestep size
    dt: f32,
}

/// Physics simulation output
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsOutput {
    /// Final position in fixed-point representation
    final_position: (i32, i32),
    /// Number of steps executed
    steps_executed: u32,
    /// Hash of trajectory
    trajectory_hash: [u8; 32],
}

fn main() {
    // Read simulation parameters
    let input: SimplePhysicsInput = env::read();
    
    // Convert to fixed-point
    let mut pos_x = to_fixed(input.initial_position.0);
    let mut pos_y = to_fixed(input.initial_position.1);
    let mut vel_x = to_fixed(input.initial_velocity.0);
    let mut vel_y = to_fixed(input.initial_velocity.1);
    let dt = to_fixed(input.dt);
    
    // Initialize trajectory hash
    use risc0_zkvm::sha::{Impl, Sha256};
    let mut hasher = Impl::hash_bytes(&[]);
    
    // Simulate physics
    for _ in 0..input.timesteps {
        // Update velocity with gravity
        vel_y = vel_y + mul_fixed(GRAVITY, dt);
        
        // Update position
        pos_x = pos_x + mul_fixed(vel_x, dt);
        pos_y = pos_y + mul_fixed(vel_y, dt);
        
        // Ground collision (y = 0)
        if pos_y < 0 {
            pos_y = 0;
            vel_y = 0;
        }
        
        // Add position to trajectory hash
        let x_bytes = pos_x.to_le_bytes();
        let y_bytes = pos_y.to_le_bytes();
        hasher = Impl::hash_bytes(&[hasher.as_bytes(), &x_bytes, &y_bytes].concat());
    }
    
    // Prepare output
    let mut trajectory_hash = [0u8; 32];
    trajectory_hash.copy_from_slice(hasher.as_bytes());
    
    let output = SimplePhysicsOutput {
        final_position: (pos_x, pos_y),
        steps_executed: input.timesteps,
        trajectory_hash,
    };
    
    // Commit result to journal
    env::commit(&output);
}