//! Simple performance benchmark

use determinisk_core::{Scalar, Vec2, Circle, World};
use std::time::Instant;

fn main() {
    println!("Performance Benchmark\n");
    
    let circle_counts = [10, 50, 100, 200, 500];
    let steps_per_test = 1000;
    
    println!("Circles | Steps | Time (ms) | Steps/sec | Circles×Steps/sec");
    println!("--------|-------|-----------|-----------|------------------");
    
    for &num_circles in &circle_counts {
        // Create world with many circles
        let mut world = World::new(1000.0, 1000.0);
        
        for i in 0..num_circles {
            let x = (i % 20) as f32 * 40.0 + 50.0;
            let y = (i / 20) as f32 * 40.0 + 50.0;
            
            let mut circle = Circle::new(
                Vec2::new(x, y),
                Scalar::from_float(5.0),
                Scalar::from_float(1.0),
            );
            
            // Random velocities
            let vx = ((i * 7) % 20) as f32 - 10.0;
            let vy = ((i * 13) % 20) as f32 - 10.0;
            circle.set_velocity(Vec2::new(vx, vy), world.timestep);
            
            world.add_circle(circle);
        }
        
        // Warm up
        for _ in 0..100 {
            world.step();
        }
        
        // Benchmark
        let start = Instant::now();
        for _ in 0..steps_per_test {
            world.step();
        }
        let duration = start.elapsed();
        
        let ms = duration.as_secs_f64() * 1000.0;
        let steps_per_sec = steps_per_test as f64 / duration.as_secs_f64();
        let operations_per_sec = (num_circles * steps_per_test) as f64 / duration.as_secs_f64();
        
        println!("{:7} | {:5} | {:9.2} | {:9.0} | {:16.0}", 
            num_circles, 
            steps_per_test, 
            ms,
            steps_per_sec,
            operations_per_sec
        );
    }
    
    println!("\nNote: This is without collision detection.");
    println!("Performance will decrease significantly with collisions enabled.");
}