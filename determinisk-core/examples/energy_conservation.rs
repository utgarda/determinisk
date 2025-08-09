//! Demonstrate energy conservation in the physics engine

use determinisk_core::{Scalar, Vec2, Circle, World};

fn calculate_total_energy(world: &World) -> f32 {
    let mut total_energy = 0.0;
    
    for circle in &world.circles {
        // Kinetic energy: 0.5 * m * v^2
        let velocity = circle.velocity(world.timestep);
        let speed_squared = velocity.magnitude_squared().to_float();
        let ke = 0.5 * circle.mass.to_float() * speed_squared;
        
        // Potential energy: m * g * h
        let height = circle.position.y.to_float();
        let pe = circle.mass.to_float() * 9.81 * height;
        
        total_energy += ke + pe;
    }
    
    total_energy
}

fn main() {
    // Create a world with no damping for perfect energy conservation
    let mut world = World::new(100.0, 200.0);
    
    // Create several scenarios
    println!("Testing energy conservation in different scenarios...\n");
    
    // Scenario 1: Free fall
    println!("Scenario 1: Free fall");
    let ball1 = Circle::new(
        Vec2::new(25.0, 150.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    world.add_circle(ball1);
    
    // Scenario 2: Horizontal motion (should maintain height)
    println!("Scenario 2: Horizontal motion");
    let mut ball2 = Circle::new(
        Vec2::new(50.0, 100.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    ball2.set_velocity(Vec2::new(20.0, 0.0), world.timestep);
    world.add_circle(ball2);
    
    // Scenario 3: Diagonal motion
    println!("Scenario 3: Projectile motion");
    let mut ball3 = Circle::new(
        Vec2::new(75.0, 50.0),
        Scalar::from_float(5.0),
        Scalar::from_float(1.0),
    );
    ball3.set_velocity(Vec2::new(15.0, 30.0), world.timestep);
    world.add_circle(ball3);
    
    // Record initial energy
    let initial_energy = calculate_total_energy(&world);
    println!("\nInitial total energy: {:.3} J", initial_energy);
    
    println!("\nTime  | Total Energy | Energy Change | Ball 1 KE/PE | Ball 2 KE/PE | Ball 3 KE/PE");
    println!("------|--------------|---------------|--------------|--------------|-------------");
    
    // Simulate for 3 seconds
    for step in 0..180 {
        world.step();
        
        if step % 20 == 0 {  // Every ~0.33 seconds
            let time = step as f32 / 60.0;
            let total_energy = calculate_total_energy(&world);
            let energy_change = total_energy - initial_energy;
            let change_percent = (energy_change / initial_energy) * 100.0;
            
            print!("{:5.1} | {:12.3} | {:+11.3}% ", time, total_energy, change_percent);
            
            // Show KE/PE for each ball
            for circle in &world.circles {
                let velocity = circle.velocity(world.timestep);
                let speed_squared = velocity.magnitude_squared().to_float();
                let ke = 0.5 * circle.mass.to_float() * speed_squared;
                let pe = circle.mass.to_float() * 9.81 * circle.position.y.to_float();
                print!("| {:5.1}/{:5.1} ", ke, pe);
            }
            println!();
        }
    }
    
    let final_energy = calculate_total_energy(&world);
    let total_change = final_energy - initial_energy;
    let change_percent = (total_change / initial_energy) * 100.0;
    
    println!("\nFinal total energy: {:.3} J", final_energy);
    println!("Total energy change: {:.3} J ({:+.3}%)", total_change, change_percent);
    
    println!("\nNote: Small energy changes are due to:");
    println!("- Ground collision handling (ball stops at y = radius)");
    println!("- Fixed-point arithmetic precision");
    println!("- Discrete time steps in Verlet integration");
}