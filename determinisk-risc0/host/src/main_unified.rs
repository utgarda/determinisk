//! Unified RISC Zero host that works with determinisk-core types

use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use determinisk_core::{scenarios, SimulationInput};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Instant;

/// Output state after simulation (matches guest output)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationOutput {
    /// Final positions of all circles
    final_positions: Vec<(i32, i32)>, // Fixed-point bit representation
    /// Number of steps executed
    steps_executed: u32,
    /// Hash of final world state
    state_hash: [u8; 32],
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Get simulation input from command line or use default
    let args: Vec<String> = env::args().collect();
    
    let input = if args.len() > 1 {
        // Load from TOML file
        println!("Loading simulation from: {}", args[1]);
        scenarios::from_toml_file(&args[1])
            .expect("Failed to load TOML file")
    } else {
        // Use default simple drop scenario
        println!("Using default simple drop scenario");
        scenarios::simple_drop_simulation()
    };

    println!("\n=== RISC ZERO PROOF GENERATION ===");
    println!("World: {}x{} m", input.world_width, input.world_height);
    println!("Gravity: ({:.1}, {:.1}) m/s²", input.gravity[0], input.gravity[1]);
    println!("Bodies: {}", input.circles.len());
    println!("Steps: {}", input.num_steps);
    println!("Timestep: {:.4} s", input.timestep);

    // Create executor environment with simulation input
    println!("\nPreparing zkVM environment...");
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover
    let prover = default_prover();

    // Generate the proof
    println!("Generating zero-knowledge proof...");
    println!("This may take several minutes for complex simulations...\n");
    
    let start_time = Instant::now();
    
    let prove_info = prover
        .prove(env, PHYSICS_GUEST_ELF)
        .expect("Failed to generate proof");

    let proving_time = start_time.elapsed();

    // Extract the receipt
    let receipt = prove_info.receipt;

    // Decode the output from the journal
    let output: SimulationOutput = receipt.journal.decode().unwrap();

    println!("\n=== SIMULATION RESULTS ===");
    println!("Steps executed: {}", output.steps_executed);
    println!("State hash: {}", hex::encode(&output.state_hash));
    
    if args.len() <= 2 || args.get(2) != Some(&"--quiet".to_string()) {
        println!("\nFinal positions (fixed-point):");
        for (i, (x, y)) in output.final_positions.iter().enumerate() {
            // Convert back to float for display
            use determinisk_core::Scalar;
            let x_float = Scalar::from_bits(*x).to_float();
            let y_float = Scalar::from_bits(*y).to_float();
            println!("  Body {}: ({:.2}, {:.2})", i, x_float, y_float);
        }
    }

    // Serialize the proof to get actual size
    let proof_bytes = bincode::serialize(&receipt).unwrap();
    let proof_size = proof_bytes.len();

    // Verify the proof
    println!("\nVerifying proof...");
    let verify_start = Instant::now();
    receipt
        .verify(PHYSICS_GUEST_ID)
        .expect("Proof verification failed");
    let verify_time = verify_start.elapsed();
    
    println!("✓ Proof verified successfully!");

    // Display actual proof metrics
    println!("\n=== PROOF METRICS ===");
    println!("Backend: RISC Zero");
    println!("Total cycles: {}", prove_info.stats.total_cycles);
    println!("User cycles: {}", prove_info.stats.user_cycles);
    println!("Segments: {}", prove_info.stats.segments);
    println!("Proof size: {} bytes ({:.1} KB)", proof_size, proof_size as f32 / 1024.0);
    println!("Proving time: {:.2}s", proving_time.as_secs_f32());
    println!("Verification time: {:.3}s", verify_time.as_secs_f32());
    
    // Calculate efficiency metrics
    let cycles_per_step = prove_info.stats.total_cycles / output.steps_executed as u64;
    let cycles_per_body = prove_info.stats.total_cycles / (input.circles.len() as u64 * output.steps_executed as u64);
    
    println!("\n=== EFFICIENCY METRICS ===");
    println!("Cycles per step: {}", cycles_per_step);
    println!("Cycles per body per step: {}", cycles_per_body);
    println!("Proof size per step: {:.1} bytes", proof_size as f32 / output.steps_executed as f32);
    
    // Save proof if requested
    if let Some(output_path) = args.get(2) {
        if output_path != "--quiet" {
            println!("\nSaving proof to: {}", output_path);
            std::fs::write(output_path, &proof_bytes)
                .expect("Failed to save proof");
            println!("✓ Proof saved successfully");
        }
    }
}

/// Example of verifying a proof from serialized data
#[allow(dead_code)]
fn verify_proof_from_bytes(proof_bytes: &[u8]) -> Result<SimulationOutput, Box<dyn std::error::Error>> {
    use risc0_zkvm::Receipt;
    
    // Deserialize the receipt
    let receipt: Receipt = bincode::deserialize(proof_bytes)?;
    
    // Verify the proof
    receipt.verify(PHYSICS_GUEST_ID)?;
    
    // Extract and return the output
    let output: SimulationOutput = receipt.journal.decode()?;
    Ok(output)
}