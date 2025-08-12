//! Collision response using impulse-based resolution
//! 
//! Inspired by JAX MD's approach of deriving forces from potentials,
//! but adapted for discrete impulse-based collision response.

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use crate::math::{Scalar, Vec2};
use crate::physics::Circle;
use crate::spatial::{Collision, BoundaryCollision, Boundary};

/// Collision response configuration
#[derive(Debug, Clone)]
pub struct CollisionConfig {
    /// Coefficient of restitution (0 = perfectly inelastic, 1 = perfectly elastic)
    pub restitution: Scalar,
    /// Position correction factor (0.2-0.8 typical)
    pub position_correction: Scalar,
    /// Minimum separation velocity to apply restitution
    pub velocity_threshold: Scalar,
}

impl Default for CollisionConfig {
    fn default() -> Self {
        Self {
            restitution: Scalar::from_float(0.8),          // 80% elastic
            position_correction: Scalar::from_float(0.4),   // 40% position correction
            velocity_threshold: Scalar::from_float(0.01),   // Minimum velocity for bounce
        }
    }
}

/// Impulse to apply to a circle
#[derive(Debug, Clone)]
pub struct Impulse {
    /// Circle index
    pub idx: usize,
    /// Velocity change
    pub delta_v: Vec2,
    /// Position correction
    pub delta_pos: Vec2,
}

/// Resolve circle-circle collisions using impulse method
/// Returns impulses to apply to circles (functional approach)
pub fn resolve_collisions(
    circles: &[Circle],
    collisions: &[Collision],
    config: &CollisionConfig,
) -> Vec<Impulse> {
    let mut impulses = Vec::new();
    
    for collision in collisions {
        let circle_a = &circles[collision.idx_a];
        let circle_b = &circles[collision.idx_b];
        
        // Calculate relative velocity
        let relative_velocity = circle_b.velocity - circle_a.velocity;
        let velocity_along_normal = relative_velocity.dot(&collision.normal);
        
        // Don't resolve if velocities are separating
        if velocity_along_normal > Scalar::ZERO {
            continue;
        }
        
        // Calculate restitution based on velocity
        let e = if velocity_along_normal.abs() > config.velocity_threshold {
            config.restitution
        } else {
            Scalar::ZERO // No bounce for very slow collisions
        };
        
        // Calculate impulse scalar
        let mass_a = circle_a.mass;
        let mass_b = circle_b.mass;
        let impulse_scalar = -(Scalar::ONE + e) * velocity_along_normal 
            / (Scalar::ONE / mass_a + Scalar::ONE / mass_b);
        
        // Calculate impulse vector
        let impulse = collision.normal * impulse_scalar;
        
        // Apply to velocities (using inverse mass)
        let delta_v_a = -impulse / mass_a;
        let delta_v_b = impulse / mass_b;
        
        // Position correction to resolve overlap
        let total_correction = collision.depth * config.position_correction;
        let mass_sum = mass_a + mass_b;
        let correction_a = collision.normal * (total_correction * mass_b / mass_sum);
        let correction_b = -collision.normal * (total_correction * mass_a / mass_sum);
        
        impulses.push(Impulse {
            idx: collision.idx_a,
            delta_v: delta_v_a,
            delta_pos: -correction_a,
        });
        
        impulses.push(Impulse {
            idx: collision.idx_b,
            delta_v: delta_v_b,
            delta_pos: -correction_b,
        });
    }
    
    impulses
}

/// Resolve boundary collisions
pub fn resolve_boundary_collisions(
    circles: &[Circle],
    collisions: &[BoundaryCollision],
    config: &CollisionConfig,
) -> Vec<Impulse> {
    let mut impulses = Vec::new();
    
    for collision in collisions {
        let circle = &circles[collision.idx];
        
        // Determine normal based on boundary
        let normal = match collision.boundary {
            Boundary::Left => Vec2::from_scalars(Scalar::ONE, Scalar::ZERO),
            Boundary::Right => Vec2::from_scalars(-Scalar::ONE, Scalar::ZERO),
            Boundary::Bottom => Vec2::from_scalars(Scalar::ZERO, Scalar::ONE),
            Boundary::Top => Vec2::from_scalars(Scalar::ZERO, -Scalar::ONE),
        };
        
        // Calculate velocity along normal
        let velocity_along_normal = circle.velocity.dot(&normal);
        
        // Don't resolve if velocity is away from boundary
        if velocity_along_normal > Scalar::ZERO {
            continue;
        }
        
        // Apply restitution
        let e = if velocity_along_normal.abs() > config.velocity_threshold {
            config.restitution
        } else {
            Scalar::ZERO
        };
        
        // Calculate impulse (boundary has infinite mass)
        let impulse_scalar = -(Scalar::ONE + e) * velocity_along_normal;
        let impulse = normal * impulse_scalar;
        
        // Velocity change
        let delta_v = impulse / circle.mass;
        
        // Position correction to push circle back inside bounds
        let delta_pos = normal * (collision.depth * config.position_correction);
        
        impulses.push(Impulse {
            idx: collision.idx,
            delta_v,
            delta_pos,
        });
    }
    
    impulses
}

/// Apply impulses to circles (functional update)
/// Returns new circle states after applying impulses
pub fn apply_impulses(circles: &[Circle], impulses: &[Impulse]) -> Vec<Circle> {
    // Create a map of accumulated impulses per circle
    let mut impulse_map: Vec<(Vec2, Vec2)> = vec![(Vec2::ZERO, Vec2::ZERO); circles.len()];
    
    // Accumulate impulses for each circle
    for impulse in impulses {
        impulse_map[impulse.idx].0 += impulse.delta_v;
        impulse_map[impulse.idx].1 += impulse.delta_pos;
    }
    
    // Apply accumulated impulses to create new circle states
    circles.iter().enumerate().map(|(idx, circle)| {
        let (delta_v, delta_pos) = impulse_map[idx];
        Circle {
            position: circle.position + delta_pos,
            old_position: circle.old_position, // Keep old position for Verlet
            velocity: circle.velocity + delta_v,
            ..*circle
        }
    }).collect()
}

/// Complete collision resolution pipeline (functional)
/// Takes circles and returns updated circles after collision resolution
pub fn resolve_all_collisions(
    circles: &[Circle],
    world_width: Scalar,
    world_height: Scalar,
    config: &CollisionConfig,
) -> Vec<Circle> {
    use crate::spatial::{SpatialGrid, detect_collisions, detect_boundary_collisions};
    
    // Build spatial grid (cell size = 2 * max radius)
    let max_radius = circles.iter()
        .map(|c| c.radius)
        .max()
        .unwrap_or(Scalar::from_float(1.0));
    let cell_size = max_radius * Scalar::from_float(2.0);
    
    let grid = SpatialGrid::build(circles, cell_size, world_width, world_height);
    
    // Get potential collision pairs from spatial grid
    let pairs = grid.get_collision_pairs();
    
    // Detect actual collisions
    let circle_collisions = detect_collisions(circles, &pairs);
    let boundary_collisions = detect_boundary_collisions(circles, world_width, world_height);
    
    // Resolve collisions to get impulses
    let mut all_impulses = resolve_collisions(circles, &circle_collisions, config);
    let boundary_impulses = resolve_boundary_collisions(circles, &boundary_collisions, config);
    all_impulses.extend(boundary_impulses);
    
    // Apply impulses to circles
    apply_impulses(circles, &all_impulses)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_head_on_collision() {
        // Two circles moving towards each other
        let circles = vec![
            Circle::new(
                Vec2::new(10.0, 10.0),
                Scalar::from_float(1.0),
                Scalar::ONE,
            ),
            Circle::new(
                Vec2::new(12.0, 10.0),
                Scalar::from_float(1.0),
                Scalar::ONE,
            ),
        ];
        
        // Set velocities
        let mut circles = circles;
        circles[0].velocity = Vec2::new(1.0, 0.0);
        circles[0].old_position = Vec2::new(9.0, 10.0);
        circles[1].velocity = Vec2::new(-1.0, 0.0);
        circles[1].old_position = Vec2::new(13.0, 10.0);
        
        let collision = Collision {
            idx_a: 0,
            idx_b: 1,
            normal: Vec2::new(1.0, 0.0),
            depth: Scalar::from_float(0.0), // Just touching
            contact: Vec2::new(11.0, 10.0),
        };
        
        let config = CollisionConfig::default();
        let impulses = resolve_collisions(&circles, &[collision], &config);
        
        // With equal masses and opposite velocities, they should exchange velocities
        assert_eq!(impulses.len(), 2);
        
        // Apply impulses and check velocities reversed (with restitution factor)
        let new_circles = apply_impulses(&circles, &impulses);
        
        // Velocities should be reversed and scaled by restitution
        let expected_v = Scalar::from_float(0.8); // restitution * 1.0
        assert!(new_circles[0].velocity.x < Scalar::ZERO); // Moving left now
        assert!(new_circles[1].velocity.x > Scalar::ZERO); // Moving right now
    }
    
    #[test]
    fn test_boundary_bounce() {
        // Circle hitting bottom boundary
        let mut circles = vec![
            Circle::new(
                Vec2::new(10.0, 0.5),
                Scalar::from_float(1.0),
                Scalar::ONE,
            ),
        ];
        circles[0].velocity = Vec2::new(0.0, -1.0);
        circles[0].old_position = Vec2::new(10.0, 1.5);
        
        let collision = BoundaryCollision {
            idx: 0,
            boundary: Boundary::Bottom,
            depth: Scalar::from_float(0.5),
            contact: Vec2::new(10.0, 0.0),
        };
        
        let config = CollisionConfig::default();
        let impulses = resolve_boundary_collisions(&circles, &[collision], &config);
        
        assert_eq!(impulses.len(), 1);
        
        let new_circles = apply_impulses(&circles, &impulses);
        
        // Velocity should be reversed and scaled by restitution
        assert!(new_circles[0].velocity.y > Scalar::ZERO); // Bouncing up
        
        // Position should be corrected
        assert!(new_circles[0].position.y > circles[0].position.y);
    }
}