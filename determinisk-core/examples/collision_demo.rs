//! Collision detection and response demonstration

use determinisk_core::{World, Circle, Vec2, Scalar};

fn main() {
    // Create world
    let mut world = World::new(20.0, 20.0);
    
    // Add two circles that will collide
    let circle1 = Circle::new(
        Vec2::new(5.0, 10.0),
        Scalar::from_float(1.0),
        Scalar::ONE,
    );
    world.add_circle(circle1);
    
    let mut circle2 = Circle::new(
        Vec2::new(15.0, 10.0),
        Scalar::from_float(1.0),
        Scalar::ONE,
    );
    // Set initial velocity towards circle1
    circle2.set_velocity(Vec2::new(-5.0, 0.0), world.timestep);
    world.add_circle(circle2);
    
    // Add a third circle falling from above
    let mut circle3 = Circle::new(
        Vec2::new(10.0, 18.0),
        Scalar::from_float(0.8),
        Scalar::from_float(0.5),
    );
    circle3.set_velocity(Vec2::new(0.0, -2.0), world.timestep);
    world.add_circle(circle3);
    
    println!("Collision Demo - 3 circles");
    println!("===========================");
    println!("Circle 1: Stationary at (5, 10)");
    println!("Circle 2: Moving left from (15, 10)");
    println!("Circle 3: Falling from (10, 18)");
    println!();
    
    // Simulate for 200 steps
    for step in 0..200 {
        world.step();
        
        // Print state every 20 steps
        if step % 20 == 0 {
            println!("Step {}:", step);
            for (i, circle) in world.circles.iter().enumerate() {
                println!("  Circle {}: pos={}, vel={}",
                    i + 1,
                    circle.position,
                    circle.velocity,
                );
            }
            
            // Check for collisions
            let grid = determinisk_core::SpatialGrid::build(
                &world.circles,
                Scalar::from_float(4.0),
                world.bounds.x,
                world.bounds.y,
            );
            let pairs = grid.get_collision_pairs();
            let collisions = determinisk_core::spatial::detect_collisions(&world.circles, &pairs);
            
            if !collisions.is_empty() {
                println!("  Collisions detected:");
                for collision in &collisions {
                    println!("    Circle {} <-> Circle {}, depth={}",
                        collision.idx_a + 1,
                        collision.idx_b + 1,
                        collision.depth,
                    );
                }
            }
            println!();
        }
    }
    
    println!("Final state:");
    for (i, circle) in world.circles.iter().enumerate() {
        println!("Circle {}: pos={}, vel={}",
            i + 1,
            circle.position,
            circle.velocity,
        );
    }
}