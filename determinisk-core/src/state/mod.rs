//! State management and serialization

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use serde::{Serialize, Deserialize};

/// Simulation state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub step: u64,
    pub time: f32,
    pub circles: Vec<CircleState>,
}

/// State of a single circle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleState {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
}