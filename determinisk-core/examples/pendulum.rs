//! Pendulum simulation using position constraints

use determinisk_core::{Scalar, Vec2, Circle, World};
use std::f32::consts::PI;

fn main() {
    // Create a world
    let mut world = World::new(200.0, 200.0);
    
    // Pendulum parameters
    let pivot = Vec2::new(100.0, 150.0);
    let length = 50.0;
    let initial_angle = PI / 4.0; // 45 degrees
    
    // Create pendulum bob
    let bob_x = pivot.x.to_float() + length * initial_angle.sin();
    let bob_y = pivot.y.to_float() - length * initial_angle.cos();
    
    let mut bob = Circle::new(
        Vec2::new(bob_x, bob_y),
        Scalar::from_float(5.0),  // radius
        Scalar::from_float(1.0),  // mass
    );
    
    // Give it a small initial velocity (perpendicular to rod)
    let initial_vel = Vec2::new(0.0, 0.0);
    bob.set_velocity(initial_vel, world.timestep);
    
    world.add_circle(bob);
    
    println!("Simulating pendulum...");
    println!("Time  | Angle (deg) | Angular Vel | Energy");
    println!("------|-------------|-------------|--------");
    
    let mut prev_angle = initial_angle;
    
    // Simulate for 5 seconds
    for step in 0..300 {
        // Apply constraint after physics step
        world.step();
        
        // Enforce pendulum length constraint
        let bob = &mut world.circles[0];
        let to_bob = bob.position - pivot;
        let current_length = to_bob.magnitude();
        
        if current_length > Scalar::ZERO {
            // Project position back to correct length
            bob.position = pivot + to_bob * (Scalar::from_float(length) / current_length);
            
            // Adjust old_position to maintain velocity tangent to constraint
            let old_to_pivot = bob.old_position - pivot;
            let old_length = old_to_pivot.magnitude();
            if old_length > Scalar::ZERO {
                bob.old_position = pivot + old_to_pivot * (Scalar::from_float(length) / old_length);
            }
        }
        
        // Calculate and display state every 10 steps
        if step % 10 == 0 {
            let time = step as f32 / 60.0;
            
            // Calculate angle from vertical
            let dx = (bob.position.x - pivot.x).to_float();
            let dy = (bob.position.y - pivot.y).to_float();
            let angle = dx.atan2(-dy);
            let angle_deg = angle * 180.0 / PI;
            
            // Angular velocity (approximate)
            let angular_vel = (angle - prev_angle) * 60.0; // rad/s
            prev_angle = angle;
            
            // Calculate energy (KE + PE)
            let velocity = bob.velocity(world.timestep);
            let speed = velocity.magnitude().to_float();
            let height = (pivot.y - bob.position.y).to_float() + length;
            let ke = 0.5 * speed * speed;
            let pe = 9.81 * height;
            let total_energy = ke + pe;
            
            println!("{:5.1} | {:11.2} | {:11.3} | {:7.3}", 
                time, angle_deg, angular_vel, total_energy);
        }
    }
    
    println!("\nNote: Energy should remain approximately constant");
    println!("(small variations due to constraint enforcement)");
}