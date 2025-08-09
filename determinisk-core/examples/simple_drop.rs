//! Simple example: Drop a ball and watch it fall

use determinisk_core::{Scalar, Vec2, Circle, World};

fn main() {
    // Create a world
    let mut world = World::new(100.0, 100.0);
    
    // Add a ball
    let ball = Circle::new(
        Vec2::new(50.0, 80.0),  // Start position
        Scalar::from_float(5.0), // Radius
        Scalar::from_float(1.0), // Mass
    );
    world.add_circle(ball);
    
    println!("Simulating ball drop...");
    println!("Time | Y Position");
    println!("-----|------------");
    
    // Simulate for 2 seconds (120 steps at 60 Hz)
    for step in 0..120 {
        world.step();
        
        // Print every 10 steps
        if step % 10 == 0 {
            let time = step as f32 / 60.0;
            let y_pos = world.circles[0].position.y.to_float();
            println!("{:4.1} | {:10.3}", time, y_pos);
        }
    }
    
    println!("\nBall reached ground!");
}