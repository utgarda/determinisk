//! Example: Run simulation from TOML file

use determinisk_core::{scenarios, World};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Get TOML file path from command line or use default
    let toml_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "scenarios/pool_break.toml".to_string()
    };
    
    println!("Loading simulation from: {}", toml_path);
    
    // Load simulation input from TOML
    let input = match scenarios::from_toml_file(&toml_path) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error loading TOML file: {}", e);
            eprintln!("Usage: {} [path/to/simulation.toml]", args[0]);
            eprintln!("\nExample TOML files:");
            eprintln!("  scenarios/pool_break.toml");
            eprintln!("  scenarios/simple_drop.toml");
            return;
        }
    };
    
    // Display loaded configuration
    println!("\n=== SIMULATION CONFIGURATION ===");
    println!("World: {}x{}", input.world_width, input.world_height);
    println!("Gravity: ({:.2}, {:.2})", input.gravity[0], input.gravity[1]);
    println!("Timestep: {:.4}s", input.timestep);
    println!("Restitution: {:.2}", input.restitution);
    println!("Bodies: {}", input.circles.len());
    println!("Steps: {}", input.num_steps);
    
    // Create world from input
    let mut world = World::from_input(&input);
    
    println!("\n=== RUNNING SIMULATION ===");
    
    // Run simulation
    for step in 0..input.num_steps {
        world.step();
        
        // Print progress every 60 steps (1 second at 60Hz)
        if step % 60 == 0 {
            let time = step as f32 * input.timestep;
            println!("t={:.1}s: {} active bodies", time, world.circles.len());
        }
    }
    
    println!("\n=== FINAL STATE ===");
    for (i, circle) in world.circles.iter().enumerate() {
        println!("Body {}: pos=({:.2}, {:.2}), vel=({:.2}, {:.2})",
            i,
            circle.position.x.to_float(),
            circle.position.y.to_float(),
            circle.velocity.x.to_float(),
            circle.velocity.y.to_float(),
        );
    }
    
    // Optionally save output to JSON
    if args.len() > 2 {
        let output_path = &args[2];
        println!("\nSaving output to: {}", output_path);
        
        // Create output with metrics
        let trace = world.run_with_recording(0); // Get final state
        
        match scenarios::to_json_file(&input, output_path) {
            Ok(_) => println!("✓ Output saved successfully"),
            Err(e) => eprintln!("Error saving output: {}", e),
        }
    }
}