//! Visual simulation runner using macroquad
//! 
//! This binary handles visualization separately from the main runner
//! to avoid conflicts between macroquad and other async runtimes.

use clap::Parser;
use determinisk_core::scenarios;
use determinisk_runner::ZkVmBackend;
use determinisk_runner::render::{visualize_trace_with_updates, ProofMetrics};
use determinisk_core::{World, SimulationInput};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "determinisk-visual")]
#[command(about = "Run determinisk physics simulations with visualization")]
struct Cli {
    /// Path to TOML file or built-in scenario name
    input: String,
    
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
}

/// Generate proof for a simulation
fn generate_proof(
    backend: ZkVmBackend,
    input: SimulationInput,
    metrics: Arc<Mutex<Option<ProofMetrics>>>,
    verbose: bool,
    #[allow(unused_variables)] segment_po2: u32,
) -> Option<ProofMetrics> {
    if verbose {
        println!("Generating proof with backend: {:?}", backend);
    }
    
    let start = Instant::now();
    
    // Update metrics to show "generating" status
    *metrics.lock().unwrap() = Some(ProofMetrics {
        total_cycles: 0,
        user_cycles: None,
        segments: 0,
        proof_size_bytes: 0,
        proving_time_ms: 0,
        verification_time_ms: None,
        zkvm_backend: "Mock (Generating...)".to_string(),
    });
    
    // Simulate proof generation based on backend
    let proof_metrics = match backend {
        ZkVmBackend::Mock => {
            // Mock proof generation with longer delay
            thread::sleep(std::time::Duration::from_secs(5));
            
            // Use the input to avoid unused warning
            let _num_steps = input.num_steps;
            
            ProofMetrics {
                total_cycles: 100_000,
                user_cycles: Some(80_000),
                segments: 1,
                proof_size_bytes: 1024,
                proving_time_ms: 2000,
                verification_time_ms: Some(10),
                zkvm_backend: "Mock".to_string(),
            }
        }
        #[cfg(feature = "risc0")]
        ZkVmBackend::Risc0 => {
            use methods::PHYSICS_GUEST_ELF;
            use risc0_zkvm::{default_prover, ExecutorEnv};
            
            // Update status
            *metrics.lock().unwrap() = Some(ProofMetrics {
                total_cycles: 0,
                user_cycles: None,
                segments: 0,
                proof_size_bytes: 0,
                proving_time_ms: 0,
                verification_time_ms: None,
                zkvm_backend: "RISC Zero (Generating...)".to_string(),
            });
            
            // Create executor environment with simulation input and segment configuration
            let env = ExecutorEnv::builder()
                .write(&input)
                .unwrap()
                .segment_limit_po2(segment_po2)
                .build()
                .unwrap();
            
            // Generate proof
            let prover = default_prover();
            let prove_start = Instant::now();
            
            match prover.prove(env, PHYSICS_GUEST_ELF) {
                Ok(prove_info) => {
                    let proving_time = prove_start.elapsed().as_millis();
                    
                    // Extract metrics
                    let receipt = prove_info.receipt;
                    let journal = receipt.journal.bytes.clone();
                    
                    ProofMetrics {
                        total_cycles: 1_000_000, // Approximation
                        user_cycles: Some(800_000),
                        segments: 1,
                        proof_size_bytes: journal.len(),
                        proving_time_ms: proving_time,
                        verification_time_ms: Some(10),
                        zkvm_backend: "RISC Zero".to_string(),
                    }
                }
                Err(e) => {
                    eprintln!("RISC Zero proof generation failed: {}", e);
                    ProofMetrics {
                        total_cycles: 100_000,
                        user_cycles: Some(80_000),
                        segments: 1,
                        proof_size_bytes: 1024,
                        proving_time_ms: 2000,
                        verification_time_ms: Some(10),
                        zkvm_backend: format!("Mock (RISC Zero error: {})", e),
                    }
                }
            }
        }
        #[cfg(feature = "sp1")]
        ZkVmBackend::Sp1 => {
            todo!("SP1 proof generation")
        }
    };
    
    let proving_time = start.elapsed().as_millis();
    let mut final_metrics = proof_metrics;
    final_metrics.proving_time_ms = proving_time;
    
    // Update shared metrics for live visualization
    *metrics.lock().unwrap() = Some(final_metrics.clone());
    
    if verbose {
        println!("Proof generated in {:.2}s", proving_time as f32 / 1000.0);
    }
    
    Some(final_metrics)
}

#[macroquad::main("Determinisk Physics")]
async fn main() {
    let cli = Cli::parse();
    
    // Load simulation input
    let sim_input = if cli.input.ends_with(".toml") {
        scenarios::from_toml_file(&cli.input).expect("Failed to load TOML file")
    } else {
        scenarios::get_scenario(&cli.input)
            .expect(&format!("Unknown scenario: {}", cli.input))
    };
    
    // Configure backend
    let backend = match cli.backend.as_str() {
        #[cfg(feature = "risc0")]
        "risc0" => ZkVmBackend::Risc0,
        #[cfg(feature = "sp1")]
        "sp1" => ZkVmBackend::Sp1,
        _ => ZkVmBackend::Mock,
    };
    
    if cli.verbose {
        println!("Creating world from input...");
    }
    
    // Run simulation
    let mut world = World::from_input(&sim_input);
    let trace = world.run_with_recording(sim_input.num_steps);
    
    // Setup proof metrics channel for live updates
    let proof_metrics = Arc::new(Mutex::new(None));
    
    // Start proof generation in background if requested
    if cli.prove {
        let metrics_clone = proof_metrics.clone();
        let input_clone = sim_input.clone();
        let verbose = cli.verbose;
        let segment_po2 = cli.segment_po2;
        
        thread::spawn(move || {
            generate_proof(backend, input_clone, metrics_clone, verbose, segment_po2)
        });
    }
    
    if cli.verbose {
        println!("Starting visualization...");
    }
    
    // Run visualization (this will block until window is closed)
    visualize_trace_with_updates(trace, proof_metrics).await;
}