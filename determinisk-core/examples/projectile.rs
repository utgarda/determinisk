//! Projectile motion - balls launched at different angles

use determinisk_core::{Scalar, Vec2, Circle, World};

fn main() {
    // Create a wider world for projectile motion
    let mut world = World::new(300.0, 150.0);
    
    // Launch parameters
    let launch_speed = 30.0;
    let launch_angles = [15.0, 30.0, 45.0, 60.0, 75.0]; // degrees
    
    println!("Launching {} projectiles at different angles...", launch_angles.len());
    
    // Create projectiles with different launch angles
    for (_i, &angle) in launch_angles.iter().enumerate() {
        let mut ball = Circle::new(
            Vec2::new(10.0, 10.0),  // Start near ground
            Scalar::from_float(2.0), // Small radius
            Scalar::from_float(0.5), // Light mass
        );
        
        // Convert angle to radians and set initial velocity
        let angle_rad = angle * std::f32::consts::PI / 180.0;
        let vx = launch_speed * angle_rad.cos();
        let vy = launch_speed * angle_rad.sin();
        
        // Set initial velocity using Verlet method
        ball.set_velocity(Vec2::new(vx, vy), world.timestep);
        
        world.add_circle(ball);
    }
    
    // Track maximum heights and ranges
    let mut max_heights = vec![10.0; launch_angles.len()];
    let mut ranges = vec![0.0; launch_angles.len()];
    let mut in_flight = vec![true; launch_angles.len()];
    
    println!("\nAngle | Max Height | Range");
    println!("------|------------|-------");
    
    // Simulate until all projectiles land
    for _step in 0..600 {  // 10 seconds max
        world.step();
        
        // Update tracking
        for (i, circle) in world.circles.iter().enumerate() {
            let y = circle.position.y.to_float();
            let x = circle.position.x.to_float();
            
            // Track maximum height
            if y > max_heights[i] {
                max_heights[i] = y;
            }
            
            // Check if landed (y ≈ radius)
            if in_flight[i] && y <= circle.radius.to_float() + 0.1 {
                in_flight[i] = false;
                ranges[i] = x - 10.0; // Subtract starting x position
            }
        }
        
        // Check if all have landed
        if !in_flight.iter().any(|&f| f) {
            break;
        }
    }
    
    // Print results
    for (i, &angle) in launch_angles.iter().enumerate() {
        println!("{:5.0}° | {:10.2} | {:6.2}", 
            angle, 
            max_heights[i] - 10.0,  // Subtract starting height
            ranges[i]
        );
    }
    
    // Theoretical comparison (no air resistance)
    println!("\nTheoretical values (no air resistance):");
    println!("Angle | Max Height | Range");
    println!("------|------------|-------");
    
    let g = 9.81;
    for &angle in &launch_angles {
        let angle_rad = angle * std::f32::consts::PI / 180.0;
        let max_h = (launch_speed * launch_speed * angle_rad.sin().powi(2)) / (2.0 * g);
        let range = (launch_speed * launch_speed * (2.0 * angle_rad).sin()) / g;
        println!("{:5.0}° | {:10.2} | {:6.2}", angle, max_h, range);
    }
}