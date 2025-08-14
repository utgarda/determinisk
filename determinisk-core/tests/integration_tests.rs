//! Integration tests for the physics engine

use determinisk_core::{Scalar, Vec2, Circle, World};
use sha2::{Sha256, Digest};

/// Helper to hash world state for determinism checks
fn hash_world_state(world: &World) -> [u8; 32] {
    let mut hasher = Sha256::new();
    
    for circle in &world.circles {
        hasher.update(&circle.position.x.to_bits().to_le_bytes());
        hasher.update(&circle.position.y.to_bits().to_le_bytes());
        hasher.update(&circle.old_position.x.to_bits().to_le_bytes());
        hasher.update(&circle.old_position.y.to_bits().to_le_bytes());
    }
    
    hasher.finalize().into()
}

#[test]
fn test_gravity_fall() {
    let mut world = World::new(100.0, 100.0);
    
    let ball = Circle::new(
        Vec2::new(50.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(ball);
    
    let initial_y = world.circles[0].position.y;
    
    // Simulate until ball hits ground (max 200 steps)
    for _ in 0..200 {
        world.step();
        
        // Check if ball hit ground (y - radius <= 0)
        if world.circles[0].position.y <= world.circles[0].radius {
            break;
        }
    }
    
    // Ball should have fallen
    assert!(world.circles[0].position.y < initial_y);
    
    // Ball should stop at ground (y = radius) - test bit-exact equality
    assert_eq!(
        world.circles[0].position.y, 
        world.circles[0].radius, 
        "Ball should be exactly at ground level"
    );
}

#[test]
fn test_determinism_single_ball() {
    // Run same simulation twice
    let mut world1 = World::new(100.0, 100.0);
    let mut world2 = World::new(100.0, 100.0);
    
    let ball = Circle::new(
        Vec2::new(50.0, 80.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    
    world1.add_circle(ball);
    world2.add_circle(ball);
    
    // Run both for 100 steps
    for _ in 0..100 {
        world1.step();
        world2.step();
    }
    
    // Results should be bit-identical
    assert_eq!(
        world1.circles[0].position.x.to_bits(),
        world2.circles[0].position.x.to_bits()
    );
    assert_eq!(
        world1.circles[0].position.y.to_bits(),
        world2.circles[0].position.y.to_bits()
    );
}

#[test]
fn test_determinism_complex_scenario() {
    const NUM_RUNS: usize = 3;
    const STEPS: usize = 500;
    
    let mut hashes = Vec::new();
    
    for _ in 0..NUM_RUNS {
        let mut world = World::new(200.0, 200.0);
        
        // Add multiple balls with different properties
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
            
            world.add_circle(ball);
        }
        
        // Run simulation
        for _ in 0..STEPS {
            world.step();
        }
        
        hashes.push(hash_world_state(&world));
    }
    
    // All runs should produce identical hash
    for i in 1..NUM_RUNS {
        assert_eq!(hashes[0], hashes[i], "Run {} produced different results", i);
    }
}

#[test]
fn test_energy_conservation() {
    let mut world = World::new(100.0, 200.0);
    
    // Drop a ball from height
    let ball = Circle::new(
        Vec2::new(50.0, 150.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(ball);
    
    // Calculate initial energy
    let initial_height = world.circles[0].position.y.to_float();
    let initial_pe = 9.81 * initial_height;
    let initial_ke = 0.0; // Starting at rest
    let initial_energy = initial_pe + initial_ke;
    
    // Run for a short time (before hitting ground)
    for _ in 0..30 {
        world.step();
    }
    
    // Calculate final energy
    let final_height = world.circles[0].position.y.to_float();
    let velocity = world.circles[0].velocity;
    let speed = velocity.magnitude().to_float();
    let final_pe = 9.81 * final_height;
    let final_ke = 0.5 * speed * speed;
    let final_energy = final_pe + final_ke;
    
    // Energy should be approximately conserved (within 1%)
    let energy_ratio = final_energy / initial_energy;
    assert!(energy_ratio > 0.99 && energy_ratio < 1.01,
        "Energy not conserved: initial={}, final={}, ratio={}",
        initial_energy, final_energy, energy_ratio);
}

#[test]
fn test_projectile_motion() {
    let mut world = World::new(300.0, 150.0);
    
    // Launch at 45 degrees
    let mut ball = Circle::new(
        Vec2::new(10.0, 10.0),
        Scalar::from_float(2.0),
        Scalar::from_float(0.5),
    );
    
    let launch_speed = 30.0;
    let angle_rad = 45.0 * std::f32::consts::PI / 180.0;
    let vx = launch_speed * angle_rad.cos();
    let vy = launch_speed * angle_rad.sin();
    
    ball.set_velocity(Vec2::new(vx, vy), world.timestep);
    world.add_circle(ball);
    
    let mut max_height = 10.0;
    let mut range = 0.0;
    
    // Simulate until ball lands
    for _ in 0..300 {
        world.step();
        
        let y = world.circles[0].position.y.to_float();
        let x = world.circles[0].position.x.to_float();
        
        if y > max_height {
            max_height = y;
        }
        
        // Check if landed
        if y <= world.circles[0].radius.to_float() + 0.1 {
            range = x - 10.0;
            break;
        }
    }
    
    // Compare with theoretical values (allowing for discretization error)
    let g = 9.81;
    let theoretical_max_height = (launch_speed * launch_speed * angle_rad.sin().powi(2)) / (2.0 * g);
    let _theoretical_range = (launch_speed * launch_speed * (2.0 * angle_rad).sin()) / g;
    
    assert!((max_height - 10.0 - theoretical_max_height).abs() / theoretical_max_height < 0.1,
        "Max height error too large: actual={}, theoretical={}", 
        max_height - 10.0, theoretical_max_height);
    
    assert!(range > 0.0, "Projectile should have traveled some distance");
}

#[test]
fn test_multiple_balls_independence() {
    let mut world = World::new(200.0, 100.0);
    
    // Add balls far apart so they don't interact
    let positions = [(30.0, 90.0), (60.0, 85.0), (90.0, 95.0), (120.0, 80.0)];
    
    for &(x, y) in &positions {
        let ball = Circle::new(
            Vec2::new(x, y),
            Scalar::from_float(5.0),
            Scalar::from_float(1.0),
        );
        world.add_circle(ball);
    }
    
    // Record initial positions
    let initial_positions: Vec<_> = world.circles.iter()
        .map(|c| c.position)
        .collect();
    
    // Simulate
    for _ in 0..60 {
        world.step();
    }
    
    // All balls should have fallen by similar amounts (same mass, same gravity)
    let fall_distances: Vec<f32> = world.circles.iter()
        .zip(&initial_positions)
        .map(|(circle, &initial)| (initial.y - circle.position.y).to_float())
        .collect();
    
    // Check that all fall distances are similar (within 0.1)
    for i in 1..fall_distances.len() {
        assert!((fall_distances[i] - fall_distances[0]).abs() < 0.1,
            "Balls fell by different amounts: {} vs {}", 
            fall_distances[0], fall_distances[i]);
    }
}

#[test]
fn test_velocity_preservation_horizontal() {
    let mut world = World::new(200.0, 100.0);
    
    // Create ball with horizontal velocity
    let mut ball = Circle::new(
        Vec2::new(50.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    ball.set_velocity(Vec2::new(20.0, 0.0), world.timestep);
    world.add_circle(ball);
    
    // Run for a few steps
    for _ in 0..10 {
        world.step();
    }
    
    // Horizontal velocity should be preserved (no horizontal forces)
    let velocity = world.circles[0].velocity;
    assert!((velocity.x.to_float() - 20.0).abs() < 0.1,
        "Horizontal velocity not preserved: {}", velocity.x.to_float());
}

#[test]
fn test_boundary_stop() {
    let mut world = World::new(100.0, 100.0);
    
    // Create ball near ground
    let ball = Circle::new(
        Vec2::new(50.0, 10.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(ball);
    
    // Simulate for many steps
    for _ in 0..200 {
        world.step();
    }
    
    // Ball should be at ground level (y = radius) - test bit-exact equality
    assert_eq!(
        world.circles[0].position.y,
        world.circles[0].radius,
        "Ball should be exactly at ground level"
    );
    
    // Velocity should be near zero
    let velocity = world.circles[0].velocity;
    let speed = velocity.magnitude().to_float();
    assert!(speed < 0.1, "Ball still moving: speed={}", speed);
}

#[test]
fn test_fixed_point_precision() {
    // Test that fixed-point arithmetic maintains precision
    let a = Scalar::from_float(1.0 / 3.0);
    let b = Scalar::from_float(3.0);
    let c = a * b;
    
    // Should be very close to 1.0
    let result = c.to_float();
    assert!((result - 1.0).abs() < 0.0001,
        "Fixed-point precision error: {} * {} = {}", 
        a.to_float(), b.to_float(), result);
    
    // Test that operations are deterministic
    let sum1 = a + a + a;
    let sum2 = a * Scalar::from_float(3.0);
    assert_eq!(sum1.to_bits(), sum2.to_bits(),
        "Different approaches should yield identical results");
}