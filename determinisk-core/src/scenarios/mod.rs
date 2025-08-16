//! Pre-defined simulation scenarios and TOML/JSON file support

#[cfg(feature = "std")]
use std::{fs, path::Path};

use crate::state::SimulationInput;

// Import individual scenarios
mod pool_break;
mod pool_break_15;
mod simple_drop;
mod three_body_collision;
mod pool_break_sim;
mod simple_drop_sim;

// Re-export scenario functions
pub use pool_break::pool_break;
pub use pool_break_15::pool_break_15;
pub use simple_drop::simple_drop;
pub use three_body_collision::three_body_collision;
pub use pool_break_sim::pool_break_simulation;
pub use simple_drop_sim::simple_drop_simulation;

/// Load simulation from TOML file
#[cfg(all(feature = "std", feature = "toml"))]
pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<SimulationInput, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let input: SimulationInput = toml::from_str(&contents)?;
    Ok(input)
}

/// Save simulation to TOML file
#[cfg(all(feature = "std", feature = "toml"))]
pub fn to_toml_file<P: AsRef<Path>>(input: &SimulationInput, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string_pretty(input)?;
    fs::write(path, toml_string)?;
    Ok(())
}

/// Load simulation from JSON file
#[cfg(all(feature = "std", feature = "serde_json"))]
pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<SimulationInput, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let input: SimulationInput = serde_json::from_str(&contents)?;
    Ok(input)
}

/// Save simulation to JSON file
#[cfg(all(feature = "std", feature = "serde_json"))]
pub fn to_json_file<P: AsRef<Path>>(input: &SimulationInput, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(input)?;
    fs::write(path, json)?;
    Ok(())
}

/// Auto-detect format and load from file
#[cfg(feature = "std")]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SimulationInput, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    match path.extension().and_then(|s| s.to_str()) {
        #[cfg(feature = "toml")]
        Some("toml") => from_toml_file(path),
        #[cfg(feature = "serde_json")]
        Some("json") => from_json_file(path),
        _ => Err("Unsupported file format. Use .toml or .json".into()),
    }
}

/// Get scenario by name
pub fn get_scenario(name: &str) -> Option<SimulationInput> {
    match name {
        "pool_break" | "pool-break" => Some(pool_break()),
        "pool_break_15" | "pool-break-15" => Some(pool_break_15()),
        "simple_drop" | "simple-drop" => Some(simple_drop()),
        "three_body" | "three-body" | "three_body_collision" => Some(three_body_collision()),
        "pool_break_sim" | "pool-break-sim" => Some(pool_break_simulation()),
        "simple_drop_sim" | "simple-drop-sim" => Some(simple_drop_simulation()),
        _ => None,
    }
}

/// List all available scenarios
pub fn list_scenarios() -> Vec<&'static str> {
    vec![
        "pool_break",
        "pool_break_15",
        "simple_drop",
        "three_body_collision",
        "pool_break_sim",
        "simple_drop_sim",
    ]
}