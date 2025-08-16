//! Pool break scenario - 15 balls in triangle formation (full rack)

use crate::state::{SimulationInput, CircleConfig};

pub fn pool_break_15() -> SimulationInput {
    let mut circles = Vec::new();
    
    // Cue ball
    circles.push(CircleConfig {
        position: [5.0, 10.0],
        velocity: [18.0, 0.1],  // Faster for more balls
        radius: 0.5,
        mass: 1.2,  // Slightly heavier cue ball
    });
    
    // Full triangle of 15 balls (standard pool/snooker)
    let start_x = 20.0;
    let start_y = 10.0;
    let radius = 0.5;
    let spacing = radius * 2.05;  // Slight gap between balls
    
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
            position: [
                start_x + spacing * 0.866,  // sqrt(3)/2 for equilateral triangle
                start_y - spacing * 0.5 + spacing * i as f32,
            ],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    // Row 3: 3 balls
    for i in 0..3 {
        circles.push(CircleConfig {
            position: [
                start_x + spacing * 1.732,  // sqrt(3)
                start_y - spacing + spacing * i as f32,
            ],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    // Row 4: 4 balls
    for i in 0..4 {
        circles.push(CircleConfig {
            position: [
                start_x + spacing * 2.598,  // 3*sqrt(3)/2
                start_y - spacing * 1.5 + spacing * i as f32,
            ],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    // Row 5: 5 balls
    for i in 0..5 {
        circles.push(CircleConfig {
            position: [
                start_x + spacing * 3.464,  // 2*sqrt(3)
                start_y - spacing * 2.0 + spacing * i as f32,
            ],
            velocity: [0.0, 0.0],
            radius,
            mass: 1.0,
        });
    }
    
    SimulationInput {
        world_width: 35.0,  // Wider table for 15 balls
        world_height: 20.0,
        gravity: [0.0, 0.0],  // No gravity for pool table
        timestep: 1.0 / 60.0,  // 60 Hz
        restitution: 0.95,  // Nearly elastic collisions
        position_correction: 0.8,
        circles,
        num_steps: 800,  // Longer simulation for more balls
        record_trajectory: true,
        seed: 0,
    }
}