//! Simple drop simulation - creates a ball drop scenario programmatically

use crate::{SimulationInput, CircleConfig};

/// Create a simple ball drop simulation
pub fn simple_drop_simulation() -> SimulationInput {
    SimulationInput {
        world_width: 100.0,
        world_height: 100.0,
        gravity: [0.0, -9.81], // Standard Earth gravity
        timestep: 0.016666667, // 60 Hz
        restitution: 0.8, // Some energy loss on bounce
        position_correction: 0.8,
        circles: vec![
            CircleConfig {
                position: [50.0, 80.0], // High up in the middle
                velocity: [0.0, 0.0], // Starting at rest
                radius: 5.0,
                mass: 1.0,
            }
        ],
        num_steps: 300, // 5 seconds at 60 Hz
        record_trajectory: true,
        seed: 0,
    }
}