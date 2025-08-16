//! CLI for running determinisk simulations

use clap::{Parser, Subcommand};
use determinisk_core::scenarios;
use determinisk_runner::{RunnerConfig, SimulationRunner, ZkVmBackend};

#[derive(Parser)]
#[command(name = "determinisk-runner")]
#[command(about = "Run determinisk physics simulations with visualization and proving")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a simulation
    Run {
        /// Path to TOML file or built-in scenario name
        input: String,
        
        /// Enable visualization
        #[arg(short, long)]
        visual: bool,
        
        /// Generate zkVM proof
        #[arg(short, long)]
        prove: bool,
        
        /// Proof backend (mock, risc0, sp1)
        #[arg(short, long, default_value = "mock")]
        backend: String,
        
        /// Segment size for RISC Zero proving (power of 2, default 20 for 6GB GPUs)
        #[arg(long, default_value = "20")]
        segment_po2: u32,
        
        /// Verbose output
        #[arg(long)]
        verbose: bool,
    },
    
    /// List available scenarios
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { input, visual, prove, backend, segment_po2, verbose } => {
            // Load simulation input
            let sim_input = if input.ends_with(".toml") {
                // Load from TOML file
                scenarios::from_toml_file(&input)?
            } else {
                // Try as built-in scenario
                scenarios::get_scenario(&input)
                    .ok_or_else(|| format!("Unknown scenario: {}", input))?
            };
            
            // Configure backend
            let backend = match backend.as_str() {
                #[cfg(feature = "risc0")]
                "risc0" => ZkVmBackend::Risc0,
                #[cfg(feature = "sp1")]
                "sp1" => ZkVmBackend::Sp1,
                _ => ZkVmBackend::Mock,
            };
            
            // Configure runner
            let config = RunnerConfig {
                visualize: visual,
                prove,
                backend,
                verbose,
                segment_po2,
            };
            
            // Run simulation
            let runner = SimulationRunner::new(config);
            let result = runner.run(sim_input)?;
            
            // Display results
            if verbose {
                println!("\n=== SIMULATION COMPLETE ===");
                println!("Execution time: {:.2}s", result.execution_time_ms as f32 / 1000.0);
                
                if let Some(metrics) = result.proof_metrics {
                    println!("\n=== PROOF METRICS ===");
                    println!("Backend: {}", metrics.zkvm_backend);
                    println!("Total cycles: {}", metrics.total_cycles);
                    if let Some(user_cycles) = metrics.user_cycles {
                        println!("User cycles: {}", user_cycles);
                    }
                    println!("Proof size: {} KB", metrics.proof_size_bytes / 1024);
                    println!("Proving time: {:.2}s", metrics.proving_time_ms as f32 / 1000.0);
                }
            }
        }
        
        Commands::List => {
            println!("Available scenarios:");
            for name in scenarios::list_scenarios() {
                println!("  - {}", name);
            }
            println!("\nYou can also provide a path to a TOML file.");
        }
    }
    
    Ok(())
}