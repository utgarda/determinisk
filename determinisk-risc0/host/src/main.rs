use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};

/// Input configuration for physics simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationInput {
    /// World dimensions
    width: f32,
    height: f32,
    /// Initial circles configuration
    circles: Vec<CircleConfig>,
    /// Number of simulation steps
    steps: u32,
}

/// Circle configuration for initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CircleConfig {
    position: (f32, f32),
    velocity: (f32, f32),
    radius: f32,
    mass: f32,
}

/// Output state after simulation
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

    // Create a simple physics simulation scenario
    let input = SimulationInput {
        width: 200.0,
        height: 200.0,
        circles: vec![
            // Ball 1: Dropped from height
            CircleConfig {
                position: (50.0, 150.0),
                velocity: (0.0, 0.0),
                radius: 5.0,
                mass: 1.0,
            },
            // Ball 2: Moving horizontally
            CircleConfig {
                position: (100.0, 50.0),
                velocity: (20.0, 0.0),
                radius: 5.0,
                mass: 1.0,
            },
            // Ball 3: Projectile motion
            CircleConfig {
                position: (20.0, 20.0),
                velocity: (30.0, 40.0),
                radius: 3.0,
                mass: 0.5,
            },
        ],
        steps: 100, // Simulate for 100 timesteps
    };

    println!("Creating physics simulation proof...");
    println!("World: {}x{}", input.width, input.height);
    println!("Circles: {}", input.circles.len());
    println!("Steps: {}", input.steps);

    // Create executor environment with simulation input
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover
    let prover = default_prover();

    // Generate the proof
    println!("\nGenerating proof...");
    let start_time = std::time::Instant::now();
    
    let prove_info = prover
        .prove(env, PHYSICS_GUEST_ELF)
        .expect("Failed to generate proof");

    let proving_time = start_time.elapsed();

    // Extract the receipt
    let receipt = prove_info.receipt;

    // Decode the output from the journal
    let output: SimulationOutput = receipt.journal.decode().unwrap();

    println!("\nSimulation Results:");
    println!("Steps executed: {}", output.steps_executed);
    println!("State hash: {:?}", hex::encode(&output.state_hash));
    println!("\nFinal positions (fixed-point representation):");
    for (i, (x, y)) in output.final_positions.iter().enumerate() {
        // Convert back to float for display
        use determinisk_core::Scalar;
        let x_float = Scalar::from_bits(*x).to_float();
        let y_float = Scalar::from_bits(*y).to_float();
        println!("  Circle {}: ({:.2}, {:.2})", i, x_float, y_float);
    }

    // Serialize the proof to get actual size
    let proof_bytes = bincode::serialize(&receipt).unwrap();
    let proof_size = proof_bytes.len();

    // Verify the proof
    println!("\nVerifying proof...");
    let verify_start = std::time::Instant::now();
    receipt
        .verify(PHYSICS_GUEST_ID)
        .expect("Proof verification failed");
    let verify_time = verify_start.elapsed();
    
    println!("✓ Proof verified successfully!");

    // Display actual proof metrics
    println!("\n=== ACTUAL PROOF METRICS ===");
    println!("Backend: RISC Zero");
    println!("Total cycles: {}", prove_info.stats.total_cycles);
    println!("User cycles: {}", prove_info.stats.user_cycles);
    println!("Segments: {}", prove_info.stats.segments);
    println!("Proof size: {} bytes ({:.1} KB)", proof_size, proof_size as f32 / 1024.0);
    println!("Proving time: {:.2}s", proving_time.as_secs_f32());
    println!("Verification time: {:.3}s", verify_time.as_secs_f32());
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