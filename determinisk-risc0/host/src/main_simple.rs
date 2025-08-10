use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};

/// Simple physics simulation input
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsInput {
    /// Initial position (x, y) in floating point
    initial_position: (f32, f32),
    /// Initial velocity (vx, vy) in floating point
    initial_velocity: (f32, f32),
    /// Number of timesteps to simulate
    timesteps: u32,
    /// Timestep size
    dt: f32,
}

/// Physics simulation output
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsOutput {
    /// Final position in fixed-point representation
    final_position: (i32, i32),
    /// Number of steps executed
    steps_executed: u32,
    /// Hash of trajectory
    trajectory_hash: [u8; 32],
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Create a simple projectile motion scenario
    let input = SimplePhysicsInput {
        initial_position: (0.0, 100.0),  // Start at height 100
        initial_velocity: (20.0, 0.0),    // Moving horizontally at 20 units/s
        timesteps: 100,                   // Simulate for 100 steps
        dt: 0.1,                          // 0.1 second per step
    };

    println!("Creating physics simulation proof...");
    println!("Initial position: ({}, {})", input.initial_position.0, input.initial_position.1);
    println!("Initial velocity: ({}, {})", input.initial_velocity.0, input.initial_velocity.1);
    println!("Timesteps: {}, dt: {}", input.timesteps, input.dt);

    // Create executor environment
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Generate the proof
    println!("\nGenerating proof...");
    let start_time = std::time::Instant::now();
    
    let prover = default_prover();
    let prove_info = prover
        .prove(env, PHYSICS_GUEST_ELF)
        .expect("Failed to generate proof");

    let proving_time = start_time.elapsed();
    println!("Proof generated in: {:?}", proving_time);

    // Extract and verify the receipt
    let receipt = prove_info.receipt;
    let output: SimplePhysicsOutput = receipt.journal.decode().unwrap();

    // Convert fixed-point back to float for display
    let final_x = output.final_position.0 as f32 / (1 << 16) as f32;
    let final_y = output.final_position.1 as f32 / (1 << 16) as f32;

    println!("\nSimulation Results:");
    println!("Final position: ({:.2}, {:.2})", final_x, final_y);
    println!("Steps executed: {}", output.steps_executed);
    println!("Trajectory hash: {}", hex::encode(&output.trajectory_hash));

    // Print proof statistics
    println!("\nProof Statistics:");
    println!("Total cycles: {}", prove_info.stats.total_cycles);
    println!("User cycles: {}", prove_info.stats.user_cycles);
    println!("Segments: {}", prove_info.stats.segments);

    // Verify the proof
    println!("\nVerifying proof...");
    receipt
        .verify(PHYSICS_GUEST_ID)
        .expect("Proof verification failed");
    
    println!("✓ Proof verified successfully!");

    // Serialize the proof
    let proof_bytes = bincode::serialize(&receipt).unwrap();
    println!("\nProof size: {} bytes", proof_bytes.len());
}