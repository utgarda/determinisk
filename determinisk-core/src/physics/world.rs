//! Physics world container and simulation

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::math::{Scalar, Vec2};
use crate::physics::{Circle, CollisionConfig};
use crate::state::SimulationInput;
use serde::{Serialize, Deserialize};

/// The physics world containing all entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub bounds: Vec2,
    pub gravity: Vec2,
    pub timestep: Scalar,
    pub circles: Vec<Circle>,
    #[serde(skip)]
    pub collision_config: CollisionConfig,
}

impl World {
    /// Create a new world
    pub fn new(width: f32, height: f32) -> Self {
        World {
            bounds: Vec2::new(width, height),
            gravity: Vec2::new(0.0, -9.81),
            timestep: Scalar::from_float(1.0 / 60.0),
            circles: Vec::new(),
            collision_config: CollisionConfig::default(),
        }
    }
    
    /// Create world from declarative input
    pub fn from_input(input: &SimulationInput) -> Self {
        let mut world = World::new(input.world_width, input.world_height);
        world.gravity = Vec2::new(input.gravity[0], input.gravity[1]);
        world.timestep = Scalar::from_float(input.timestep);
        world.collision_config.restitution = Scalar::from_float(input.restitution);
        world.collision_config.position_correction = Scalar::from_float(input.position_correction);
        
        for circle_cfg in &input.circles {
            let mut circle = Circle::new(
                Vec2::new(circle_cfg.position[0], circle_cfg.position[1]),
                Scalar::from_float(circle_cfg.radius),
                Scalar::from_float(circle_cfg.mass),
            );
            circle.set_velocity(
                Vec2::new(circle_cfg.velocity[0], circle_cfg.velocity[1]),
                world.timestep,
            );
            world.add_circle(circle);
        }
        
        world
    }
    
    /// Add a circle to the world
    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }
    
    /// Perform one physics step with collision detection
    pub fn step(&mut self) {
        // Step 1: Apply forces and integrate positions (Verlet)
        for circle in &mut self.circles {
            let current = circle.position;
            
            // Calculate acceleration
            let acceleration = self.gravity;
            
            // Verlet integration
            circle.position = current * Scalar::TWO - circle.old_position 
                + acceleration * self.timestep * self.timestep;
            
            // Update velocity for collision calculations
            circle.velocity = (circle.position - circle.old_position) / self.timestep;
            
            circle.old_position = current;
        }
        
        // Step 2: Detect and resolve collisions (functional approach)
        self.circles = crate::physics::resolve_all_collisions(
            &self.circles,
            self.bounds.x,
            self.bounds.y,
            &self.collision_config,
        );
        
        // Step 3: Update velocities after collision for next frame
        for circle in &mut self.circles {
            circle.velocity = (circle.position - circle.old_position) / self.timestep;
        }
    }
    
    /// Perform physics step without collisions (for testing)
    pub fn step_no_collision(&mut self) {
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