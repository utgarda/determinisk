//! Physics world container and simulation

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::math::{Scalar, Vec2};
use crate::physics::Circle;
use serde::{Serialize, Deserialize};

/// The physics world containing all entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub bounds: Vec2,
    pub gravity: Vec2,
    pub timestep: Scalar,
    pub circles: Vec<Circle>,
}

impl World {
    /// Create a new world
    pub fn new(width: f32, height: f32) -> Self {
        World {
            bounds: Vec2::new(width, height),
            gravity: Vec2::new(0.0, -9.81),
            timestep: Scalar::from_float(1.0 / 60.0),
            circles: Vec::new(),
        }
    }
    
    /// Add a circle to the world
    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }
    
    /// Perform one physics step
    pub fn step(&mut self) {
        // Basic Verlet integration (no collisions yet)
        for circle in &mut self.circles {
            let current = circle.position;
            
            // Calculate acceleration
            let acceleration = self.gravity;
            
            // Verlet integration
            circle.position = current * Scalar::TWO - circle.old_position 
                + acceleration * self.timestep * self.timestep;
            
            circle.old_position = current;
            
            // Simple boundary check
            if circle.position.y - circle.radius < Scalar::ZERO {
                circle.position.y = circle.radius;
                circle.old_position.y = circle.radius;
            }
        }
    }
}