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

#[cfg(test)]
mod tests;

pub use math::{Scalar, Vec2};
pub use physics::{Circle, World};
pub use state::SimulationState;