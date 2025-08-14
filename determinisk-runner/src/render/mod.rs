//! Visualization module for physics simulations
//! 
//! This module provides optional visualization using Macroquad.
//! It's only compiled when the "visual" feature is enabled.

#[cfg(feature = "visual")]
pub mod visualizer;

#[cfg(feature = "visual")]
pub use visualizer::{visualize_trace, visualize_trace_with_updates, ProofMetrics};