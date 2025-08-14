//! Spatial data structures for efficient collision detection
//! 
//! Inspired by JAX MD's neighbor list approach, but adapted for zkVM constraints
//! using deterministic BTreeMap instead of arrays for sparse grids.

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, collections::BTreeMap};
#[cfg(feature = "std")]
use std::collections::BTreeMap;

use crate::math::{Scalar, Vec2};
use crate::physics::Circle;

/// Spatial grid for broad-phase collision detection
/// Cell size is typically 2x the maximum circle radius
#[derive(Debug, Clone)]
pub struct SpatialGrid {
    /// Grid cells containing circle indices
    /// Using BTreeMap for deterministic iteration order
    cells: BTreeMap<GridCell, Vec<usize>>,
    /// Size of each grid cell
    cell_size: Scalar,
    /// World boundaries for wrapping
    _world_width: Scalar,
    _world_height: Scalar,
}

/// Grid cell coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

impl SpatialGrid {
    /// Create a new spatial grid
    pub fn new(cell_size: Scalar, world_width: Scalar, world_height: Scalar) -> Self {
        Self {
            cells: BTreeMap::new(),
            cell_size,
            _world_width: world_width,
            _world_height: world_height,
        }
    }

    /// Build grid from circle positions (functional update)
    /// This is a pure function - returns new grid without mutation
    pub fn build(circles: &[Circle], cell_size: Scalar, world_width: Scalar, world_height: Scalar) -> Self {
        let mut grid = Self::new(cell_size, world_width, world_height);
        
        for (idx, circle) in circles.iter().enumerate() {
            let cell = grid.position_to_cell(circle.position);
            grid.cells.entry(cell).or_insert_with(Vec::new).push(idx);
            
            // Also add to neighboring cells if circle overlaps boundaries
            // This ensures we don't miss collisions at cell edges
            let radius = circle.radius;
            let neighbors = grid.get_overlapping_cells(circle.position, radius);
            for neighbor_cell in neighbors {
                if neighbor_cell != cell {
                    grid.cells.entry(neighbor_cell).or_insert_with(Vec::new).push(idx);
                }
            }
        }
        
        grid
    }

    /// Convert world position to grid cell
    fn position_to_cell(&self, pos: Vec2) -> GridCell {
        GridCell {
            x: (pos.x / self.cell_size).to_int(),
            y: (pos.y / self.cell_size).to_int(),
        }
    }

    /// Get all cells that a circle might overlap
    fn get_overlapping_cells(&self, center: Vec2, radius: Scalar) -> Vec<GridCell> {
        let mut cells = Vec::new();
        
        // Calculate the bounding box of the circle
        let min_x = (center.x - radius) / self.cell_size;
        let max_x = (center.x + radius) / self.cell_size;
        let min_y = (center.y - radius) / self.cell_size;
        let max_y = (center.y + radius) / self.cell_size;
        
        // Add all cells in the bounding box
        for x in min_x.to_int()..=max_x.to_int() {
            for y in min_y.to_int()..=max_y.to_int() {
                cells.push(GridCell { x, y });
            }
        }
        
        cells
    }

    /// Get potential collision pairs from the grid
    /// Returns pairs of circle indices that might be colliding
    pub fn get_collision_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        let mut checked = BTreeMap::new(); // Track checked pairs to avoid duplicates
        
        // Iterate over all cells in deterministic order (BTreeMap guarantees this)
        for (_cell, indices) in &self.cells {
            // Check all pairs within this cell
            for i in 0..indices.len() {
                for j in (i + 1)..indices.len() {
                    let idx_a = indices[i];
                    let idx_b = indices[j];
                    
                    // Ensure consistent ordering
                    let (min_idx, max_idx) = if idx_a < idx_b {
                        (idx_a, idx_b)
                    } else {
                        (idx_b, idx_a)
                    };
                    
                    // Only add if we haven't checked this pair yet
                    let key = (min_idx, max_idx);
                    if !checked.contains_key(&key) {
                        checked.insert(key, true);
                        pairs.push(key);
                    }
                }
            }
        }
        
        pairs
    }
}

/// Collision detection result
#[derive(Debug, Clone)]
pub struct Collision {
    /// Index of first circle
    pub idx_a: usize,
    /// Index of second circle
    pub idx_b: usize,
    /// Collision normal (from A to B)
    pub normal: Vec2,
    /// Penetration depth
    pub depth: Scalar,
    /// Contact point (in world space)
    pub contact: Vec2,
}

/// Detect actual collisions from potential pairs
/// This is a pure function that checks if circles actually overlap
pub fn detect_collisions(circles: &[Circle], pairs: &[(usize, usize)]) -> Vec<Collision> {
    let mut collisions = Vec::new();
    
    for &(idx_a, idx_b) in pairs {
        let circle_a = &circles[idx_a];
        let circle_b = &circles[idx_b];
        
        // Calculate distance between centers
        let delta = circle_b.position - circle_a.position;
        let dist_sq = delta.length_squared();
        let sum_radii = circle_a.radius + circle_b.radius;
        let sum_radii_sq = sum_radii * sum_radii;
        
        // Check if circles overlap
        if dist_sq < sum_radii_sq && dist_sq > Scalar::ZERO {
            let dist = dist_sq.sqrt();
            let normal = delta / dist; // Normalized direction from A to B
            let depth = sum_radii - dist;
            
            // Contact point is between the two circle centers
            let contact = circle_a.position + normal * circle_a.radius;
            
            collisions.push(Collision {
                idx_a,
                idx_b,
                normal,
                depth,
                contact,
            });
        }
    }
    
    collisions
}

/// Collision with boundary
#[derive(Debug, Clone)]
pub struct BoundaryCollision {
    /// Index of the circle
    pub idx: usize,
    /// Which boundary was hit
    pub boundary: Boundary,
    /// Penetration depth
    pub depth: Scalar,
    /// Contact point
    pub contact: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    Left,
    Right,
    Top,
    Bottom,
}

/// Detect collisions with world boundaries
pub fn detect_boundary_collisions(
    circles: &[Circle],
    world_width: Scalar,
    world_height: Scalar,
) -> Vec<BoundaryCollision> {
    let mut collisions = Vec::new();
    
    for (idx, circle) in circles.iter().enumerate() {
        let pos = circle.position;
        let radius = circle.radius;
        
        // Check left boundary
        if pos.x - radius < Scalar::ZERO {
            collisions.push(BoundaryCollision {
                idx,
                boundary: Boundary::Left,
                depth: radius - pos.x,
                contact: Vec2::from_scalars(Scalar::ZERO, pos.y),
            });
        }
        
        // Check right boundary
        if pos.x + radius > world_width {
            collisions.push(BoundaryCollision {
                idx,
                boundary: Boundary::Right,
                depth: (pos.x + radius) - world_width,
                contact: Vec2::from_scalars(world_width, pos.y),
            });
        }
        
        // Check bottom boundary (y=0 is bottom)
        if pos.y - radius < Scalar::ZERO {
            collisions.push(BoundaryCollision {
                idx,
                boundary: Boundary::Bottom,
                depth: radius - pos.y,
                contact: Vec2::from_scalars(pos.x, Scalar::ZERO),
            });
        }
        
        // Check top boundary
        if pos.y + radius > world_height {
            collisions.push(BoundaryCollision {
                idx,
                boundary: Boundary::Top,
                depth: (pos.y + radius) - world_height,
                contact: Vec2::from_scalars(pos.x, world_height),
            });
        }
    }
    
    collisions
}