//! Tests for physics accuracy and conservation laws

use determinisk_core::{Scalar, Vec2, Circle, World};

/// Calculate total energy of the system
fn calculate_total_energy(world: &World) -> f32 {
    let mut total = 0.0;
    
    for circle in &world.circles {
        // Kinetic energy: 0.5 * m * v^2
        let velocity = circle.velocity(world.timestep);
        let speed_squared = velocity.magnitude_squared().to_float();
        let ke = 0.5 * circle.mass.to_float() * speed_squared;
        
        // Potential energy: m * g * h
        let height = circle.position.y.to_float();
        let pe = circle.mass.to_float() * 9.81 * height;
        
        total += ke + pe;
    }
    
    total
}

/// Calculate total momentum of the system
fn calculate_total_momentum(world: &World) -> Vec2 {
    let mut total = Vec2::ZERO;
    
    for circle in &world.circles {
        let velocity = circle.velocity(world.timestep);
        let momentum = velocity * circle.mass;
        total = total + momentum;
    }
    
    total
}

#[test]
fn test_energy_conservation_free_fall() {
    let mut world = World::new(100.0, 200.0);
    
    let ball = Circle::new(
        Vec2::new(50.0, 180.0),
        Scalar::from_float(5.0),
        Scalar::from_float(2.0),
    );
    world.add_circle(ball);
    
    let initial_energy = calculate_total_energy(&world);
    
    // Simulate free fall (stop before ground collision)
    for _ in 0..50 {
        world.step();
        
        // Stop if getting close to ground
        if world.circles[0].position.y.to_float() < 20.0 {
            break;
        }
    }
    
    let final_energy = calculate_total_energy(&world);
    let energy_ratio = final_energy / initial_energy;
    
    // Energy should be conserved within 0.1%
    assert!(
        (energy_ratio - 1.0).abs() < 0.001,
        "Energy not conserved: initial={:.3}, final={:.3}, ratio={:.6}",
        initial_energy, final_energy, energy_ratio
    );
}

#[test]
fn test_energy_conservation_multiple_balls() {
    let mut world = World::new(200.0, 300.0);
    
    // Add several balls at different heights
    for i in 0..5 {
        let ball = Circle::new(
            Vec2::new(40.0 + i as f32 * 30.0, 200.0 + i as f32 * 20.0),
            Scalar::from_float(4.0),
            Scalar::from_float(1.0 + i as f32 * 0.5),
        );
        world.add_circle(ball);
    }
    
    let initial_energy = calculate_total_energy(&world);
    
    // Simulate for a short time
    for _ in 0..40 {
        world.step();
    }
    
    let final_energy = calculate_total_energy(&world);
    let energy_change = (final_energy - initial_energy).abs() / initial_energy;
    
    assert!(
        energy_change < 0.01,
        "Energy change too large: {:.2}%",
        energy_change * 100.0
    );
}

#[test]
fn test_momentum_conservation_horizontal() {
    let mut world = World::new(500.0, 100.0);
    world.gravity = Vec2::ZERO; // No external forces
    
    // Add balls with various horizontal velocities
    let velocities = [10.0, -5.0, 15.0, -20.0, 8.0];
    let masses = [1.0, 2.0, 0.5, 1.5, 3.0];
    
    for (i, (&vx, &mass)) in velocities.iter().zip(&masses).enumerate() {
        let mut ball = Circle::new(
            Vec2::new(50.0 + i as f32 * 80.0, 50.0),
            Scalar::from_float(5.0),
            Scalar::from_float(mass),
        );
        ball.set_velocity(Vec2::new(vx, 0.0), world.timestep);
        world.add_circle(ball);
    }
    
    let initial_momentum = calculate_total_momentum(&world);
    
    // Simulate
    for _ in 0..100 {
        world.step();
    }
    
    let final_momentum = calculate_total_momentum(&world);
    
    // Momentum should be conserved (no external forces)
    let momentum_change_x = (final_momentum.x - initial_momentum.x).abs().to_float();
    let momentum_change_y = (final_momentum.y - initial_momentum.y).abs().to_float();
    
    assert!(
        momentum_change_x < 0.01,
        "X momentum not conserved: initial={:.3}, final={:.3}",
        initial_momentum.x.to_float(), final_momentum.x.to_float()
    );
    
    assert!(
        momentum_change_y < 0.01,
        "Y momentum not conserved: initial={:.3}, final={:.3}",
        initial_momentum.y.to_float(), final_momentum.y.to_float()
    );
}

#[test]
fn test_projectile_trajectory() {
    let mut world = World::new(1000.0, 500.0);
    
    let angles = [30.0, 45.0, 60.0];
    let launch_speed = 40.0;
    
    for (i, &angle_deg) in angles.iter().enumerate() {
        let mut ball = Circle::new(
            Vec2::new(50.0, 10.0),
            Scalar::from_float(2.0),
            Scalar::from_float(1.0),
        );
        
        let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
        let vx = launch_speed * angle_rad.cos();
        let vy = launch_speed * angle_rad.sin();
        
        ball.set_velocity(Vec2::new(vx, vy), world.timestep);
        world.add_circle(ball);
        
        // Track trajectory
        let mut max_height = 10.0;
        let mut positions = Vec::new();
        
        // Simulate until ball returns to ground
        for _ in 0..500 {
            world.step();
            
            let pos = world.circles[i].position;
            let y = pos.y.to_float();
            
            positions.push((pos.x.to_float(), y));
            
            if y > max_height {
                max_height = y;
            }
            
            // Stop if returned to ground
            if positions.len() > 10 && y <= 10.1 {
                break;
            }
        }
        
        // Verify parabolic trajectory
        // At the peak, vertical velocity should be near zero
        let peak_idx = positions.iter()
            .position(|(_, y)| (*y - max_height).abs() < 0.5)
            .expect("Should find peak");
        
        if peak_idx > 0 && peak_idx < positions.len() - 1 {
            let y_before = positions[peak_idx - 1].1;
            let y_after = positions[peak_idx + 1].1;
            
            // Should be symmetric around peak
            assert!(
                (y_before - y_after).abs() < 2.0,
                "Trajectory not symmetric around peak for angle {}°",
                angle_deg
            );
        }
    }
}

#[test]
fn test_galilean_relativity() {
    // Test that physics is the same in different reference frames
    let mut world1 = World::new(200.0, 100.0);
    let mut world2 = World::new(200.0, 100.0);
    
    // World 1: Ball falling straight down
    let ball1 = Circle::new(
        Vec2::new(50.0, 80.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world1.add_circle(ball1);
    
    // World 2: Ball with horizontal velocity
    let mut ball2 = Circle::new(
        Vec2::new(50.0, 80.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    ball2.set_velocity(Vec2::new(20.0, 0.0), world2.timestep);
    world2.add_circle(ball2);
    
    // Simulate both
    for _ in 0..50 {
        world1.step();
        world2.step();
    }
    
    // Vertical motion should be identical
    let y1 = world1.circles[0].position.y.to_float();
    let y2 = world2.circles[0].position.y.to_float();
    
    assert!(
        (y1 - y2).abs() < 0.01,
        "Vertical motion differs: stationary={:.3}, moving={:.3}",
        y1, y2
    );
    
    // Horizontal velocity should be preserved in world2
    let vx2 = world2.circles[0].velocity(world2.timestep).x.to_float();
    assert!(
        (vx2 - 20.0).abs() < 0.1,
        "Horizontal velocity not preserved: {:.3}",
        vx2
    );
}


#[test]
fn test_pendulum_period() {
    // Simple pendulum approximation using constraint
    let mut world = World::new(200.0, 200.0);
    
    let pivot = Vec2::new(100.0, 150.0);
    let length = 50.0;
    let initial_angle: f32 = 0.1; // Small angle approximation
    
    let bob_x = pivot.x.to_float() + length * initial_angle.sin();
    let bob_y = pivot.y.to_float() - length * initial_angle.cos();
    
    let bob = Circle::new(
        Vec2::new(bob_x, bob_y),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(bob);
    
    let mut crossings = Vec::new();
    let mut prev_x = bob_x - 100.0;
    
    // Simulate and detect midpoint crossings
    for step in 0..600 {
        world.step();
        
        // Enforce pendulum constraint
        let bob = &mut world.circles[0];
        let to_bob = bob.position - pivot;
        let current_length = to_bob.magnitude();
        
        if current_length > Scalar::ZERO {
            bob.position = pivot + to_bob * (Scalar::from_float(length) / current_length);
        }
        
        let current_x = bob.position.x.to_float() - 100.0;
        
        // Detect crossing through center
        if prev_x < 0.0 && current_x >= 0.0 {
            crossings.push(step);
        }
        
        prev_x = current_x;
    }
    
    // Calculate periods from crossings
    if crossings.len() >= 3 {
        let period1 = (crossings[2] - crossings[0]) as f32 / 60.0; // Two half-periods
        let theoretical_period = 2.0 * std::f32::consts::PI * (length / 9.81).sqrt();
        
        // Should be within 10% of theoretical (small angle approximation)
        let error = ((period1 - theoretical_period) / theoretical_period).abs();
        assert!(
            error < 0.1,
            "Pendulum period error too large: measured={:.3}s, theoretical={:.3}s",
            period1, theoretical_period
        );
    }
}