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

    println!("=== Deterministic Physics Proof Generation with Full determinisk-core ===\n");

    // Test Case 1: Ball dropped from height
    println!("Test 1: Ball dropped from height");
    let drop_input = SimulationInput {
        width: 200.0,
        height: 200.0,
        circles: vec![
            CircleConfig {
                position: (50.0, 150.0),
                velocity: (0.0, 0.0),
                radius: 5.0,
                mass: 1.0,
            },
        ],
        steps: 50,
    };
    generate_and_verify_proof("Drop Test", drop_input);

    // Test Case 2: Horizontal projectile
    println!("\nTest 2: Horizontal projectile");
    let projectile_input = SimulationInput {
        width: 200.0,
        height: 200.0,
        circles: vec![
            CircleConfig {
                position: (10.0, 100.0),
                velocity: (30.0, 0.0),
                radius: 5.0,
                mass: 1.0,
            },
        ],
        steps: 60,
    };
    generate_and_verify_proof("Projectile Test", projectile_input);

    // Test Case 3: Multiple balls
    println!("\nTest 3: Multiple balls");
    let multi_input = SimulationInput {
        width: 200.0,
        height: 200.0,
        circles: vec![
            CircleConfig {
                position: (50.0, 150.0),
                velocity: (0.0, 0.0),
                radius: 5.0,
                mass: 1.0,
            },
            CircleConfig {
                position: (100.0, 120.0),
                velocity: (10.0, 0.0),
                radius: 7.0,
                mass: 2.0,
            },
            CircleConfig {
                position: (150.0, 100.0),
                velocity: (-10.0, 10.0),
                radius: 3.0,
                mass: 0.5,
            },
        ],
        steps: 40,
    };
    generate_and_verify_proof("Multi-ball Test", multi_input);

    println!("\n=== All proofs generated and verified successfully! ===");
    println!("Using full determinisk-core library with World, Circle, Vec2, and Scalar types!");
}

fn generate_and_verify_proof(test_name: &str, input: SimulationInput) {
    println!("Generating proof for: {}", test_name);
    println!("  World: {}x{}, Circles: {}, Steps: {}",
        input.width, input.height, input.circles.len(), input.steps);

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
    let output: SimulationOutput = receipt.journal.decode().unwrap();

    // Convert fixed-point back to float for display
    use determinisk_core::Scalar;
    println!("  Final positions:");
    for (i, (x, y)) in output.final_positions.iter().enumerate() {
        let x_float = Scalar::from_bits(*x).to_float();
        let y_float = Scalar::from_bits(*y).to_float();
        println!("    Circle {}: ({:.2}, {:.2})", i, x_float, y_float);
    }
    
    println!("  Proof time: {:.2}s", proving_time.as_secs_f32());
    println!("  Cycles: {} (segments: {})", 
        prove_info.stats.total_cycles, prove_info.stats.segments);

    // Verify the proof
    receipt.verify(PHYSICS_GUEST_ID)
        .expect("Proof verification failed");
    
    println!("  ✓ Proof verified!");
}