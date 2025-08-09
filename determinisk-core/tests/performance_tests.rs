//! Performance regression tests

use determinisk_core::{Scalar, Vec2, Circle, World};
use std::time::{Duration, Instant};

/// Helper to measure performance
fn measure_performance<F>(mut f: F, iterations: usize) -> Duration
where
    F: FnMut(),
{
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    start.elapsed()
}

#[test]
fn test_performance_single_ball() {
    let mut world = World::new(100.0, 100.0);
    let ball = Circle::new(
        Vec2::new(50.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(ball);
    
    let duration = measure_performance(|| world.step(), 10000);
    let steps_per_second = 10000.0 / duration.as_secs_f64();
    
    // Should handle at least 1 million steps per second for single ball
    assert!(
        steps_per_second > 1_000_000.0,
        "Performance too low: {:.0} steps/sec (expected > 1M)",
        steps_per_second
    );
}

#[test]
fn test_performance_scaling() {
    let circle_counts = [10, 50, 100];
    let mut performances = Vec::new();
    
    for &count in &circle_counts {
        let mut world = World::new(1000.0, 1000.0);
        
        // Add circles
        for i in 0..count {
            let x = (i % 20) as f32 * 40.0 + 50.0;
            let y = (i / 20) as f32 * 40.0 + 50.0;
            
            let circle = Circle::new(
                Vec2::new(x, y),
                Scalar::from_float(5.0),
                Scalar::from_float(1.0),
            );
            world.add_circle(circle);
        }
        
        // Warm up
        for _ in 0..100 {
            world.step();
        }
        
        // Measure
        let duration = measure_performance(|| world.step(), 1000);
        let steps_per_second = 1000.0 / duration.as_secs_f64();
        performances.push((count, steps_per_second));
    }
    
    // Check that performance scales reasonably
    // Without collision detection, should be roughly linear
    for i in 1..performances.len() {
        let (count1, perf1) = performances[i - 1];
        let (count2, perf2) = performances[i];
        
        let expected_ratio = count2 as f64 / count1 as f64;  // More circles = slower
        let actual_ratio = perf1 / perf2;
        
        // Allow 100% deviation from linear scaling (performance can vary)
        assert!(
            actual_ratio > expected_ratio * 0.5 && actual_ratio < expected_ratio * 2.0,
            "Poor scaling: {}→{} circles, perf {:.0}→{:.0} (ratio {:.2}, expected {:.2})",
            count1, count2, perf1, perf2, actual_ratio, expected_ratio
        );
    }
}

#[test]
fn test_fixed_point_arithmetic_performance() {
    // Compare fixed-point operations performance
    let a = Scalar::from_float(1.234);
    let b = Scalar::from_float(5.678);
    
    // Measure basic operations
    let add_duration = measure_performance(|| {
        let _ = a + b;
    }, 1_000_000);
    
    let mul_duration = measure_performance(|| {
        let _ = a * b;
    }, 1_000_000);
    
    let sqrt_duration = measure_performance(|| {
        let _ = a.sqrt();
    }, 100_000);
    
    // Should complete in reasonable time
    assert!(add_duration.as_millis() < 100, "Addition too slow: {:?}", add_duration);
    assert!(mul_duration.as_millis() < 100, "Multiplication too slow: {:?}", mul_duration);
    assert!(sqrt_duration.as_millis() < 1000, "Square root too slow: {:?}", sqrt_duration);
}

#[test]
fn test_vector_operations_performance() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    
    let dot_duration = measure_performance(|| {
        let _ = v1.dot(&v2);
    }, 1_000_000);
    
    let normalize_duration = measure_performance(|| {
        let _ = v1.normalized();
    }, 100_000);
    
    assert!(dot_duration.as_millis() < 100, "Dot product too slow: {:?}", dot_duration);
    assert!(normalize_duration.as_millis() < 1000, "Normalization too slow: {:?}", normalize_duration);
}

#[test]
#[ignore] // Run with --ignored for detailed benchmarks
fn test_performance_detailed_benchmark() {
    println!("\nDetailed Performance Benchmark");
    println!("==============================");
    
    let configurations = [
        (10, 1000, "Small simulation"),
        (100, 1000, "Medium simulation"),
        (500, 100, "Large simulation"),
    ];
    
    for (circle_count, steps, description) in &configurations {
        let mut world = World::new(2000.0, 2000.0);
        
        // Create circles with random velocities
        for i in 0..*circle_count {
            let x = (i % 40) as f32 * 40.0 + 50.0;
            let y = (i / 40) as f32 * 40.0 + 50.0;
            
            let mut circle = Circle::new(
                Vec2::new(x, y),
                Scalar::from_float(5.0),
                Scalar::from_float(1.0),
            );
            
            let vx = ((i * 7) % 20) as f32 - 10.0;
            let vy = ((i * 13) % 20) as f32 - 10.0;
            circle.set_velocity(Vec2::new(vx, vy), world.timestep);
            
            world.add_circle(circle);
        }
        
        // Warm up
        for _ in 0..50 {
            world.step();
        }
        
        // Benchmark
        let start = Instant::now();
        for _ in 0..*steps {
            world.step();
        }
        let duration = start.elapsed();
        
        let total_operations = circle_count * steps;
        let ops_per_second = total_operations as f64 / duration.as_secs_f64();
        let ms_per_frame = duration.as_millis() as f64 / *steps as f64;
        
        println!("\n{}: {} circles, {} steps", description, circle_count, steps);
        println!("  Total time: {:.2}ms", duration.as_millis());
        println!("  Time per frame: {:.3}ms", ms_per_frame);
        println!("  Circle operations/sec: {:.0}", ops_per_second);
        println!("  Can handle {} circles at 60 FPS", (16.67 / ms_per_frame * *circle_count as f64) as usize);
    }
}

#[test]
fn test_memory_usage_stability() {
    // Ensure memory usage doesn't grow over time
    let mut world = World::new(200.0, 200.0);
    
    // Add fixed number of circles
    for i in 0..20 {
        let circle = Circle::new(
            Vec2::new(50.0 + i as f32 * 5.0, 100.0),
            Scalar::from_float(3.0),
            Scalar::from_float(1.0),
        );
        world.add_circle(circle);
    }
    
    let initial_circle_count = world.circles.len();
    let initial_capacity = world.circles.capacity();
    
    // Run for many steps
    for _ in 0..10000 {
        world.step();
    }
    
    // Circle count and capacity should remain constant
    assert_eq!(world.circles.len(), initial_circle_count);
    assert_eq!(world.circles.capacity(), initial_capacity);
}