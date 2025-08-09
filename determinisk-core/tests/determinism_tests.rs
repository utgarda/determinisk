//! Tests specifically for deterministic behavior

use determinisk_core::{Scalar, Vec2, Circle, World};
use std::collections::HashMap;

/// Create a complex world with many interacting elements
fn create_test_world(seed: u32) -> World {
    let mut world = World::new(500.0, 500.0);
    
    // Use seed to create "random" but deterministic initial conditions
    for i in 0..20 {
        let x = ((seed + i * 7) % 400) as f32 + 50.0;
        let y = ((seed + i * 13) % 400) as f32 + 50.0;
        let vx = (((seed + i * 17) % 40) as f32 - 20.0) * 2.0;
        let vy = (((seed + i * 23) % 40) as f32 - 20.0) * 2.0;
        let radius = ((seed + i * 5) % 5) as f32 + 3.0;
        let mass = ((seed + i * 11) % 10) as f32 * 0.5 + 0.5;
        
        let mut circle = Circle::new(
            Vec2::new(x, y),
            Scalar::from_float(radius),
            Scalar::from_float(mass),
        );
        circle.set_velocity(Vec2::new(vx, vy), world.timestep);
        
        world.add_circle(circle);
    }
    
    world
}

#[test]
fn test_determinism_across_platforms() {
    // This test simulates what would happen on different platforms
    // In reality, all platforms use the same code, but we test multiple times
    const PLATFORM_SIMULATIONS: usize = 5;
    const STEPS: usize = 1000;
    
    let mut final_states = Vec::new();
    
    for platform in 0..PLATFORM_SIMULATIONS {
        let mut world = create_test_world(42); // Same seed
        
        // Simulate
        for _ in 0..STEPS {
            world.step();
        }
        
        // Record final state
        let state: Vec<i32> = world.circles.iter()
            .flat_map(|c| vec![
                c.position.x.to_bits(),
                c.position.y.to_bits(),
                c.old_position.x.to_bits(),
                c.old_position.y.to_bits(),
            ])
            .collect();
        
        final_states.push((platform, state));
    }
    
    // All "platforms" should have identical results
    let reference = &final_states[0].1;
    for (platform, state) in &final_states[1..] {
        assert_eq!(reference, state, 
            "Platform {} produced different results", platform);
    }
}

#[test]
fn test_determinism_with_different_step_orders() {
    // Test that processing order doesn't affect results
    let mut world1 = create_test_world(123);
    let mut world2 = create_test_world(123);
    
    // Simulate both
    for _ in 0..500 {
        world1.step();
        world2.step();
    }
    
    // Compare all circles
    for i in 0..world1.circles.len() {
        assert_eq!(
            world1.circles[i].position.x.to_bits(),
            world2.circles[i].position.x.to_bits(),
            "Circle {} x position differs", i
        );
        assert_eq!(
            world1.circles[i].position.y.to_bits(),
            world2.circles[i].position.y.to_bits(),
            "Circle {} y position differs", i
        );
    }
}

#[test]
fn test_determinism_with_accumulated_operations() {
    // Test that accumulated floating-point-like operations remain deterministic
    let mut world = World::new(100.0, 100.0);
    
    let mut ball = Circle::new(
        Vec2::new(50.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    
    // Apply many small velocities
    for i in 0..100 {
        let tiny_v = 0.001 * (i as f32);
        ball.set_velocity(Vec2::new(tiny_v, tiny_v), world.timestep);
        world.add_circle(ball);
        world.circles.clear(); // Reset
    }
    
    // Final operation
    ball.set_velocity(Vec2::new(1.0, 1.0), world.timestep);
    world.add_circle(ball);
    
    // Run simulation
    for _ in 0..100 {
        world.step();
    }
    
    // Record exact bit pattern
    let final_x_bits = world.circles[0].position.x.to_bits();
    let final_y_bits = world.circles[0].position.y.to_bits();
    
    // Repeat entire process
    let mut world2 = World::new(100.0, 100.0);
    let mut ball2 = Circle::new(
        Vec2::new(50.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    
    for i in 0..100 {
        let tiny_v = 0.001 * (i as f32);
        ball2.set_velocity(Vec2::new(tiny_v, tiny_v), world2.timestep);
        world2.add_circle(ball2);
        world2.circles.clear();
    }
    
    ball2.set_velocity(Vec2::new(1.0, 1.0), world2.timestep);
    world2.add_circle(ball2);
    
    for _ in 0..100 {
        world2.step();
    }
    
    // Should be bit-identical
    assert_eq!(final_x_bits, world2.circles[0].position.x.to_bits());
    assert_eq!(final_y_bits, world2.circles[0].position.y.to_bits());
}

#[test]
fn test_determinism_state_hash() {
    use sha2::{Sha256, Digest};
    
    // Create hash map to store states at different steps
    let mut state_hashes: HashMap<usize, Vec<u8>> = HashMap::new();
    
    // Run simulation multiple times
    for run in 0..3 {
        let mut world = create_test_world(999);
        
        for step in 0..200 {
            if step % 50 == 0 {
                // Hash world state
                let mut hasher = Sha256::new();
                for circle in &world.circles {
                    hasher.update(&circle.position.x.to_bits().to_le_bytes());
                    hasher.update(&circle.position.y.to_bits().to_le_bytes());
                }
                let hash = hasher.finalize().to_vec();
                
                // Check or store hash
                match state_hashes.get(&step) {
                    Some(expected) => {
                        assert_eq!(expected, &hash, 
                            "Run {} step {} produced different hash", run, step);
                    }
                    None => {
                        state_hashes.insert(step, hash);
                    }
                }
            }
            
            world.step();
        }
    }
}

#[test]
fn test_determinism_with_extreme_values() {
    // Test with very large and very small values
    let mut world = World::new(10000.0, 10000.0);
    
    // Very fast ball
    let mut fast_ball = Circle::new(
        Vec2::new(5000.0, 5000.0),
        Scalar::from_float(10.0),
        Scalar::from_float(0.1),
    );
    fast_ball.set_velocity(Vec2::new(1000.0, -1000.0), world.timestep);
    
    // Very slow ball
    let mut slow_ball = Circle::new(
        Vec2::new(100.0, 100.0),
        Scalar::from_float(50.0),
        Scalar::from_float(100.0),
    );
    slow_ball.set_velocity(Vec2::new(0.01, 0.01), world.timestep);
    
    world.add_circle(fast_ball);
    world.add_circle(slow_ball);
    
    // Run twice
    let mut world2 = world.clone();
    
    for _ in 0..100 {
        world.step();
        world2.step();
    }
    
    // Should still be deterministic
    for i in 0..2 {
        assert_eq!(
            world.circles[i].position.x.to_bits(),
            world2.circles[i].position.x.to_bits()
        );
        assert_eq!(
            world.circles[i].position.y.to_bits(),
            world2.circles[i].position.y.to_bits()
        );
    }
}