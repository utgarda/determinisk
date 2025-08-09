//! Multiple balls falling with different properties

use determinisk_core::{Scalar, Vec2, Circle, World};

fn main() {
    // Create a world
    let mut world = World::new(200.0, 100.0);
    
    // Add multiple balls with different properties
    let balls = [
        // (x, y, radius, mass)
        (30.0, 90.0, 3.0, 0.5),   // Small, light ball
        (60.0, 85.0, 5.0, 1.0),   // Medium ball
        (90.0, 95.0, 7.0, 2.0),   // Large, heavy ball
        (120.0, 80.0, 4.0, 0.8),  // Another medium ball
        (150.0, 88.0, 6.0, 1.5),  // Large ball
    ];
    
    for (x, y, radius, mass) in balls.iter() {
        let mut ball = Circle::new(
            Vec2::new(*x, *y),
            Scalar::from_float(*radius),
            Scalar::from_float(*mass),
        );
        
        // Vary properties
        ball.restitution = Scalar::from_float(0.3 + radius / 10.0);
        ball.friction = Scalar::from_float(0.1 + mass / 10.0);
        
        world.add_circle(ball);
    }
    
    println!("Simulating {} balls falling...", world.circles.len());
    println!("Time | Ball 1 Y | Ball 2 Y | Ball 3 Y | Ball 4 Y | Ball 5 Y");
    println!("-----|----------|----------|----------|----------|----------");
    
    // Simulate for 2 seconds
    for step in 0..120 {
        world.step();
        
        // Print every 10 steps
        if step % 10 == 0 {
            let time = step as f32 / 60.0;
            print!("{:4.1} ", time);
            
            for circle in &world.circles {
                print!("| {:8.2} ", circle.position.y.to_float());
            }
            println!();
        }
    }
    
    // Show final positions
    println!("\nFinal positions:");
    for (i, circle) in world.circles.iter().enumerate() {
        println!("Ball {}: y = {:.3}, radius = {:.1}", 
            i + 1, 
            circle.position.y.to_float(),
            circle.radius.to_float()
        );
    }
}