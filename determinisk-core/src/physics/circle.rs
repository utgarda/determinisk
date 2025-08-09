//! Circle entity for physics simulation

use crate::math::{Scalar, Vec2};
use serde::{Serialize, Deserialize};

/// A physics circle with position, velocity, and properties
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circle {
    pub position: Vec2,
    pub old_position: Vec2,  // For Verlet integration
    pub radius: Scalar,
    pub mass: Scalar,
    pub restitution: Scalar,
    pub friction: Scalar,
}

impl Circle {
    /// Create a new circle
    pub fn new(position: Vec2, radius: Scalar, mass: Scalar) -> Self {
        Circle {
            position,
            old_position: position,
            radius,
            mass,
            restitution: Scalar::from_float(0.5),
            friction: Scalar::from_float(0.1),
        }
    }
    
    /// Get current velocity from position history
    pub fn velocity(&self, dt: Scalar) -> Vec2 {
        (self.position - self.old_position) / dt
    }
    
    /// Set velocity by adjusting old_position
    pub fn set_velocity(&mut self, velocity: Vec2, dt: Scalar) {
        self.old_position = self.position - velocity * dt;
    }
}