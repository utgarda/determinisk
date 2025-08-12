//! Pool break simulation - creates a pool break scenario programmatically

use crate::{SimulationInput, CircleConfig};

/// Create a pool break simulation with 11 balls
pub fn pool_break_simulation() -> SimulationInput {
    let mut circles = Vec::new();
    
    // Cue ball - moving fast towards the triangle
    circles.push(CircleConfig {
        position: [5.0, 10.0],
        velocity: [15.0, 0.1], // Slight angle for more interesting dynamics
        radius: 0.5,
        mass: 1.2, // Slightly heavier cue ball
    });
    
    // Set up triangle of balls (like pool/billiards)
    let start_x = 20.0;
    let start_y = 10.0;
    let radius = 0.5;
    let spacing = radius * 2.0 * 0.866; // sqrt(3)/2 for tight packing
    
    // Row 1: 1 ball (apex)
    circles.push(CircleConfig {
        position: [start_x, start_y],
        velocity: [0.0, 0.0],
        radius,
        mass: 1.0,
    });
    
    // Row 2: 2 balls
    for i in 0..2 {
        circles.push(CircleConfig {
            position: [start_x + spacing, start_y - radius + radius * 2.0 * i as f32],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    // Row 3: 3 balls
    for i in 0..3 {
        circles.push(CircleConfig {
            position: [start_x + spacing * 2.0, start_y - radius * 2.0 + radius * 2.0 * i as f32],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    // Row 4: 4 balls
    for i in 0..4 {
        circles.push(CircleConfig {
            position: [start_x + spacing * 3.0, start_y - radius * 3.0 + radius * 2.0 * i as f32],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    SimulationInput {
        world_width: 30.0,
        world_height: 20.0,
        gravity: [0.0, 0.0], // No gravity for pool table
        timestep: 0.016666667, // 60 Hz
        restitution: 0.95, // High elasticity for pool balls
        position_correction: 0.8,
        circles,
        num_steps: 600, // 10 seconds at 60 Hz
        record_trajectory: true,
        seed: 0,
    }
}