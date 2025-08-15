//! Simulation runner with parallel proof generation and visualization support

use determinisk_core::{SimulationInput, SimulationTrace, World};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[cfg(feature = "visual")]
use crate::render::ProofMetrics;

/// Configuration for simulation runner
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    /// Enable visualization
    pub visualize: bool,
    /// Generate zkVM proof
    pub prove: bool,
    /// zkVM backend to use
    pub backend: ZkVmBackend,
    /// Verbose output
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum ZkVmBackend {
    Mock,
    #[cfg(feature = "risc0")]
    Risc0,
    #[cfg(feature = "sp1")]
    Sp1,
}

/// Result from running a simulation
#[derive(Debug, Clone)]
pub struct RunnerResult {
    /// The simulation trace
    pub trace: SimulationTrace,
    /// Proof metrics if proof was generated
    pub proof_metrics: Option<ProofMetrics>,
    /// Total execution time
    pub execution_time_ms: u128,
}

/// Unified simulation runner
pub struct SimulationRunner {
    config: RunnerConfig,
}

impl SimulationRunner {
    /// Create a new runner with the given configuration
    pub fn new(config: RunnerConfig) -> Self {
        Self { config }
    }
    
    /// Run a simulation from input
    pub fn run(&self, input: SimulationInput) -> Result<RunnerResult, Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        // Create world and run simulation
        if self.config.verbose {
            println!("Creating world from input...");
        }
        
        let mut world = World::from_input(&input);
        let trace = world.run_with_recording(input.num_steps);
        
        // Setup proof metrics channel for live updates
        let proof_metrics = Arc::new(Mutex::new(None));
        let proof_metrics_clone = proof_metrics.clone();
        
        // Start proof generation in background if requested
        let proof_handle = if self.config.prove {
            let backend = self.config.backend.clone();
            let input_clone = input.clone();
            let verbose = self.config.verbose;
            
            Some(thread::spawn(move || {
                generate_proof(backend, input_clone, proof_metrics_clone, verbose)
            }))
        } else {
            None
        };
        
        // Visualize if requested
        if self.config.visualize {
            println!("Visualization requires the visual binary. Run:");
            println!("  cargo run --bin visual -- {}", 
                if self.config.prove { "--prove" } else { "" }
            );
            println!("\nNote: The standard runner cannot display visualizations due to");
            println!("macroquad requiring control of the main thread.");
            return Err("Use the visual binary for visualization".into());
        }
        
        // Wait for proof generation to complete
        let final_proof_metrics = if let Some(handle) = proof_handle {
            handle.join().map_err(|_| "Proof generation thread panicked")?
        } else {
            None
        };
        
        let execution_time_ms = start.elapsed().as_millis();
        
        Ok(RunnerResult {
            trace,
            proof_metrics: final_proof_metrics,
            execution_time_ms,
        })
    }
    
    /// Run multiple simulations in parallel
    pub fn run_batch(&self, inputs: Vec<SimulationInput>) -> Vec<RunnerResult> {
        // For now, run sequentially (async parallel would require tokio runtime)
        let mut results = Vec::new();
        for input in inputs {
            let result = self.run(input).unwrap_or_else(|e| {
                eprintln!("Simulation failed: {}", e);
                RunnerResult {
                    trace: SimulationTrace {
                        input: SimulationInput {
                            world_width: 100.0,
                            world_height: 100.0,
                            gravity: [0.0, -9.81],
                            timestep: 0.016,
                            restitution: 0.8,
                            position_correction: 0.8,
                            circles: vec![],
                            num_steps: 0,
                            record_trajectory: false,
                            seed: 0,
                        },
                        states: vec![],
                        output: determinisk_core::SimulationOutput {
                            final_state: determinisk_core::SimulationState {
                                step: 0,
                                time: 0.0,
                                circles: vec![],
                                frame_collisions: 0,
                                frame_boundary_hits: 0,
                            },
                            steps_executed: 0,
                            metrics: determinisk_core::SimulationMetrics {
                                total_energy: 0.0,
                                max_velocity: 0.0,
                                collision_count: 0,
                                boundary_hits: 0,
                            },
                        },
                    },
                    proof_metrics: None,
                    execution_time_ms: 0,
                }
            });
            results.push(result);
        }
        results
    }
}

/// Generate proof for a simulation
fn generate_proof(
    backend: ZkVmBackend,
    input: SimulationInput,
    metrics: Arc<Mutex<Option<ProofMetrics>>>,
    verbose: bool,
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
            use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
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
            
            // Create executor environment with simulation input
            let env = ExecutorEnv::builder()
                .write(&input)
                .unwrap()
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
                    
                    // Verify the proof!
                    let verify_start = Instant::now();
                    match receipt.verify(PHYSICS_GUEST_ID) {
                        Ok(_) => {
                            if verbose {
                                println!("✓ Proof verified successfully!");
                            }
                        }
                        Err(e) => {
                            eprintln!("✗ Proof verification failed: {}", e);
                        }
                    }
                    let verification_time = verify_start.elapsed().as_millis();
                    
                    // Get actual proof size (serialized receipt)
                    let proof_bytes = bincode::serialize(&receipt).unwrap_or_default();
                    let proof_size = proof_bytes.len();
                    
                    // Get cycle count from stats
                    let stats = prove_info.stats;
                    let total_cycles = stats.total_cycles;
                    let user_cycles = stats.user_cycles;
                    let segments = stats.segments;
                    
                    if verbose {
                        println!("Proof size: {} KB", proof_size / 1024);
                        println!("Total cycles: {}", total_cycles);
                        println!("Segments: {}", segments);
                    }
                    
                    ProofMetrics {
                        total_cycles,
                        user_cycles: Some(user_cycles),
                        segments: segments as u32,
                        proof_size_bytes: proof_size,
                        proving_time_ms: proving_time,
                        verification_time_ms: Some(verification_time),
                        zkvm_backend: "RISC Zero".to_string(),
                    }
                }
                Err(e) => {
                    eprintln!("RISC Zero proof generation failed: {}", e);
                    // Fallback to mock
                    thread::sleep(std::time::Duration::from_secs(2));
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
            // Real SP1 proof generation would go here
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

