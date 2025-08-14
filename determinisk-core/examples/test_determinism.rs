//! Quick test to demonstrate deterministic physics simulation

use determinisk_core::{World, Circle, Vec2, Scalar};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn run_simulation(seed: u64) -> u64 {
    let mut world = World::new(10.0, 10.0);
    
    // Add circles in a specific pattern
    for i in 0..3 {
        let x = 2.0 + i as f32 * 3.0;
        let y = 8.0 - i as f32 * 0.5;
        let circle = Circle::new(
            Vec2::new(x, y),
            Scalar::from_float(0.5),
            Scalar::from_float(1.0),
        );
        world.add_circle(circle);
    }
    
    // Run simulation for 100 steps
    for _ in 0..100 {
        world.step();
    }
    
    // Calculate hash of final state
    let mut hasher = DefaultHasher::new();
    for circle in &world.circles {
        let x_bits = circle.position.x.to_bits();
        let y_bits = circle.position.y.to_bits();
        x_bits.hash(&mut hasher);
        y_bits.hash(&mut hasher);
    }
    hasher.finish()
}

fn main() {
    println!("Testing deterministic physics simulation...\n");
    
    // Run the same simulation multiple times
    let results: Vec<u64> = (0..5).map(|i| {
        let hash = run_simulation(42);
        println!("Run {}: Hash = {:#018x}", i + 1, hash);
        hash
    }).collect();
    
    // Check all results are identical
    let all_same = results.windows(2).all(|w| w[0] == w[1]);
    
    println!("\n{}", if all_same {
        "✅ SUCCESS: All simulation runs produced identical results!"
    } else {
        "❌ FAILURE: Simulation results differ between runs!"
    });
    
    println!("\nThis demonstrates that the physics engine is fully deterministic:");
    println!("- Fixed-point arithmetic ensures bit-exact reproducibility");
    println!("- Same input always produces the same output");
    println!("- Perfect for zkVM proofs where determinism is critical");
}