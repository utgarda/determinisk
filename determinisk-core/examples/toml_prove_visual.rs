//! Example: Run TOML simulation with parallel proof generation and visualization
//!
//! This example demonstrates how to:
//! 1. Load a simulation from TOML
//! 2. Start proof generation in the background
//! 3. Visualize the simulation with live proof metrics updates

#[cfg(not(all(feature = "visual", feature = "runner")))]
fn main() {
    println!("Run with --features visual,runner to enable this example");
}

#[cfg(all(feature = "visual", feature = "runner"))]
#[macroquad::main("TOML Proof + Visualization")]
async fn main() {
    use determinisk_core::{scenarios, runner::{SimulationRunner, RunnerConfig, ZkVmBackend}};
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    let toml_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "scenarios/pool_break.toml".to_string()
    };
    
    println!("Loading: {}", toml_path);
    
    // Load simulation from TOML
    let input = scenarios::from_toml_file(&toml_path)
        .expect("Failed to load TOML");
    
    println!("Simulation loaded");
    println!("Bodies: {}", input.circles.len());
    println!("Steps: {}", input.num_steps);
    
    // Configure runner for parallel proof + visualization
    #[cfg(feature = "risc0")]
    let backend = ZkVmBackend::Risc0;
    
    #[cfg(not(feature = "risc0"))]
    let backend = ZkVmBackend::Mock;
    
    let config = RunnerConfig {
        visualize: true,
        prove: true,  // Generate proof in background
        backend,
        verbose: true,
    };
    
    println!("\nStarting parallel proof generation and visualization...");
    println!("Proof will be generated in the background while you watch the simulation.");
    
    // Run simulation with parallel proof generation
    let runner = SimulationRunner::new(config);
    let result = runner.run(input).await.expect("Failed to run simulation");
    
    // Display final results
    println!("\n=== SIMULATION COMPLETE ===");
    println!("Execution time: {:.2}s", result.execution_time_ms as f32 / 1000.0);
    
    if let Some(metrics) = result.proof_metrics {
        println!("\n=== PROOF METRICS ===");
        println!("Backend: {}", metrics.zkvm_backend);
        println!("Total cycles: {}", metrics.total_cycles);
        if let Some(user_cycles) = metrics.user_cycles {
            println!("User cycles: {}", user_cycles);
        }
        println!("Segments: {}", metrics.segments);
        println!("Proof size: {} bytes ({:.1} KB)", 
            metrics.proof_size_bytes, 
            metrics.proof_size_bytes as f32 / 1024.0);
        println!("Proving time: {:.2}s", metrics.proving_time_ms as f32 / 1000.0);
        if let Some(verify_ms) = metrics.verification_time_ms {
            println!("Verification time: {:.3}s", verify_ms as f32 / 1000.0);
        }
    }
    
    println!("\n✓ Done!");
}