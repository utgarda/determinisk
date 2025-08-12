//! Simple ball drop scenario

use crate::state::{SimulationInput, CircleConfig};

pub fn simple_drop() -> SimulationInput {
    SimulationInput {
        world_width: 100.0,
        world_height: 100.0,
        gravity: [0.0, -9.81],  // Earth gravity
        timestep: 1.0 / 60.0,  // 60 Hz
        restitution: 0.8,  // Some energy loss on bounce
        position_correction: 0.8,
        circles: vec![
            CircleConfig {
                position: [50.0, 80.0],  // Start high
                velocity: [0.0, 0.0],  // No initial velocity
                radius: 5.0,
                mass: 1.0,
            }
        ],
        num_steps: 120,  // 2 seconds at 60 Hz
        record_trajectory: true,
        seed: 0,
    }
}