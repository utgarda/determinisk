//! State management and serialization

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use serde::{Serialize, Deserialize};
use crate::{World, Scalar};

/// Simulation state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub step: u64,
    pub time: f32,
    pub circles: Vec<CircleState>,
    pub frame_collisions: u32,
    pub frame_boundary_hits: u32,
}

/// State of a single circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleState {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
    pub mass: f32,
}

/// Input configuration for a simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationInput {
    // World configuration
    pub world_width: f32,
    pub world_height: f32,
    pub gravity: [f32; 2],
    pub timestep: f32,
    
    // Physics configuration
    #[serde(default = "default_restitution")]
    pub restitution: f32,  // Coefficient of restitution (0.0-1.0)
    #[serde(default = "default_position_correction")]
    pub position_correction: f32,  // Position correction factor
    
    // Objects
    pub circles: Vec<CircleConfig>,
    
    // Simulation parameters
    pub num_steps: u32,
    pub record_trajectory: bool,
    pub seed: u64,  // For deterministic randomness (0 = no seed)
    
}

fn default_restitution() -> f32 {
    0.8  // Default to 80% elastic collisions
}

fn default_position_correction() -> f32 {
    0.8  // Default correction factor
}

/// Initial configuration for a circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleConfig {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
    pub mass: f32,
}

/// Output of a simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutput {
    pub final_state: SimulationState,
    pub steps_executed: u32,
    pub metrics: SimulationMetrics,
}

/// Metrics computed during simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationMetrics {
    pub total_energy: f32,
    pub max_velocity: f32,
    pub collision_count: u32,
    pub boundary_hits: u32,
}

/// Complete trace of a simulation including all intermediate states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationTrace {
    pub input: SimulationInput,
    pub states: Vec<SimulationState>,
    pub output: SimulationOutput,
}

impl World {
    /// Capture current state as a snapshot
    pub fn capture_state(&self, step: u64) -> SimulationState {
        // Count current collisions and boundary hits
        let collisions = self.detect_collisions();
        let boundary_hits = self.detect_boundary_collisions();
        
        SimulationState {
            step,
            time: (step as f32) * self.timestep.to_float(),
            circles: self.circles.iter().map(|c| CircleState {
                position: [c.position.x.to_float(), c.position.y.to_float()],
                velocity: [c.velocity.x.to_float(), c.velocity.y.to_float()],
                radius: c.radius.to_float(),
                mass: c.mass.to_float(),
            }).collect(),
            frame_collisions: collisions.len() as u32,
            frame_boundary_hits: boundary_hits.len() as u32,
        }
    }
    
    /// Run simulation with trajectory recording
    pub fn run_with_recording(&mut self, num_steps: u32) -> SimulationTrace {
        let input = SimulationInput {
            world_width: self.bounds.x.to_float(),
            world_height: self.bounds.y.to_float(),
            gravity: [self.gravity.x.to_float(), self.gravity.y.to_float()],
            timestep: self.timestep.to_float(),
            restitution: self.collision_config.restitution.to_float(),
            position_correction: self.collision_config.position_correction.to_float(),
            circles: self.circles.iter().map(|c| CircleConfig {
                position: [c.position.x.to_float(), c.position.y.to_float()],
                velocity: [c.velocity.x.to_float(), c.velocity.y.to_float()],
                radius: c.radius.to_float(),
                mass: c.mass.to_float(),
            }).collect(),
            num_steps,
            record_trajectory: true,
            seed: 0,
        };
        
        let mut states = Vec::new();
        let mut max_velocity = 0.0f32;
        let mut collision_count = 0u32;
        let mut boundary_hits = 0u32;
        
        // Record initial state
        states.push(self.capture_state(0));
        
        // Run simulation and record each step
        for step in 1..=num_steps {
            self.step();
            states.push(self.capture_state(step as u64));
            
            // Update metrics
            for circle in &self.circles {
                let v_squared = circle.velocity.x * circle.velocity.x + 
                               circle.velocity.y * circle.velocity.y;
                if v_squared > Scalar::ZERO {
                    let vel_mag = v_squared.sqrt().to_float();
                    max_velocity = max_velocity.max(vel_mag);
                }
            }
            
            // Count collisions (simplified - would need proper event tracking)
            let collisions = self.detect_collisions();
            collision_count += collisions.len() as u32;
            
            // Count boundary hits
            let boundary_collisions = self.detect_boundary_collisions();
            boundary_hits += boundary_collisions.len() as u32;
        }
        
        // Calculate total energy
        let total_energy = self.calculate_total_energy().to_float();
        
        let output = SimulationOutput {
            final_state: states.last().unwrap().clone(),
            steps_executed: num_steps,
            metrics: SimulationMetrics {
                total_energy,
                max_velocity,
                collision_count,
                boundary_hits,
            },
        };
        
        SimulationTrace {
            input,
            states,
            output,
        }
    }
    
    /// Helper to detect collisions (for metrics)
    pub fn detect_collisions(&self) -> Vec<(usize, usize)> {
        use crate::spatial::SpatialGrid;
        
        let max_radius = self.circles.iter()
            .map(|c| c.radius)
            .max()
            .unwrap_or(Scalar::from_float(1.0));
        let cell_size = max_radius * Scalar::from_float(2.0);
        
        let grid = SpatialGrid::build(&self.circles, cell_size, self.bounds.x, self.bounds.y);
        let pairs = grid.get_collision_pairs();
        
        let collisions = crate::spatial::detect_collisions(&self.circles, &pairs);
        collisions.iter().map(|c| (c.idx_a, c.idx_b)).collect()
    }
    
    /// Helper to detect boundary collisions (for metrics)
    fn detect_boundary_collisions(&self) -> Vec<usize> {
        let boundary_collisions = crate::spatial::detect_boundary_collisions(
            &self.circles,
            self.bounds.x,
            self.bounds.y,
        );
        boundary_collisions.iter().map(|c| c.idx).collect()
    }
    
    /// Calculate total energy of the system
    fn calculate_total_energy(&self) -> Scalar {
        let mut total = Scalar::ZERO;
        for circle in &self.circles {
            // Kinetic energy: 0.5 * m * v^2
            let v_squared = circle.velocity.x * circle.velocity.x + 
                           circle.velocity.y * circle.velocity.y;
            let kinetic = circle.mass * v_squared * Scalar::from_float(0.5);
            
            // Potential energy: m * g * h
            let potential = circle.mass * (-self.gravity.y) * circle.position.y;
            
            total = total + kinetic + potential;
        }
        total
    }
}