# Determinisk: zkVM Rust 2D Physics Engine

## Primary Goal

**Determinisk** is a **deterministic, verifiable 2D circle physics engine** optimized for recursive proving in modern zkVMs (RISC Zero/SP1), incorporating design patterns from [JAX MD](https://github.com/google/jax-md) while respecting zero-knowledge virtual machine constraints. The engine must provide **observable, human-readable interfaces** for debugging and verification while maintaining the strict determinism required for proof generation.

## Core Objectives

- **Deterministic Simulation**: Bit-identical results across platforms using fixed-point arithmetic
- **zkVM Optimization**: Efficient recursive proving through temporal segmentation
- **Energy Conservation**: Maintain physical accuracy through Verlet integration
- **Observable State**: Human-readable inputs/outputs with comprehensive event logging
- **Composable Proofs**: Hierarchical aggregation for long-running simulations

## System Architecture

The architecture adapts [JAX MD's modular design](https://github.com/google/jax-md#design-philosophy) to zkVM constraints:

```
┌─────────────────────────────────────────────┐
│           Host Application Layer            │
│    (Orchestration, Proof Aggregation)       │
├─────────────────────────────────────────────┤
│          Physics Guest Programs             │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐   │
│  │Dynamics  │ │Collision │ │Constraint│   │
│  │Integrator│ │Detection │ │ Solver   │   │
│  └──────────┘ └──────────┘ └──────────┘   │
├─────────────────────────────────────────────┤
│         Core zkVM Runtime (RISC Zero/SP1)   │
│     (Fixed-Point Math, Spatial Structures)  │
└─────────────────────────────────────────────┘
```

Similar to JAX MD's separation of [space](https://github.com/google/jax-md/blob/main/jax_md/space.py), [partition](https://github.com/google/jax-md/blob/main/jax_md/partition.py), and [simulate](https://github.com/google/jax-md/blob/main/jax_md/simulate.py) modules, each component maintains clear boundaries and pure functional interfaces.

## Design Philosophy

Drawing from [JAX MD's functional approach](https://github.com/google/jax-md) while adapting to zkVM constraints:
- **Pure functional state transitions** - All physics updates are side-effect free, following JAX MD's [state update pattern](https://github.com/google/jax-md/blob/main/jax_md/simulate.py)
- **Fixed memory layout** - Pre-allocated structures with compile-time bounds
- **Temporal segmentation** - Physics divided into provable chunks of 2^21 cycles
- **Hierarchical proof aggregation** - Binary tree composition of segment proofs

## High-Level Implementation Phases

### Phase 1: Foundation
- Fixed-point arithmetic library with Q16.16 implementation
- Basic circle representation and state management
- Simple Verlet integration without collisions
- Initial zkVM integration with RISC Zero or SP1

### Phase 2: Collision System
- Spatial hashing implementation
- Circle-circle collision detection
- Sequential impulse solver
- Neighbor list management

### Phase 3: Optimization
- Custom precompiles for physics operations
- Memory layout optimization
- Batch processing implementation
- Recursive proving architecture

### Phase 4: Production Features
- Comprehensive testing suite
- Performance benchmarking framework
- Documentation and examples
- API stabilization

## JAX MD References

The design patterns in this engine are inspired by JAX MD's functional approach to molecular dynamics. Key references:

### Core JAX MD Documentation
- **[JAX MD GitHub Repository](https://github.com/google/jax-md)** - Main repository with examples and API documentation
- **[JAX MD Paper on arXiv](https://arxiv.org/abs/1912.04232)** - "JAX MD: A Framework for Differentiable Physics" by Schoenholz & Cubuk (NeurIPS 2020)
- **[JAX MD Colab Notebooks](https://github.com/google/jax-md/tree/main/notebooks)** - Interactive examples demonstrating key concepts
- **[JAX MD: End-to-End Differentiable, Hardware Accelerated, Molecular Dynamics](https://arxiv.org/abs/2007.09593)** - Extended paper with performance analysis

### Relevant JAX MD Modules Adapted for zkVM

#### Spatial Partitioning
- **[partition.py](https://github.com/google/jax-md/blob/main/jax_md/partition.py)** - Cell list and neighbor list implementations that inspired our spatial grid approach
- **[space.py](https://github.com/google/jax-md/blob/main/jax_md/space.py)** - Periodic and non-periodic space implementations adapted for fixed-point arithmetic

#### Energy and Forces
- **[energy.py](https://github.com/google/jax-md/blob/main/jax_md/energy.py)** - Energy functions and force derivation patterns
- **[quantity.py](https://github.com/google/jax-md/blob/main/jax_md/quantity.py)** - Physical quantity calculations adapted for deterministic fixed-point

#### Integration Methods
- **[simulate.py](https://github.com/google/jax-md/blob/main/jax_md/simulate.py)** - Integration schemes including Verlet variants
- **[minimize.py](https://github.com/google/jax-md/blob/main/jax_md/minimize.py)** - Optimization methods for stable configurations

### JAX MD Design Patterns Applied

1. **Functional State Transitions**: JAX MD's pure functional approach ([see example](https://github.com/google/jax-md/blob/main/notebooks/lennard_jones.ipynb)) maps directly to zkVM's deterministic requirements

2. **Neighbor Lists with Skin Distance**: The [neighbor list tutorial](https://github.com/google/jax-md/blob/main/notebooks/neighbor_list.ipynb) demonstrates the skin distance concept we adapt for fixed-point arithmetic

3. **Spatial Hashing**: JAX MD's [cell list implementation](https://github.com/google/jax-md/blob/main/jax_md/partition.py#L456) provides the foundation for our spatial grid approach

4. **Energy Conservation**: The [NVE ensemble example](https://github.com/google/jax-md/blob/main/notebooks/nve_ensemble.ipynb) shows energy-conserving integration techniques

**Note**: While JAX MD's automatic differentiation capabilities don't translate to zkVM environments, its functional programming patterns and spatial data structures provide an excellent architectural foundation for deterministic physics simulation.

## Detailed Implementation Steps

### Step 1: Fixed-Point Math Foundation

**Implementation:**
```rust
use fixed::types::I32F16;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scalar(I32F16);

impl Scalar {
    pub fn from_float(f: f32) -> Self {
        Scalar(I32F16::from_num(f))
    }
    
    pub fn to_float(&self) -> f32 {
        self.0.to_num()
    }
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.4}", self.to_float())
    }
}
```

**Testing:**
- Verify arithmetic operations produce bit-identical results across platforms
- Test determinism with repeated calculations
- Validate serialization round-trips preserve exact values
- Confirm overflow behavior is predictable and documented

### Step 2: Vector Mathematics

**Implementation:**
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { 
            x: Scalar::from_float(x), 
            y: Scalar::from_float(y) 
        }
    }
    
    pub fn magnitude_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }
    
    pub fn magnitude(&self) -> Scalar {
        // Newton-Raphson square root
        let squared = self.magnitude_squared();
        if squared.0 <= I32F16::ZERO {
            return Scalar(I32F16::ZERO);
        }
        
        let mut guess = Scalar(squared.0 >> 1);
        for _ in 0..8 {
            guess = Scalar((guess.0 + squared.0 / guess.0) >> 1);
        }
        guess
    }
    
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag.0 > I32F16::ZERO {
            Vec2 { x: self.x / mag, y: self.y / mag }
        } else {
            *self
        }
    }
    
    pub fn dot(&self, other: &Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x.to_float(), self.y.to_float())
    }
}
```

**Testing:**
- Validate magnitude calculations for unit vectors
- Test normalization of zero and non-zero vectors
- Verify dot product geometric properties
- Confirm vector operations maintain precision bounds

### Step 3: Circle Definition with Human-Readable Config

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleConfig {
    pub id: String,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
    pub mass: f32,
    pub restitution: f32,  // Bounciness: 0.0 = no bounce, 1.0 = perfect bounce
    pub friction: f32,      // 0.0 = frictionless, 1.0 = high friction
    pub tags: Vec<String>,  // User-defined tags for filtering/querying
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub position: Vec2,
    pub old_position: Vec2,  // For Verlet integration
    pub radius: Scalar,
    pub mass: Scalar,
    pub restitution: Scalar,
    pub friction: Scalar,
}

impl Circle {
    pub fn from_config(config: &CircleConfig) -> Self {
        let position = Vec2::new(config.position[0], config.position[1]);
        let velocity = Vec2::new(config.velocity[0], config.velocity[1]);
        
        // Calculate old_position from velocity for Verlet
        // old_pos = current_pos - velocity * dt (assuming dt=1/60)
        let dt = Scalar::from_float(1.0 / 60.0);
        let old_position = Vec2 {
            x: position.x - velocity.x * dt,
            y: position.y - velocity.y * dt,
        };
        
        Circle {
            position,
            old_position,
            radius: Scalar::from_float(config.radius),
            mass: Scalar::from_float(config.mass),
            restitution: Scalar::from_float(config.restitution),
            friction: Scalar::from_float(config.friction),
        }
    }
    
    pub fn velocity(&self, dt: Scalar) -> Vec2 {
        Vec2 {
            x: (self.position.x - self.old_position.x) / dt,
            y: (self.position.y - self.old_position.y) / dt,
        }
    }
}
```

**Testing:**
- Verify circle creation from various configurations
- Test velocity calculation from position history
- Validate parameter ranges (mass > 0, 0 ≤ restitution ≤ 1)
- Confirm tags are preserved for user queries

### Step 4: World Configuration and Boundaries

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: f32,
    pub height: f32,
    pub gravity: [f32; 2],
    pub damping: f32,  // Global velocity damping
    pub timestep: f32,
    pub boundary_type: BoundaryType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryType {
    Solid { restitution: f32 },     // Bouncy walls
    Periodic,                        // Wrap around
    Open,                           // No boundaries
}

pub struct World {
    pub bounds: Vec2,
    pub gravity: Vec2,
    pub damping: Scalar,
    pub timestep: Scalar,
    pub boundary_type: BoundaryType,
    pub circles: Vec<Circle>,
    pub circle_ids: Vec<String>,
}

impl World {
    pub fn from_config(world_config: &WorldConfig, circle_configs: &[CircleConfig]) -> Self {
        World {
            bounds: Vec2::new(world_config.width, world_config.height),
            gravity: Vec2::new(world_config.gravity[0], world_config.gravity[1]),
            damping: Scalar::from_float(world_config.damping),
            timestep: Scalar::from_float(world_config.timestep),
            boundary_type: world_config.boundary_type.clone(),
            circles: circle_configs.iter().map(Circle::from_config).collect(),
            circle_ids: circle_configs.iter().map(|c| c.id.clone()).collect(),
        }
    }
}
```

**Testing:**
- Validate world creation with various boundary types
- Test configuration serialization/deserialization
- Verify gravity and damping effects
- Confirm timestep influences simulation speed correctly

### Step 5: Simulation State and Snapshots

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub step: u64,
    pub time: f32,
    pub circles: Vec<CircleState>,
    pub energy: EnergyMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleState {
    pub id: String,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub kinetic_energy: f32,
    pub potential_energy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyMetrics {
    pub total_kinetic: f32,
    pub total_potential: f32,
    pub total_energy: f32,
}

impl World {
    pub fn capture_state(&self, step: u64) -> SimulationState {
        let mut total_ke = Scalar::from_float(0.0);
        let mut total_pe = Scalar::from_float(0.0);
        
        let circle_states: Vec<CircleState> = self.circles.iter()
            .zip(&self.circle_ids)
            .map(|(circle, id)| {
                let vel = circle.velocity(self.timestep);
                let speed_squared = vel.magnitude_squared();
                let ke = circle.mass * speed_squared / Scalar::from_float(2.0);
                let pe = circle.mass * self.gravity.magnitude() * circle.position.y;
                
                total_ke = total_ke + ke;
                total_pe = total_pe + pe;
                
                CircleState {
                    id: id.clone(),
                    position: [circle.position.x.to_float(), circle.position.y.to_float()],
                    velocity: [vel.x.to_float(), vel.y.to_float()],
                    kinetic_energy: ke.to_float(),
                    potential_energy: pe.to_float(),
                }
            })
            .collect();
        
        SimulationState {
            step,
            time: (Scalar::from_float(step as f32) * self.timestep).to_float(),
            circles: circle_states,
            energy: EnergyMetrics {
                total_kinetic: total_ke.to_float(),
                total_potential: total_pe.to_float(),
                total_energy: (total_ke + total_pe).to_float(),
            },
        }
    }
}
```

**Testing:**
- Verify state captures all relevant physics data
- Test energy calculation accuracy
- Validate JSON export readability
- Confirm state can be used for debugging and analysis

### Step 6: Basic Verlet Integration (No Collisions)

Adapted from [JAX MD's Verlet integrators](https://github.com/google/jax-md/blob/main/jax_md/simulate.py#L90) for fixed-point arithmetic.

**Implementation:**
```rust
impl World {
    pub fn step_verlet_basic(&mut self) {
        for circle in &mut self.circles {
            // Store current position
            let current = circle.position;
            
            // Calculate acceleration (only gravity for now)
            let acceleration = self.gravity;
            
            // Verlet integration: x_new = 2x - x_old + a*dt²
            circle.position = Vec2 {
                x: Scalar::from_float(2.0) * current.x - circle.old_position.x 
                   + acceleration.x * self.timestep * self.timestep,
                y: Scalar::from_float(2.0) * current.y - circle.old_position.y 
                   + acceleration.y * self.timestep * self.timestep,
            };
            
            // Apply damping
            if self.damping.0 > I32F16::ZERO {
                let velocity = Vec2 {
                    x: circle.position.x - circle.old_position.x,
                    y: circle.position.y - circle.old_position.y,
                };
                circle.position = Vec2 {
                    x: circle.position.x - velocity.x * self.damping,
                    y: circle.position.y - velocity.y * self.damping,
                };
            }
            
            // Update old position
            circle.old_position = current;
        }
    }
}
```

**Testing:**
- Validate free-fall under gravity
- Test energy conservation without damping
- Verify damping reduces velocity over time
- Confirm integration stability over long simulations

### Step 7: Boundary Handling

**Implementation:**
```rust
impl World {
    pub fn apply_boundaries(&mut self) -> Vec<BoundaryEvent> {
        let mut events = Vec::new();
        
        for (i, circle) in self.circles.iter_mut().enumerate() {
            let id = &self.circle_ids[i];
            
            match &self.boundary_type {
                BoundaryType::Solid { restitution } => {
                    let r = circle.radius;
                    let rest = Scalar::from_float(*restitution);
                    
                    // Check X boundaries
                    if circle.position.x - r < Scalar::from_float(0.0) {
                        circle.position.x = r;
                        let vel_x = circle.position.x - circle.old_position.x;
                        circle.old_position.x = circle.position.x + vel_x * rest;
                        
                        events.push(BoundaryEvent {
                            step: 0, // Will be filled by caller
                            circle_id: id.clone(),
                            boundary: Boundary::Left,
                            impact_velocity: vel_x.to_float(),
                        });
                    }
                    
                    if circle.position.x + r > self.bounds.x {
                        circle.position.x = self.bounds.x - r;
                        let vel_x = circle.position.x - circle.old_position.x;
                        circle.old_position.x = circle.position.x + vel_x * rest;
                        
                        events.push(BoundaryEvent {
                            step: 0,
                            circle_id: id.clone(),
                            boundary: Boundary::Right,
                            impact_velocity: vel_x.to_float(),
                        });
                    }
                    
                    // Similar for Y boundaries...
                },
                
                BoundaryType::Periodic => {
                    circle.position.x = Vec2 {
                        x: circle.position.x.0.rem_euclid(self.bounds.x.0),
                        y: circle.position.y,
                    }.x;
                    
                    circle.position.y = Vec2 {
                        x: circle.position.x,
                        y: circle.position.y.0.rem_euclid(self.bounds.y.0),
                    }.y;
                },
                
                BoundaryType::Open => {
                    // No boundary handling
                }
            }
        }
        
        events
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryEvent {
    pub step: u64,
    pub circle_id: String,
    pub boundary: Boundary,
    pub impact_velocity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Boundary {
    Left, Right, Top, Bottom,
}
```

**Testing:**
- Verify solid boundaries produce correct bounces
- Test periodic boundaries wrap correctly
- Validate boundary events are generated
- Confirm restitution affects bounce height

### Step 8: Spatial Grid Implementation

Inspired by [JAX MD's cell list implementation](https://github.com/google/jax-md/blob/main/jax_md/partition.py#L456) but adapted for fixed-size arrays and deterministic ordering.

**Implementation:**
```rust
const GRID_SIZE: usize = 32;

pub struct SpatialGrid {
    cell_size: Scalar,
    inverse_cell_size: Scalar,
    cells: Vec<Vec<usize>>,  // Dynamic for initial implementation
    width: usize,
    height: usize,
}

impl SpatialGrid {
    pub fn new(world_bounds: Vec2, max_radius: Scalar) -> Self {
        // Cell size should be at least 2x max radius
        let cell_size = max_radius * Scalar::from_float(2.0);
        let inverse_cell_size = Scalar::from_float(1.0) / cell_size;
        
        let width = ((world_bounds.x * inverse_cell_size).to_float() as usize).max(1);
        let height = ((world_bounds.y * inverse_cell_size).to_float() as usize).max(1);
        
        SpatialGrid {
            cell_size,
            inverse_cell_size,
            cells: vec![Vec::new(); width * height],
            width,
            height,
        }
    }
    
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }
    
    pub fn insert(&mut self, index: usize, position: Vec2, radius: Scalar) {
        let min_x = ((position.x - radius) * self.inverse_cell_size)
            .to_float().max(0.0) as usize;
        let max_x = ((position.x + radius) * self.inverse_cell_size)
            .to_float().min((self.width - 1) as f32) as usize;
        let min_y = ((position.y - radius) * self.inverse_cell_size)
            .to_float().max(0.0) as usize;
        let max_y = ((position.y + radius) * self.inverse_cell_size)
            .to_float().min((self.height - 1) as f32) as usize;
        
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let cell_idx = y * self.width + x;
                self.cells[cell_idx].push(index);
            }
        }
    }
    
    pub fn get_potential_collisions(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for cell in &self.cells {
            for i in 0..cell.len() {
                for j in (i + 1)..cell.len() {
                    let pair = if cell[i] < cell[j] {
                        (cell[i], cell[j])
                    } else {
                        (cell[j], cell[i])
                    };
                    
                    if seen.insert(pair) {
                        pairs.push(pair);
                    }
                }
            }
        }
        
        pairs
    }
}
```

**Testing:**
- Validate grid correctly partitions space
- Test potential collision detection accuracy
- Verify performance scaling with circle count
- Confirm no collisions are missed

### Step 9: Circle-Circle Collision Detection

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionEvent {
    pub step: u64,
    pub time: f32,
    pub circle_a: String,
    pub circle_b: String,
    pub position: [f32; 2],
    pub normal: [f32; 2],
    pub penetration: f32,
    pub relative_velocity: f32,
    pub impulse_magnitude: f32,
}

pub struct CollisionData {
    pub indices: (usize, usize),
    pub normal: Vec2,
    pub penetration: Scalar,
    pub contact_point: Vec2,
}

impl World {
    pub fn detect_collisions(&self) -> Vec<CollisionData> {
        let mut collisions = Vec::new();
        
        // Build spatial grid
        let max_radius = self.circles.iter()
            .map(|c| c.radius)
            .max()
            .unwrap_or(Scalar::from_float(1.0));
        
        let mut grid = SpatialGrid::new(self.bounds, max_radius);
        
        for (i, circle) in self.circles.iter().enumerate() {
            grid.insert(i, circle.position, circle.radius);
        }
        
        // Check potential collisions
        for (i, j) in grid.get_potential_collisions() {
            let c1 = &self.circles[i];
            let c2 = &self.circles[j];
            
            let delta = c2.position - c1.position;
            let dist_squared = delta.magnitude_squared();
            let radius_sum = c1.radius + c2.radius;
            let radius_sum_squared = radius_sum * radius_sum;
            
            if dist_squared < radius_sum_squared && dist_squared.0 > I32F16::ZERO {
                let distance = delta.magnitude();
                let normal = delta.normalized();
                let penetration = radius_sum - distance;
                
                // Contact point is between the two centers
                let t = c1.radius / radius_sum;
                let contact_point = Vec2 {
                    x: c1.position.x + delta.x * t,
                    y: c1.position.y + delta.y * t,
                };
                
                collisions.push(CollisionData {
                    indices: (i, j),
                    normal,
                    penetration,
                    contact_point,
                });
            }
        }
        
        collisions
    }
}
```

**Testing:**
- Detect all overlapping circle pairs
- Calculate correct penetration depths
- Compute accurate contact points and normals
- Verify spatial grid optimization works

### Step 10: Collision Resolution with Impulses

**Implementation:**
```rust
impl World {
    pub fn resolve_collisions(&mut self, collisions: &[CollisionData]) -> Vec<CollisionEvent> {
        let mut events = Vec::new();
        
        for collision in collisions {
            let (i, j) = collision.indices;
            
            // Get velocities
            let v1 = self.circles[i].velocity(self.timestep);
            let v2 = self.circles[j].velocity(self.timestep);
            
            // Relative velocity
            let relative_velocity = v2 - v1;
            let velocity_along_normal = relative_velocity.dot(&collision.normal);
            
            // Don't resolve if velocities are separating
            if velocity_along_normal.0 > I32F16::ZERO {
                continue;
            }
            
            // Calculate impulse scalar
            let e = (self.circles[i].restitution + self.circles[j].restitution) 
                    / Scalar::from_float(2.0);
            let impulse_scalar = -(Scalar::from_float(1.0) + e) * velocity_along_normal 
                                / (Scalar::from_float(1.0) / self.circles[i].mass 
                                   + Scalar::from_float(1.0) / self.circles[j].mass);
            
            // Apply impulse
            let impulse = Vec2 {
                x: collision.normal.x * impulse_scalar,
                y: collision.normal.y * impulse_scalar,
            };
            
            // Update velocities through old_position manipulation
            let dt = self.timestep;
            self.circles[i].old_position = Vec2 {
                x: self.circles[i].old_position.x + impulse.x * dt / self.circles[i].mass,
                y: self.circles[i].old_position.y + impulse.y * dt / self.circles[i].mass,
            };
            
            self.circles[j].old_position = Vec2 {
                x: self.circles[j].old_position.x - impulse.x * dt / self.circles[j].mass,
                y: self.circles[j].old_position.y - impulse.y * dt / self.circles[j].mass,
            };
            
            // Position correction to resolve penetration
            let correction_magnitude = collision.penetration * Scalar::from_float(0.8);
            let correction = Vec2 {
                x: collision.normal.x * correction_magnitude,
                y: collision.normal.y * correction_magnitude,
            };
            
            let total_mass = self.circles[i].mass + self.circles[j].mass;
            self.circles[i].position = Vec2 {
                x: self.circles[i].position.x - correction.x * self.circles[j].mass / total_mass,
                y: self.circles[i].position.y - correction.y * self.circles[j].mass / total_mass,
            };
            
            self.circles[j].position = Vec2 {
                x: self.circles[j].position.x + correction.x * self.circles[i].mass / total_mass,
                y: self.circles[j].position.y + correction.y * self.circles[i].mass / total_mass,
            };
            
            // Record event
            events.push(CollisionEvent {
                step: 0,  // Will be filled by caller
                time: 0.0,
                circle_a: self.circle_ids[i].clone(),
                circle_b: self.circle_ids[j].clone(),
                position: [collision.contact_point.x.to_float(), 
                          collision.contact_point.y.to_float()],
                normal: [collision.normal.x.to_float(), 
                        collision.normal.y.to_float()],
                penetration: collision.penetration.to_float(),
                relative_velocity: velocity_along_normal.to_float(),
                impulse_magnitude: impulse_scalar.to_float(),
            });
        }
        
        events
    }
}
```

**Testing:**
- Verify momentum conservation in collisions
- Test elastic and inelastic collisions
- Validate position correction prevents overlap
- Confirm collision events contain accurate data

### Step 11: Proximity Detection System

Building on [JAX MD's neighbor detection patterns](https://github.com/google/jax-md/blob/main/jax_md/partition.py#L300) to track spatial relationships and trigger events.

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityZone {
    pub id: String,
    pub circle_id: String,
    pub radius: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityEvent {
    pub step: u64,
    pub time: f32,
    pub zone_id: String,
    pub circle_id: String,
    pub event_type: ProximityEventType,
    pub distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProximityEventType {
    Enter,
    Exit,
    Stay,
}

pub struct ProximitySystem {
    zones: Vec<ProximityZone>,
    zone_states: Vec<HashSet<String>>,  // Circles currently in each zone
}

impl ProximitySystem {
    pub fn new(zones: Vec<ProximityZone>) -> Self {
        let zone_states = vec![HashSet::new(); zones.len()];
        ProximitySystem { zones, zone_states }
    }
    
    pub fn update(&mut self, world: &World, step: u64) -> Vec<ProximityEvent> {
        let mut events = Vec::new();
        let time = (Scalar::from_float(step as f32) * world.timestep).to_float();
        
        for (zone_idx, zone) in self.zones.iter().enumerate() {
            // Find zone center circle
            let center_idx = world.circle_ids.iter()
                .position(|id| id == &zone.circle_id);
            
            if let Some(center_idx) = center_idx {
                let center = world.circles[center_idx].position;
                let zone_radius = Scalar::from_float(zone.radius);
                
                let mut current_inside = HashSet::new();
                
                // Check all circles
                for (i, circle) in world.circles.iter().enumerate() {
                    let id = &world.circle_ids[i];
                    
                    // Skip self
                    if id == &zone.circle_id {
                        continue;
                    }
                    
                    let dist = (circle.position - center).magnitude();
                    let is_inside = dist < zone_radius + circle.radius;
                    
                    if is_inside {
                        current_inside.insert(id.clone());
                        
                        // Check if this is a new entry
                        if !self.zone_states[zone_idx].contains(id) {
                            events.push(ProximityEvent {
                                step,
                                time,
                                zone_id: zone.id.clone(),
                                circle_id: id.clone(),
                                event_type: ProximityEventType::Enter,
                                distance: dist.to_float(),
                            });
                        }
                    }
                }
                
                // Check for exits
                for id in &self.zone_states[zone_idx] {
                    if !current_inside.contains(id) {
                        events.push(ProximityEvent {
                            step,
                            time,
                            zone_id: zone.id.clone(),
                            circle_id: id.clone(),
                            event_type: ProximityEventType::Exit,
                            distance: 0.0,  // Could calculate if needed
                        });
                    }
                }
                
                self.zone_states[zone_idx] = current_inside;
            }
        }
        
        events
    }
}
```

**Testing:**
- Detect when circles enter proximity zones
- Track exits from zones correctly
- Handle multiple overlapping zones
- Generate events with accurate timing

### Step 12: Force System Implementation

Following [JAX MD's energy-based force derivation](https://github.com/google/jax-md/blob/main/jax_md/energy.py) patterns, adapted for springs and force fields.

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpringConfig {
    pub id: String,
    pub circle_a: String,
    pub circle_b: String,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

pub struct Spring {
    indices: (usize, usize),
    rest_length: Scalar,
    stiffness: Scalar,
    damping: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFieldConfig {
    pub id: String,
    pub field_type: ForceFieldType,
    pub position: Option<[f32; 2]>,  // For point forces
    pub strength: f32,
    pub range: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForceFieldType {
    Gravity,
    PointAttractor,
    PointRepulsor,
    Vortex,
    Damping,
}

pub struct ForceSystem {
    springs: Vec<Spring>,
    spring_ids: Vec<String>,
    force_fields: Vec<ForceFieldConfig>,
}

impl ForceSystem {
    pub fn compute_forces(&self, world: &World) -> Vec<Vec2> {
        let mut forces = vec![Vec2::new(0.0, 0.0); world.circles.len()];
        
        // Apply global gravity
        for i in 0..world.circles.len() {
            forces[i] = Vec2 {
                x: forces[i].x + world.gravity.x * world.circles[i].mass,
                y: forces[i].y + world.gravity.y * world.circles[i].mass,
            };
        }
        
        // Apply springs
        for spring in &self.springs {
            let (i, j) = spring.indices;
            let pos_a = world.circles[i].position;
            let pos_b = world.circles[j].position;
            
            let delta = pos_b - pos_a;
            let distance = delta.magnitude();
            
            if distance.0 > I32F16::ZERO {
                // Spring force
                let force_magnitude = spring.stiffness * (distance - spring.rest_length);
                
                // Damping
                let vel_a = world.circles[i].velocity(world.timestep);
                let vel_b = world.circles[j].velocity(world.timestep);
                let relative_vel = vel_b - vel_a;
                let damping_force = spring.damping * relative_vel.dot(&delta) / distance;
                
                let total_force = (force_magnitude + damping_force) * delta.normalized();
                
                forces[i] = Vec2 {
                    x: forces[i].x + total_force.x,
                    y: forces[i].y + total_force.y,
                };
                forces[j] = Vec2 {
                    x: forces[j].x - total_force.x,
                    y: forces[j].y - total_force.y,
                };
            }
        }
        
        // Apply force fields
        for field in &self.force_fields {
            match field.field_type {
                ForceFieldType::PointAttractor => {
                    if let Some(pos) = field.position {
                        let center = Vec2::new(pos[0], pos[1]);
                        let strength = Scalar::from_float(field.strength);
                        
                        for i in 0..world.circles.len() {
                            let delta = center - world.circles[i].position;
                            let dist = delta.magnitude();
                            
                            if dist.0 > I32F16::ZERO {
                                if let Some(range) = field.range {
                                    if dist.to_float() > range {
                                        continue;
                                    }
                                }
                                
                                let force = delta.normalized() * strength * world.circles[i].mass;
                                forces[i] = Vec2 {
                                    x: forces[i].x + force.x,
                                    y: forces[i].y + force.y,
                                };
                            }
                        }
                    }
                },
                // ... other force field types
                _ => {}
            }
        }
        
        forces
    }
}
```

**Testing:**
- Validate spring forces follow Hooke's law
- Test damping reduces oscillations
- Verify force fields affect particles correctly
- Confirm forces combine additively

### Step 13: Complete Simulation Step

**Implementation:**
```rust
pub struct SimulationEngine {
    pub world: World,
    pub force_system: ForceSystem,
    pub proximity_system: ProximitySystem,
    pub current_step: u64,
    pub event_log: EventLog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub collision_events: Vec<CollisionEvent>,
    pub boundary_events: Vec<BoundaryEvent>,
    pub proximity_events: Vec<ProximityEvent>,
}

impl SimulationEngine {
    pub fn step(&mut self) -> StepResult {
        let step_start = self.current_step;
        let time = (Scalar::from_float(step_start as f32) * self.world.timestep).to_float();
        
        // 1. Compute forces
        let forces = self.force_system.compute_forces(&self.world);
        
        // 2. Integrate positions with Verlet
        for (i, circle) in self.world.circles.iter_mut().enumerate() {
            let current = circle.position;
            let acceleration = Vec2 {
                x: forces[i].x / circle.mass,
                y: forces[i].y / circle.mass,
            };
            
            circle.position = Vec2 {
                x: Scalar::from_float(2.0) * current.x - circle.old_position.x 
                   + acceleration.x * self.world.timestep * self.world.timestep,
                y: Scalar::from_float(2.0) * current.y - circle.old_position.y 
                   + acceleration.y * self.world.timestep * self.world.timestep,
            };
            
            if self.world.damping.0 > I32F16::ZERO {
                let velocity = Vec2 {
                    x: circle.position.x - circle.old_position.x,
                    y: circle.position.y - circle.old_position.y,
                };
                circle.position = Vec2 {
                    x: circle.position.x - velocity.x * self.world.damping,
                    y: circle.position.y - velocity.y * self.world.damping,
                };
            }
            
            circle.old_position = current;
        }
        
        // 3. Handle boundaries
        let mut boundary_events = self.world.apply_boundaries();
        for event in &mut boundary_events {
            event.step = step_start;
        }
        
        // 4. Detect and resolve collisions
        let collisions = self.world.detect_collisions();
        let mut collision_events = self.world.resolve_collisions(&collisions);
        for event in &mut collision_events {
            event.step = step_start;
            event.time = time;
        }
        
        // 5. Update proximity detection
        let proximity_events = self.proximity_system.update(&self.world, step_start);
        
        // 6. Log events
        self.event_log.collision_events.extend(collision_events.clone());
        self.event_log.boundary_events.extend(boundary_events.clone());
        self.event_log.proximity_events.extend(proximity_events.clone());
        
        self.current_step += 1;
        
        StepResult {
            step: step_start,
            time,
            collision_events,
            boundary_events,
            proximity_events,
            state: self.world.capture_state(step_start),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step: u64,
    pub time: f32,
    pub collision_events: Vec<CollisionEvent>,
    pub boundary_events: Vec<BoundaryEvent>,
    pub proximity_events: Vec<ProximityEvent>,
    pub state: SimulationState,
}
```

**Testing:**
- Run complete physics simulations
- Verify all subsystems integrate correctly
- Test event generation completeness
- Validate state consistency across steps

### Step 14: State Serialization and Hashing

**Implementation:**
```rust
use sha2::{Sha256, Digest};

impl World {
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Hash all circle states in order
        for circle in &self.circles {
            hasher.update(&circle.position.x.0.to_be_bytes());
            hasher.update(&circle.position.y.0.to_be_bytes());
            hasher.update(&circle.old_position.x.0.to_be_bytes());
            hasher.update(&circle.old_position.y.0.to_be_bytes());
        }
        
        hasher.finalize().into()
    }
    
    pub fn serialize_deterministic(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Write header
        bytes.extend_from_slice(&(self.circles.len() as u32).to_be_bytes());
        
        // Write each circle
        for (i, circle) in self.circles.iter().enumerate() {
            // Write ID length and ID
            let id = &self.circle_ids[i];
            bytes.extend_from_slice(&(id.len() as u32).to_be_bytes());
            bytes.extend_from_slice(id.as_bytes());
            
            // Write physics data
            bytes.extend_from_slice(&circle.position.x.0.to_bits().to_be_bytes());
            bytes.extend_from_slice(&circle.position.y.0.to_bits().to_be_bytes());
            bytes.extend_from_slice(&circle.old_position.x.0.to_bits().to_be_bytes());
            bytes.extend_from_slice(&circle.old_position.y.0.to_bits().to_be_bytes());
            bytes.extend_from_slice(&circle.radius.0.to_bits().to_be_bytes());
            bytes.extend_from_slice(&circle.mass.0.to_bits().to_be_bytes());
        }
        
        bytes
    }
    
    pub fn deserialize_deterministic(bytes: &[u8]) -> Result<(Self, Vec<String>), String> {
        let mut cursor = 0;
        
        // Read header
        let count = u32::from_be_bytes(
            bytes[cursor..cursor+4].try_into()
                .map_err(|_| "Invalid header")?
        ) as usize;
        cursor += 4;
        
        let mut circles = Vec::with_capacity(count);
        let mut ids = Vec::with_capacity(count);
        
        // Read each circle
        for _ in 0..count {
            // Read ID
            let id_len = u32::from_be_bytes(
                bytes[cursor..cursor+4].try_into()
                    .map_err(|_| "Invalid ID length")?
            ) as usize;
            cursor += 4;
            
            let id = String::from_utf8(
                bytes[cursor..cursor+id_len].to_vec()
            ).map_err(|_| "Invalid ID")?;
            cursor += id_len;
            ids.push(id);
            
            // Read physics data
            let pos_x = I32F16::from_bits(i32::from_be_bytes(
                bytes[cursor..cursor+4].try_into()
                    .map_err(|_| "Invalid position")?
            ));
            cursor += 4;
            
            // ... read remaining fields
            
            circles.push(Circle {
                position: Vec2 { x: Scalar(pos_x), /* ... */ },
                // ... other fields
            });
        }
        
        Ok((World { circles, circle_ids: ids, /* ... */ }, ids))
    }
}
```

**Testing:**
- Verify serialization preserves exact state
- Test hashing produces consistent results
- Validate deserialization recovers identical state
- Confirm different states have different hashes

### Step 15: zkVM Integration Foundation

**Implementation:**
```rust
// For RISC Zero
#[cfg(feature = "zkvm")]
mod zkvm {
    use risc0_zkvm::guest::env;
    
    pub struct ZkVMPhysicsGuest {
        world: World,
        segment_size: u32,
    }
    
    impl ZkVMPhysicsGuest {
        pub fn execute_segment() {
            // Read initial state from host
            let initial_state: SerializedWorldState = env::read();
            let steps_to_execute: u32 = env::read();
            
            let mut world = World::deserialize_deterministic(&initial_state.data)
                .expect("Invalid initial state");
            
            let initial_hash = world.compute_hash();
            
            // Execute physics steps
            for step in 0..steps_to_execute {
                // Check cycle count to avoid exceeding segment limit
                if env::get_cycle_count() > MAX_CYCLES_PER_SEGMENT {
                    break;
                }
                
                world.physics_step();
            }
            
            let final_hash = world.compute_hash();
            let final_state = world.serialize_deterministic();
            
            // Commit results
            env::commit(&PhysicsSegmentProof {
                initial_hash,
                final_hash,
                steps_executed: step,
                final_state,
            });
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PhysicsSegmentProof {
    pub initial_hash: [u8; 32],
    pub final_hash: [u8; 32],
    pub steps_executed: u32,
    pub final_state: Vec<u8>,
}
```

**Testing:**
- Execute physics in zkVM environment
- Verify cycle count stays within limits
- Test proof generation succeeds
- Validate state transitions are provable

### Step 16: Performance Monitoring

**Implementation:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub step: u64,
    pub collision_checks: u32,
    pub collision_resolutions: u32,
    pub force_calculations: u32,
    pub spatial_grid_insertions: u32,
    pub cycle_count: Option<u32>,  // For zkVM
}

impl SimulationEngine {
    pub fn step_with_metrics(&mut self) -> (StepResult, PerformanceMetrics) {
        let mut metrics = PerformanceMetrics {
            step: self.current_step,
            collision_checks: 0,
            collision_resolutions: 0,
            force_calculations: 0,
            spatial_grid_insertions: 0,
            cycle_count: None,
        };
        
        #[cfg(feature = "zkvm")]
        let start_cycles = env::get_cycle_count();
        
        // Track metrics during simulation
        metrics.force_calculations = self.world.circles.len() as u32;
        
        // ... perform simulation step
        
        #[cfg(feature = "zkvm")]
        {
            metrics.cycle_count = Some(env::get_cycle_count() - start_cycles);
        }
        
        (step_result, metrics)
    }
}
```

**Testing:**
- Measure performance at different scales
- Verify sub-quadratic collision detection
- Track cycle counts in zkVM
- Identify optimization opportunities

### Step 17: Recursive Proving Architecture

**Implementation:**
```rust
pub struct RecursiveProver {
    segment_size: u32,
    aggregation_level: u32,
}

impl RecursiveProver {
    pub fn prove_simulation(
        &self,
        initial_state: &World,
        num_steps: u64,
    ) -> Result<AggregatedProof, String> {
        // Divide into segments
        let segments_needed = (num_steps + self.segment_size as u64 - 1) 
                            / self.segment_size as u64;
        
        let mut segment_proofs = Vec::new();
        let mut current_state = initial_state.serialize_deterministic();
        
        // Generate individual segment proofs
        for segment in 0..segments_needed {
            let steps_in_segment = std::cmp::min(
                self.segment_size,
                (num_steps - segment * self.segment_size as u64) as u32
            );
            
            #[cfg(feature = "zkvm")]
            let proof = {
                let receipt = risc0_zkvm::prove(
                    PHYSICS_SEGMENT_ELF,
                    &(current_state, steps_in_segment),
                )?;
                
                let output: PhysicsSegmentProof = receipt.journal.decode()?;
                current_state = output.final_state.clone();
                
                SegmentProof {
                    segment_id: segment,
                    receipt,
                    output,
                }
            };
            
            segment_proofs.push(proof);
        }
        
        // Aggregate proofs in binary tree
        self.aggregate_proofs(segment_proofs)
    }
    
    fn aggregate_proofs(&self, proofs: Vec<SegmentProof>) -> Result<AggregatedProof, String> {
        if proofs.len() == 1 {
            return Ok(AggregatedProof::Leaf(proofs[0].clone()));
        }
        
        let mut current_level = proofs;
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let aggregated = if chunk.len() == 2 {
                    self.aggregate_pair(&chunk[0], &chunk[1])?
                } else {
                    chunk[0].clone()
                };
                next_level.push(aggregated);
            }
            
            current_level = next_level;
        }
        
        Ok(AggregatedProof::Root(Box::new(current_level[0].clone())))
    }
}
```

**Testing:**
- Prove multi-segment simulations
- Verify proof aggregation correctness
- Test state continuity across segments
- Validate final proof verification

### Step 18: Configuration Validation and Error Handling

**Implementation:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum ValidationError {
    InvalidRadius { id: String, radius: f32 },
    InvalidMass { id: String, mass: f32 },
    OutOfBounds { id: String, position: [f32; 2] },
    DuplicateId { id: String },
    InvalidTimestep { timestep: f32 },
    InvalidWorldSize { width: f32, height: f32 },
    SpringReferenceError { spring_id: String, circle_id: String },
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_world_config(config: &WorldConfig) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        if config.timestep <= 0.0 || config.timestep > 0.1 {
            errors.push(ValidationError::InvalidTimestep { 
                timestep: config.timestep 
            });
        }
        
        if config.width <= 0.0 || config.height <= 0.0 {
            errors.push(ValidationError::InvalidWorldSize {
                width: config.width,
                height: config.height,
            });
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    pub fn validate_circles(
        circles: &[CircleConfig],
        world: &WorldConfig,
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();
        
        for circle in circles {
            // Check for duplicate IDs
            if !seen_ids.insert(circle.id.clone()) {
                errors.push(ValidationError::DuplicateId {
                    id: circle.id.clone(),
                });
            }
            
            // Validate radius
            if circle.radius <= 0.0 || circle.radius > world.width / 2.0 {
                errors.push(ValidationError::InvalidRadius {
                    id: circle.id.clone(),
                    radius: circle.radius,
                });
            }
            
            // Validate mass
            if circle.mass <= 0.0 {
                errors.push(ValidationError::InvalidMass {
                    id: circle.id.clone(),
                    mass: circle.mass,
                });
            }
            
            // Check bounds
            if circle.position[0] < 0.0 || circle.position[0] > world.width ||
               circle.position[1] < 0.0 || circle.position[1] > world.height {
                errors.push(ValidationError::OutOfBounds {
                    id: circle.id.clone(),
                    position: circle.position,
                });
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

**Testing:**
- Reject invalid configurations
- Provide clear error messages
- Validate all parameter ranges
- Catch reference errors early

### Step 19: Replay System

**Implementation:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRecording {
    pub metadata: RecordingMetadata,
    pub initial_config: SimulationConfig,
    pub frames: Vec<SimulationFrame>,
    pub events: EventLog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub version: String,
    pub timestamp: u64,
    pub total_steps: u64,
    pub timestep: f32,
    pub determinism_hash: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationFrame {
    pub step: u64,
    pub state_hash: [u8; 32],
    pub state: Option<SimulationState>,  // Can be sparse for compression
}

pub struct ReplaySystem {
    recording: SimulationRecording,
    current_frame: usize,
}

impl ReplaySystem {
    pub fn record_simulation(
        engine: &mut SimulationEngine,
        steps: u64,
        frame_interval: u64,
    ) -> SimulationRecording {
        let initial_hash = engine.world.compute_hash();
        let mut frames = Vec::new();
        
        for step in 0..steps {
            let result = engine.step();
            
            // Record frame at intervals or important events
            if step % frame_interval == 0 || !result.collision_events.is_empty() {
                frames.push(SimulationFrame {
                    step,
                    state_hash: engine.world.compute_hash(),
                    state: Some(result.state),
                });
            } else {
                // Just record hash for verification
                frames.push(SimulationFrame {
                    step,
                    state_hash: engine.world.compute_hash(),
                    state: None,
                });
            }
        }
        
        SimulationRecording {
            metadata: RecordingMetadata {
                version: "1.0.0".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                total_steps: steps,
                timestep: engine.world.timestep.to_float(),
                determinism_hash: engine.world.compute_hash(),
            },
            initial_config: // ... extract from engine
            frames,
            events: engine.event_log.clone(),
        }
    }
    
    pub fn verify_replay(&mut self, engine: &mut SimulationEngine) -> Result<(), String> {
        // Reset to initial state
        engine.world = World::from_config(
            &self.recording.initial_config.world,
            &self.recording.initial_config.circles,
        );
        
        for frame in &self.recording.frames {
            engine.step();
            let current_hash = engine.world.compute_hash();
            
            if current_hash != frame.state_hash {
                return Err(format!(
                    "Determinism failure at step {}: expected {:?}, got {:?}",
                    frame.step, frame.state_hash, current_hash
                ));
            }
        }
        
        Ok(())
    }
}
```

**Testing:**
- Record complete simulations
- Verify deterministic replay
- Test compression with sparse frames
- Validate event synchronization

### Step 20: Advanced Debugging and Visualization Hooks

**Implementation:**
```rust
pub trait DebugVisualizer {
    fn on_step_start(&mut self, step: u64, world: &World);
    fn on_collision(&mut self, collision: &CollisionData, world: &World);
    fn on_force_applied(&mut self, circle_id: usize, force: Vec2);
    fn on_step_complete(&mut self, step: u64, world: &World, events: &StepResult);
}

pub struct DebugRecorder {
    pub spatial_grid_viz: Vec<SpatialGridSnapshot>,
    pub force_vectors: Vec<ForceVectorSnapshot>,
    pub collision_normals: Vec<CollisionNormalSnapshot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpatialGridSnapshot {
    pub step: u64,
    pub occupied_cells: Vec<(usize, usize, Vec<String>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceVectorSnapshot {
    pub step: u64,
    pub circle_id: String,
    pub position: [f32; 2],
    pub force: [f32; 2],
    pub magnitude: f32,
}

impl DebugVisualizer for DebugRecorder {
    fn on_force_applied(&mut self, circle_id: usize, force: Vec2) {
        // Record force for visualization
        self.force_vectors.push(ForceVectorSnapshot {
            step: 0,  // Will be updated
            circle_id: format!("circle_{}", circle_id),
            position: [0.0, 0.0],  // Will be updated
            force: [force.x.to_float(), force.y.to_float()],
            magnitude: force.magnitude().to_float(),
        });
    }
    
    // ... other methods
}

pub struct SimulationEngineDebug {
    engine: SimulationEngine,
    visualizers: Vec<Box<dyn DebugVisualizer>>,
}

impl SimulationEngineDebug {
    pub fn step_with_debug(&mut self) -> StepResult {
        let step = self.engine.current_step;
        
        // Notify start
        for viz in &mut self.visualizers {
            viz.on_step_start(step, &self.engine.world);
        }
        
        // Execute with hooks
        let result = self.engine.step();
        
        // Notify completion
        for viz in &mut self.visualizers {
            viz.on_step_complete(step, &self.engine.world, &result);
        }
        
        result
    }
}
```

**Testing:**
- Capture internal algorithm state
- Visualize spatial partitioning
- Record force vectors for analysis
- Export debug data for external tools

## Testing Strategy

Following [JAX MD's testing approach](https://github.com/google/jax-md/tree/main/tests) adapted for zkVM constraints:

### Determinism Tests
- **Cross-platform equivalence**: Same input produces identical output on x86, ARM, WASM
- **Compiler independence**: Results unchanged with different optimization levels
- **Serialization round-trip**: State survives serialization/deserialization
- **Replay consistency**: Recorded simulations replay identically

### Physics Accuracy Tests
- **Energy conservation**: Total energy drift < 0.1% over 1000 steps (see [JAX MD's NVE examples](https://github.com/google/jax-md/blob/main/notebooks/nve_ensemble.ipynb))
- **Momentum conservation**: Linear and angular momentum preserved in collisions
- **Stable stacking**: 10+ circles can form stable piles
- **Orbit stability**: Circular orbits maintain radius over time

### Performance Tests
- **Collision scaling**: Sub-quadratic growth with spatial partitioning (similar to [JAX MD's scaling](https://github.com/google/jax-md#performance))
- **Memory bounds**: Fixed memory usage regardless of simulation time
- **Cycle efficiency**: Complete segments within zkVM limits
- **Proof generation**: Under 60 seconds per segment with GPU

## Performance Optimization Strategies

### Precompile Development
```rust
pub mod precompiles {
    pub const CIRCLE_COLLISION_DETECTION: u32 = 0x1001;
    pub const SPATIAL_HASH_INSERT: u32 = 0x1002;
    pub const VERLET_INTEGRATION: u32 = 0x1003;
    pub const IMPULSE_RESOLUTION: u32 = 0x1004;
}
```

### Memory Layout Optimization

Following [JAX MD's array-based storage patterns](https://github.com/google/jax-md/blob/main/jax_md/dataclasses.py) adapted for cache efficiency:

```rust
#[repr(C, align(64))]  // Cache line alignment
pub struct OptimizedCircleData {
    // Hot data - accessed frequently
    pos_x: [FixedPoint; BATCH_SIZE],
    pos_y: [FixedPoint; BATCH_SIZE],
    old_x: [FixedPoint; BATCH_SIZE],
    old_y: [FixedPoint; BATCH_SIZE],
    
    // Cold data - accessed less frequently
    radius: [FixedPoint; BATCH_SIZE],
    mass: [FixedPoint; BATCH_SIZE],
    restitution: [FixedPoint; BATCH_SIZE],
    friction: [FixedPoint; BATCH_SIZE],
}
```

## zkVM Platform Selection

### RISC Zero (Recommended)
- **Mature ecosystem** with extensive documentation
- **GPU acceleration** providing 10-20x speedup
- **Three-layer proof system** optimal for recursive physics
- **Application-defined precompiles** in zkVM 1.2
- **Performance**: 30-60 seconds per segment with GPU

### SP1 (Alternative)
- **Faster development iteration**
- **Custom precompiles** are critical
- **Open-source contribution** important
- **Trade-offs**: Larger proof sizes

## Success Metrics

### Core Functionality
- Handle 1000+ colliding circles at 60 Hz
- Deterministic across all target platforms
- Energy conservation within 0.1%
- Complete event logging and replay

### zkVM Performance
- Proof generation under 2 minutes for 1000 timesteps
- Support for recursive proving of hour-long simulations
- Memory usage under 8GB for moderate simulations
- Final proof size under 1KB

## Conclusion

**Determinisk** provides both a high-level roadmap and detailed implementation steps for building a production-ready, zkVM-optimized 2D physics engine. The 20 implementation steps with code examples provide a clear path from basic fixed-point math to a fully-featured deterministic physics simulation with comprehensive debugging capabilities. 

The engine successfully adapts [JAX MD's functional programming patterns](https://github.com/google/jax-md) to the constraints of zero-knowledge virtual machines, maintaining the elegance of JAX MD's design while ensuring bit-exact determinism required for proof generation. By following this phased approach with continuous testing and validation, Determinisk can deliver an efficient, observable, and verifiable physics engine suitable for zero-knowledge proof generation.

For developers familiar with JAX MD, the [migration guide](https://github.com/google/jax-md/blob/main/README.md) and [example notebooks](https://github.com/google/jax-md/tree/main/notebooks) provide additional context for understanding the design decisions in this zkVM adaptation.