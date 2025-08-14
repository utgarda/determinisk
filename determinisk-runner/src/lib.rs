//! Determinisk Runner - Simulation runner with visualization and zkVM proving

pub mod runner;

#[cfg(feature = "visual")]
pub mod render;

pub mod proof;

pub use runner::{RunnerConfig, SimulationRunner, ZkVmBackend};

#[cfg(feature = "visual")]
pub use render::{visualize_trace, visualize_trace_with_updates, ProofMetrics};