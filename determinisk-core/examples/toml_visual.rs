//! Load TOML and visualize

use determinisk_core::{scenarios, World};

#[cfg(not(feature = "visual"))]
fn main() {
    println!("Run with --features visual to enable visualization");
}

#[cfg(feature = "visual")]
#[macroquad::main("TOML Visualization")]
async fn main() {
    use determinisk_core::render::visualize_trace;
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    let toml_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "scenarios/pool_break.toml".to_string()
    };
    
    println!("Loading: {}", toml_path);
    
    // Load from TOML
    let input = scenarios::from_toml_file(&toml_path)
        .expect("Failed to load TOML");
    
    println!("Running simulation");
    println!("Bodies: {}", input.circles.len());
    
    // Create world and run
    let mut world = World::from_input(&input);
    let trace = world.run_with_recording(input.num_steps);
    
    println!("Collisions: {}", trace.output.metrics.collision_count);
    println!("\nStarting visualization...");
    
    // Visualize
    visualize_trace(trace).await;
}