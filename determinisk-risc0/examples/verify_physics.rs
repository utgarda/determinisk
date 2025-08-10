use methods::{PHYSICS_GUEST_ELF, PHYSICS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};

/// Simple physics simulation input
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsInput {
    initial_position: (f32, f32),
    initial_velocity: (f32, f32),
    timesteps: u32,
    dt: f32,
}

/// Physics simulation output
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimplePhysicsOutput {
    final_position: (i32, i32),
    steps_executed: u32,
    trajectory_hash: [u8; 32],
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    println!("=== Deterministic Physics Proof Generation Demo ===\n");

    // Test Case 1: Ball dropped from height
    println!("Test 1: Ball dropped from height");
    let drop_input = SimplePhysicsInput {
        initial_position: (0.0, 50.0),   // Start at height 50
        initial_velocity: (0.0, 0.0),     // No initial velocity
        timesteps: 50,                    // Simulate for 50 steps
        dt: 0.1,                          // 0.1 second per step
    };
    generate_and_verify_proof("Drop Test", drop_input);

    // Test Case 2: Horizontal projectile
    println!("\nTest 2: Horizontal projectile");
    let projectile_input = SimplePhysicsInput {
        initial_position: (0.0, 100.0),   // Start at height 100
        initial_velocity: (30.0, 0.0),    // Moving horizontally at 30 units/s
        timesteps: 60,                    // Simulate for 60 steps
        dt: 0.1,                          // 0.1 second per step
    };
    generate_and_verify_proof("Projectile Test", projectile_input);

    // Test Case 3: Angled launch
    println!("\nTest 3: Angled launch");
    let angled_input = SimplePhysicsInput {
        initial_position: (0.0, 0.0),     // Start at ground
        initial_velocity: (20.0, 30.0),   // Launch at angle
        timesteps: 80,                    // Simulate for 80 steps
        dt: 0.1,                          // 0.1 second per step
    };
    generate_and_verify_proof("Angled Launch", angled_input);

    println!("\n=== All proofs generated and verified successfully! ===");
}

fn generate_and_verify_proof(test_name: &str, input: SimplePhysicsInput) {
    println!("Generating proof for: {}", test_name);
    println!("  Initial: ({:.1}, {:.1}), Velocity: ({:.1}, {:.1})",
        input.initial_position.0, input.initial_position.1,
        input.initial_velocity.0, input.initial_velocity.1);

    // Create executor environment
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Generate the proof
    let start_time = std::time::Instant::now();
    let prover = default_prover();
    let prove_info = prover.prove(env, PHYSICS_GUEST_ELF)
        .expect("Failed to generate proof");
    let proving_time = start_time.elapsed();

    // Extract the receipt
    let receipt = prove_info.receipt;
    let output: SimplePhysicsOutput = receipt.journal.decode().unwrap();

    // Convert fixed-point back to float for display
    let final_x = output.final_position.0 as f32 / (1 << 16) as f32;
    let final_y = output.final_position.1 as f32 / (1 << 16) as f32;

    println!("  Final position: ({:.2}, {:.2})", final_x, final_y);
    println!("  Proof time: {:.2}s", proving_time.as_secs_f32());
    println!("  Cycles: {} (segments: {})", 
        prove_info.stats.total_cycles, prove_info.stats.segments);

    // Verify the proof
    receipt.verify(PHYSICS_GUEST_ID)
        .expect("Proof verification failed");
    
    println!("  ✓ Proof verified!");
}