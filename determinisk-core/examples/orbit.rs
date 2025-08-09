//! Orbital mechanics simulation - circular orbit using position correction

use determinisk_core::{Scalar, Vec2, Circle, World};
use std::f32::consts::PI;

fn main() {
    // Create a world with no gravity (we'll apply central force)
    let mut world = World::new(400.0, 400.0);
    world.gravity = Vec2::ZERO; // Disable global gravity
    
    // Central "sun" position
    let center = Vec2::new(200.0, 200.0);
    let orbital_radius = 100.0;
    
    // Create orbiting bodies at different positions
    let orbit_configs = [
        (0.0,   1.0, 5.0),    // angle, relative_speed, radius
        (PI/2.0, 1.0, 4.0),   // 90 degrees
        (PI,     1.0, 6.0),   // 180 degrees
        (3.0*PI/2.0, 1.0, 3.0), // 270 degrees
        (PI/4.0, 0.7, 8.0),   // Elliptical orbit (slower)
        (PI/4.0, 1.3, 7.0),   // Elliptical orbit (faster)
    ];
    
    println!("Simulating orbital mechanics...");
    println!("Creating {} orbiting bodies around central point", orbit_configs.len());
    
    for (angle, speed_factor, radius) in orbit_configs.iter() {
        // Position on circle
        let x = center.x.to_float() + orbital_radius * angle.cos();
        let y = center.y.to_float() + orbital_radius * angle.sin();
        
        let mut body = Circle::new(
            Vec2::new(x, y),
            Scalar::from_float(*radius),
            Scalar::from_float(1.0),
        );
        
        // Orbital velocity (perpendicular to radius)
        // v = sqrt(GM/r) for circular orbit, GM = 500, r = 100
        let orbital_speed = (500.0_f32 / orbital_radius).sqrt() * speed_factor;
        let vx = -orbital_speed * angle.sin();
        let vy = orbital_speed * angle.cos();
        
        body.set_velocity(Vec2::new(vx, vy), world.timestep);
        world.add_circle(body);
    }
    
    // Track orbital parameters
    let mut initial_distances = Vec::new();
    for circle in &world.circles {
        let dist = (circle.position - center).magnitude().to_float();
        initial_distances.push(dist);
    }
    
    println!("\nTime  | Body 1 | Body 2 | Body 3 | Body 4 | Body 5 | Body 6 | Energy");
    println!("------|--------|--------|--------|--------|--------|--------|--------");
    
    // Simulate for 10 seconds
    for step in 0..600 {
        // Apply central force (gravity towards center)
        for circle in &mut world.circles {
            let to_center = center - circle.position;
            let distance = to_center.magnitude();
            
            if distance > Scalar::ZERO {
                // F = GMm/r² pointing towards center
                // We'll use a simple model where GM = 500 (reduced to prevent overflow)
                let force_magnitude = Scalar::from_float(500.0) / (distance * distance);
                let force = to_center.normalized() * force_magnitude * circle.mass;
                
                // Apply force using Verlet (modifying old_position)
                let acceleration = force / circle.mass;
                let dt2 = world.timestep * world.timestep;
                
                // Store current position
                let current = circle.position;
                
                // Verlet with central force
                circle.position = current * Scalar::TWO - circle.old_position 
                    + acceleration * dt2;
                
                circle.old_position = current;
            }
        }
        
        // Print status every 60 steps (1 second)
        if step % 60 == 0 {
            let time = step as f32 / 60.0;
            print!("{:5.1} ", time);
            
            let mut total_energy = 0.0;
            
            for (i, circle) in world.circles.iter().enumerate() {
                let dist = (circle.position - center).magnitude().to_float();
                let deviation = ((dist - initial_distances[i]) / initial_distances[i] * 100.0).abs();
                print!("| {:5.1}% ", deviation);
                
                // Calculate orbital energy
                let vel = circle.velocity(world.timestep);
                let speed = vel.magnitude().to_float();
                let ke = 0.5 * circle.mass.to_float() * speed * speed;
                let pe = -500.0 * circle.mass.to_float() / dist; // Gravitational PE
                total_energy += ke + pe;
            }
            
            println!("| {:6.1}", total_energy);
        }
    }
    
    println!("\nNotes:");
    println!("- Percentages show deviation from initial orbital radius");
    println!("- Perfect circular orbits should maintain ~0% deviation");
    println!("- Elliptical orbits (bodies 5 & 6) show larger variations");
    println!("- Total energy should remain approximately constant");
    
    // Final statistics
    println!("\nFinal orbital characteristics:");
    for (i, circle) in world.circles.iter().enumerate() {
        let dist = (circle.position - center).magnitude().to_float();
        let vel = circle.velocity(world.timestep);
        let speed = vel.magnitude().to_float();
        
        // Estimate orbital period (T = 2πr/v for circular orbit)
        let period = 2.0 * PI * dist / speed;
        
        println!("Body {}: r = {:.1}, v = {:.1}, T ≈ {:.1}s", 
            i + 1, dist, speed, period);
    }
}