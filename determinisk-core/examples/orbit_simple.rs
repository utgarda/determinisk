//! Simple circular orbit demonstration

use determinisk_core::{Scalar, Vec2, Circle, World};

fn main() {
    // Create a world without gravity
    let mut world = World::new(200.0, 200.0);
    world.gravity = Vec2::ZERO;
    
    // Create a body in circular orbit
    let center = Vec2::new(100.0, 100.0);
    let radius = 50.0;
    
    let mut orbiter = Circle::new(
        Vec2::new(150.0, 100.0),  // Start at right side
        Scalar::from_float(3.0),   // Small radius
        Scalar::from_float(1.0),   // Unit mass
    );
    
    // Set tangential velocity for circular orbit
    // For stable orbit: v = sqrt(acceleration * radius)
    // We'll use a simple acceleration that won't overflow
    let orbital_speed = 5.0;
    orbiter.set_velocity(Vec2::new(0.0, orbital_speed), world.timestep);
    
    world.add_circle(orbiter);
    
    println!("Simulating circular orbit...");
    println!("Step | X Pos | Y Pos | Distance | Speed");
    println!("-----|-------|-------|----------|-------");
    
    let mut positions = Vec::new();
    
    // Simulate for 200 steps
    for step in 0..200 {
        // Apply centripetal acceleration
        let to_center = center - world.circles[0].position;
        let distance = to_center.magnitude();
        
        if distance > Scalar::ZERO {
            // Simple centripetal acceleration: a = v²/r
            // But we'll use a constant acceleration to avoid overflow
            let accel_magnitude = Scalar::from_float(1.0);
            let acceleration = to_center.normalized() * accel_magnitude;
            
            // Apply acceleration using position adjustment
            let dt2 = world.timestep * world.timestep;
            world.circles[0].position = world.circles[0].position + acceleration * dt2;
        }
        
        world.step();
        
        // Record position
        let pos = world.circles[0].position;
        positions.push((pos.x.to_float(), pos.y.to_float()));
        
        // Print status every 20 steps
        if step % 20 == 0 {
            let dist = (pos - center).magnitude().to_float();
            let vel = world.circles[0].velocity(world.timestep);
            let speed = vel.magnitude().to_float();
            
            println!("{:4} | {:5.1} | {:5.1} | {:8.2} | {:5.2}", 
                step, 
                pos.x.to_float(), 
                pos.y.to_float(),
                dist,
                speed
            );
        }
    }
    
    // Check if orbit is approximately circular
    let distances: Vec<f32> = positions.iter()
        .map(|(x, y)| {
            let dx = x - 100.0;
            let dy = y - 100.0;
            (dx * dx + dy * dy).sqrt()
        })
        .collect();
    
    let avg_distance = distances.iter().sum::<f32>() / distances.len() as f32;
    let max_deviation = distances.iter()
        .map(|&d| (d - avg_distance).abs())
        .fold(0.0, f32::max);
    
    println!("\nOrbit Analysis:");
    println!("Average radius: {:.2}", avg_distance);
    println!("Maximum deviation: {:.2} ({:.1}%)", 
        max_deviation, 
        max_deviation / avg_distance * 100.0
    );
}