# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Determinisk** is a deterministic 2D circle physics engine optimized for zkVM (RISC Zero/SP1) with the following architecture:
- Fixed-point arithmetic (Q16.16) for determinism
- Functional programming patterns inspired by JAX MD
- Spatial hashing for efficient collision detection
- Verlet integration for energy conservation
- Comprehensive event logging and state serialization

## Common Development Tasks

### Initialize Rust Project
```bash
cargo init --lib
cargo add fixed serde sha2 --features serde/derive
cargo add risc0-zkvm --features guest --optional
cargo add sp1-zkvm --optional
```

### Build Commands
```bash
# Standard build
cargo build --release

# Build with RISC Zero support
cargo build --release --features zkvm,risc0

# Build with SP1 support  
cargo build --release --features zkvm,sp1

# Build for zkVM guest
cargo build --release --target riscv32im-risc0-zkvm-elf --features zkvm
```

### Test Commands
```bash
# Run all tests
cargo test

# Run determinism tests
cargo test determinism

# Run physics accuracy tests
cargo test physics

# Run with verbose output
cargo test -- --nocapture

# Test specific module
cargo test math::
```

### Lint and Format
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

## Code Architecture

### Module Structure
```
src/
├── lib.rs              # Main library exports
├── math/               # Fixed-point arithmetic
│   ├── scalar.rs       # Q16.16 fixed-point type
│   └── vec2.rs         # 2D vector operations
├── physics/            # Core physics simulation
│   ├── circle.rs       # Circle entity and config
│   ├── world.rs        # World state and boundaries
│   ├── collision.rs    # Collision detection/resolution
│   ├── integrate.rs    # Verlet integration
│   └── forces.rs       # Force system (springs, fields)
├── spatial/            # Spatial data structures
│   └── grid.rs         # Spatial hashing grid
├── events/             # Event logging system
│   ├── collision.rs    # Collision events
│   ├── boundary.rs     # Boundary events
│   └── proximity.rs    # Proximity detection
├── state/              # State management
│   ├── serialize.rs    # Deterministic serialization
│   └── hash.rs         # State hashing
├── zkvm/               # zkVM integration
│   ├── guest.rs        # Guest program entry
│   ├── host.rs         # Host orchestration
│   └── prove.rs        # Recursive proving
└── debug/              # Debug and visualization
    └── recorder.rs     # Debug hooks
```

### Key Design Patterns

1. **Pure Functional Updates**: All physics state transitions are side-effect free
2. **Fixed Memory Layout**: Pre-allocated structures with compile-time bounds
3. **Deterministic Ordering**: Consistent iteration order for reproducible results
4. **Event Sourcing**: Complete event log for debugging and replay

### Important Implementation Details

- **Fixed-Point Type**: `Scalar(I32F16)` wraps the `fixed` crate's Q16.16 type
- **Verlet Integration**: Positions stored as current and old for implicit velocity
- **Spatial Grid**: Cell size = 2x max circle radius for broad-phase collision
- **Collision Response**: Sequential impulse solver with position correction
- **State Hashing**: SHA-256 of serialized state for determinism verification

## zkVM Integration

### RISC Zero Setup
```bash
# Install RISC Zero toolchain
cargo install cargo-risczero
cargo risczero install

# Create guest program
cargo risczero new determinisk-guest
```

### SP1 Setup
```bash
# Install SP1 toolchain
curl -L https://sp1.succinct.xyz | bash
sp1up

# Create SP1 project
cargo prove new determinisk-sp1
```

### Proof Generation
```rust
// Segment size for recursive proving
const MAX_CYCLES_PER_SEGMENT: u32 = 2_097_152; // 2^21

// Generate proof for simulation segment
let proof = prove_segment(world_state, num_steps)?;
```

## Testing Strategy

### Determinism Tests
- Cross-platform: x86, ARM, WASM must produce identical results
- Serialization: Round-trip must preserve exact state
- Replay: Recorded simulations must replay identically

### Physics Tests
- Energy conservation: < 0.1% drift over 1000 steps
- Momentum conservation in collisions
- Stable stacking of 10+ circles
- Circular orbit stability

### Performance Benchmarks
```bash
# Run benchmarks
cargo bench

# Profile with flamegraph
cargo flamegraph --bench physics_bench
```

## Development Workflow

1. **Implement in phases**: Start with Phase 1 (fixed-point math, basic integration)
2. **Test determinism first**: Every new feature must pass determinism tests
3. **Profile in zkVM**: Use `env::get_cycle_count()` to measure performance
4. **Optimize hotspots**: Focus on collision detection and force calculations

## Commit Standards

Use semantic commit messages following this format:
- `feat:` New feature (use "add" for new functionality, not "implement")
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, missing semicolons, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks
- `perf:` Performance improvements

Examples:
- `feat: add collision detection with spatial hashing`
- `feat: add SP1 zkVM proof generation` (not "implement")
- `fix: correct energy calculation in pendulum example`
- `test: add determinism tests for multi-body scenarios`

## Common Pitfalls

- **Floating-point usage**: Never use `f32`/`f64` in physics calculations
- **HashMap iteration**: Use `BTreeMap` or sorted vectors for deterministic order
- **Random numbers**: All randomness must come from deterministic seeds
- **Time-based logic**: Use step counters, not wall-clock time
- **Testing fixed-point values**: Always test using bit-exact fixed-point comparisons
  - Use `assert_eq!(a, b)` for exact equality, never convert to float for testing
  - Keep all calculations in fixed-point, including energy and other derived quantities
  - Only convert to float for human-readable output or comparing with theoretical formulas
  - For ground collision and boundaries, test exact fixed-point equality: `position.y == radius`

## References

- JAX MD patterns: https://github.com/google/jax-md
- Fixed-point crate: https://docs.rs/fixed
- RISC Zero docs: https://dev.risczero.com
- SP1 docs: https://docs.succinct.xyz/sp1