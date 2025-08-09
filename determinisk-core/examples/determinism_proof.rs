//! Prove deterministic behavior across multiple runs

use determinisk_core::{Scalar, Vec2, Circle, World};
use sha2::{Sha256, Digest};

fn hash_world_state(world: &World) -> String {
    let mut hasher = Sha256::new();
    
    // Hash all circle positions and velocities
    for circle in &world.circles {
        hasher.update(&circle.position.x.to_bits().to_le_bytes());
        hasher.update(&circle.position.y.to_bits().to_le_bytes());
        hasher.update(&circle.old_position.x.to_bits().to_le_bytes());
        hasher.update(&circle.old_position.y.to_bits().to_le_bytes());
    }
    
    // Convert to hex string
    format!("{:x}", hasher.finalize())
}

fn create_complex_world() -> World {
    let mut world = World::new(200.0, 200.0);
    
    // Add multiple balls with various initial conditions
    for i in 0..5 {
        let x = 30.0 + i as f32 * 30.0;
        let y = 150.0 - i as f32 * 10.0;
        let vx = (i as f32 - 2.0) * 5.0;
        let vy = i as f32 * 3.0;
        
        let mut ball = Circle::new(
            Vec2::new(x, y),
            Scalar::from_float(3.0 + i as f32),
            Scalar::from_float(0.5 + i as f32 * 0.3),
        );
        ball.set_velocity(Vec2::new(vx, vy), world.timestep);
        ball.restitution = Scalar::from_float(0.5 + i as f32 * 0.1);
        
        world.add_circle(ball);
    }
    
    world
}

fn main() {
    println!("Demonstrating deterministic physics simulation...\n");
    
    // Run the same simulation multiple times
    const NUM_RUNS: usize = 5;
    const STEPS_PER_RUN: usize = 1000;
    
    let mut hashes: Vec<Vec<String>> = vec![];
    
    for run in 0..NUM_RUNS {
        println!("Run {}: Simulating {} steps...", run + 1, STEPS_PER_RUN);
        
        // Create identical starting conditions
        let mut world = create_complex_world();
        let mut run_hashes = vec![];
        
        // Record initial state
        run_hashes.push(hash_world_state(&world));
        
        // Run simulation
        for step in 0..STEPS_PER_RUN {
            world.step();
            
            // Record state every 100 steps
            if (step + 1) % 100 == 0 {
                run_hashes.push(hash_world_state(&world));
            }
        }
        
        hashes.push(run_hashes);
        
        // Print final positions for this run
        println!("  Final positions:");
        for (i, circle) in world.circles.iter().enumerate() {
            println!("    Ball {}: ({:.3}, {:.3})", 
                i + 1,
                circle.position.x.to_float(),
                circle.position.y.to_float()
            );
        }
    }
    
    // Verify all runs produced identical results
    println!("\nVerifying determinism...");
    println!("Step  | Hash (first 16 chars)          | All Match?");
    println!("------|--------------------------------|------------");
    
    let num_checkpoints = hashes[0].len();
    let mut all_deterministic = true;
    
    for checkpoint in 0..num_checkpoints {
        let step = checkpoint * 100;
        let reference_hash = &hashes[0][checkpoint];
        let hash_preview = &reference_hash[..16];
        
        let all_match = hashes.iter()
            .all(|run| &run[checkpoint] == reference_hash);
        
        println!("{:5} | {} | {}", 
            step, 
            hash_preview,
            if all_match { "✓ Yes" } else { "✗ NO!" }
        );
        
        if !all_match {
            all_deterministic = false;
            // Show which runs differ
            for (run_idx, run_hashes) in hashes.iter().enumerate() {
                if &run_hashes[checkpoint] != reference_hash {
                    println!("      Run {} differs: {}", 
                        run_idx + 1, 
                        &run_hashes[checkpoint][..16]
                    );
                }
            }
        }
    }
    
    println!("\n{}", if all_deterministic {
        "✓ SUCCESS: All runs produced identical results!"
    } else {
        "✗ FAILURE: Runs produced different results!"
    });
    
    // Show bit-level precision
    println!("\nBit-level precision check:");
    println!("Comparing final position bits of Ball 1 across all runs:");
    
    for run in 0..NUM_RUNS {
        let world = create_complex_world();
        // Re-run simulation for this check
        let mut test_world = world;
        for _ in 0..STEPS_PER_RUN {
            test_world.step();
        }
        
        let x_bits = test_world.circles[0].position.x.to_bits();
        let y_bits = test_world.circles[0].position.y.to_bits();
        
        println!("  Run {}: x_bits = 0x{:08x}, y_bits = 0x{:08x}", 
            run + 1, x_bits, y_bits);
    }
}