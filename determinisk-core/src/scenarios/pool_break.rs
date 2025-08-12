//! Pool break scenario - 11 balls in triangle formation

use crate::state::{SimulationInput, CircleConfig};

pub fn pool_break() -> SimulationInput {
    let mut circles = Vec::new();
    
    // Cue ball
    circles.push(CircleConfig {
        position: [5.0, 10.0],
        velocity: [15.0, 0.1],  // Slight angle for interesting dynamics
        radius: 0.5,
        mass: 1.2,  // Slightly heavier cue ball
    });
    
    // Triangle of balls
    let start_x = 20.0;
    let start_y = 10.0;
    let radius = 0.5;
    let spacing = radius * 2.1;  // Slight gap between balls
    
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
    
    SimulationInput {
        world_width: 30.0,
        world_height: 20.0,
        gravity: [0.0, 0.0],  // No gravity for pool table
        timestep: 1.0 / 60.0,  // 60 Hz
        restitution: 0.95,  // Nearly elastic collisions
        position_correction: 0.8,
        circles,
        num_steps: 600,  // 10 seconds at 60 Hz
        record_trajectory: true,
        seed: 0,
    }
}