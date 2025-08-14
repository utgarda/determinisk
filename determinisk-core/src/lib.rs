//! Determinisk - A deterministic 2D physics engine for zkVM
//! 
//! This crate provides the core physics simulation functionality
//! with bit-exact determinism across platforms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod math;
pub mod physics;
pub mod spatial;
pub mod state;

#[cfg(feature = "std")]
pub mod scenarios;

#[cfg(test)]
mod tests;

pub use math::{Scalar, Vec2};
pub use physics::{Circle, World, CollisionConfig, resolve_all_collisions};
pub use spatial::{SpatialGrid, Collision, BoundaryCollision};
pub use state::{
    SimulationState, CircleState, 
    SimulationInput, CircleConfig,
    SimulationOutput, SimulationMetrics,
    SimulationTrace,
};