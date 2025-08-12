//! Three-body collision scenario

use crate::state::{SimulationInput, CircleConfig};

pub fn three_body_collision() -> SimulationInput {
    SimulationInput {
        world_width: 20.0,
        world_height: 20.0,
        gravity: [0.0, -9.81],
        timestep: 1.0 / 60.0,
        restitution: 0.9,  // Mostly elastic
        position_correction: 0.8,
        circles: vec![
            CircleConfig {
                position: [5.0, 10.0],
                velocity: [5.0, 0.0],  // Moving right
                radius: 0.5,
                mass: 1.0,
            },
            CircleConfig {
                position: [15.0, 10.0],
                velocity: [-5.0, 0.0],  // Moving left
                radius: 0.5,
                mass: 1.0,
            },
            CircleConfig {
                position: [10.0, 5.0],
                velocity: [0.0, 3.0],  // Moving up
                radius: 0.3,
                mass: 0.5,  // Lighter ball
            },
        ],
        num_steps: 300,  // 5 seconds at 60 Hz
        record_trajectory: true,
        seed: 0,
    }
}