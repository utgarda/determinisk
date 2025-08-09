//! Integration tests for deterministic physics

#[cfg(test)]
mod integration_tests {
    use crate::{Scalar, Vec2, Circle, World};
    
    #[test]
    fn test_basic_simulation() {
        let mut world = World::new(100.0, 100.0);
        
        // Add a circle
        let circle = Circle::new(
            Vec2::new(50.0, 50.0),
            Scalar::from_float(5.0),
            Scalar::from_float(1.0),
        );
        world.add_circle(circle);
        
        // Run simulation for 60 steps (1 second at 60 Hz)
        let initial_y = world.circles[0].position.y;
        for _ in 0..60 {
            world.step();
        }
        
        // Circle should have fallen due to gravity
        assert!(world.circles[0].position.y < initial_y);
    }
    
    #[test]
    fn test_determinism() {
        // Create two identical worlds
        let mut world1 = World::new(100.0, 100.0);
        let mut world2 = World::new(100.0, 100.0);
        
        // Add identical circles
        let circle = Circle::new(
            Vec2::new(50.0, 80.0),
            Scalar::from_float(5.0),
            Scalar::from_float(1.0),
        );
        world1.add_circle(circle);
        world2.add_circle(circle);
        
        // Run both simulations
        for _ in 0..100 {
            world1.step();
            world2.step();
        }
        
        // Results should be bit-identical
        assert_eq!(
            world1.circles[0].position.x.to_bits(),
            world2.circles[0].position.x.to_bits()
        );
        assert_eq!(
            world1.circles[0].position.y.to_bits(),
            world2.circles[0].position.y.to_bits()
        );
    }
}