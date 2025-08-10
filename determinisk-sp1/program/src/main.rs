//! SP1 guest program for deterministic physics simulation

#![no_main]
sp1_zkvm::entrypoint!(main);

use determinisk_core::{World, Circle, Vec2, Scalar};

pub fn main() {
    // Read simulation parameters from SP1 input
    let num_steps: u32 = sp1_zkvm::io::read();
    let initial_x: i32 = sp1_zkvm::io::read();
    let initial_y: i32 = sp1_zkvm::io::read();
    let velocity_x: i32 = sp1_zkvm::io::read();
    let velocity_y: i32 = sp1_zkvm::io::read();

    // Create world with gravity
    let mut world = World::new(200.0, 200.0);
    
    // Create a ball with given initial conditions
    let mut ball = Circle::new(
        Vec2::new(
            Scalar::from_bits(initial_x).to_float(),
            Scalar::from_bits(initial_y).to_float(),
        ),
        Scalar::from_float(3.0),
        Scalar::from_float(1.0),
    );
    
    // Set initial velocity
    ball.set_velocity(
        Vec2::new(
            Scalar::from_bits(velocity_x).to_float(),
            Scalar::from_bits(velocity_y).to_float(),
        ),
        world.timestep,
    );
    
    world.add_circle(ball);
    
    // Run simulation
    for _ in 0..num_steps {
        world.step();
    }
    
    // Commit final state to public output
    let final_pos = world.circles[0].position;
    let final_vel = world.circles[0].velocity(world.timestep);
    
    sp1_zkvm::io::commit(&final_pos.x.to_bits());
    sp1_zkvm::io::commit(&final_pos.y.to_bits());
    sp1_zkvm::io::commit(&final_vel.x.to_bits());
    sp1_zkvm::io::commit(&final_vel.y.to_bits());
    sp1_zkvm::io::commit(&num_steps);
}