# Determinisk - zkVM 2D Physics Engine

A deterministic 2D physics engine optimized for zero-knowledge virtual machines (RISC Zero, SP1).

## 🎉 Working Proof Generation!

We have successfully generated and verified cryptographic proofs for physics simulations! The system can prove:
- Correct execution of physics simulations
- Deterministic trajectories with gravity
- Ground collision detection  
- Bit-exact reproducibility

### Live Demo Results

```
Test 1: Ball dropped from height
  Final position: (0.00, 0.00)
  Proof time: 23.36s
  Cycles: 65536
  ✓ Proof verified!

Test 2: Horizontal projectile
  Final position: (179.98, 0.00)
  Proof time: 37.15s
  Cycles: 131072
  ✓ Proof verified!

Test 3: Angled launch
  Final position: (159.99, 0.00)
  Proof time: 44.58s
  Cycles: 131072
  ✓ Proof verified!
```

## Current Status

### ✅ Completed
- **zkVM Toolchain Setup**
  - RISC Zero 2.3.1 installed and configured
  - SP1 5.2.1 installed and configured
  - Rust workspace structure created

- **Core Library Foundation**
  - Fixed-point arithmetic using Q16.16 format (I16F16)
  - 2D vector mathematics with deterministic operations
  - Basic circle physics with Verlet integration
  - Simple gravity simulation
  - All tests passing with bit-exact determinism

- **Proof Generation (NEW!)**
  - RISC Zero guest program implemented
  - Host orchestration for proof generation
  - Multiple physics scenarios proven
  - Independent proof verification working

### 🚧 Next Steps (Phase 2)
- Spatial hashing for collision detection
- Circle-circle collision resolution
- Boundary handling (solid, periodic, open)
- Event logging system

### 🔮 Future Phases
- Force systems (springs, attractors)
- Proximity detection
- State serialization and hashing
- zkVM guest program integration
- Recursive proving architecture

## Project Structure

```
determinisk/
├── determinisk-core/        # Core physics library
│   ├── src/
│   │   ├── math/           # Fixed-point scalar and vector
│   │   ├── physics/        # Circle and world simulation
│   │   ├── spatial/        # Spatial data structures (TODO)
│   │   └── state/          # State management (TODO)
│   └── examples/
│       └── simple_drop.rs  # Basic gravity demo
├── determinisk-risc0/       # RISC Zero guest/host programs
└── determinisk-sp1/         # SP1 proving setup
```

## Quick Start

### Build with CUDA Support (Docker)

```bash
# Build in Docker with CUDA support (solves GCC 15/CUDA incompatibility)
./build-docker.sh build

# After building, run with GPU-accelerated proving (40x faster than CPU)
./target/release/runner run determinisk-runner/input.toml --prove --backend risc0 --verbose

# Or use built-in scenarios
./target/release/runner run pool_break --prove --backend risc0 --verbose
```

### Run Physics Simulation with Visualization and RISC Zero Proofs

```bash
# Navigate to runner directory
cd determinisk-runner

# Run with RISC Zero real proofs and visualization (CPU proving ~80s)
cargo run --release --features risc0 --bin visual -- input.toml

# Or use a built-in scenario
cargo run --release --features risc0 --bin visual -- pool_break
```

### Run Simulation Without Visualization

```bash
# Run with proof generation and verification (CUDA accelerated: ~2s)
../target/release/runner run input.toml --prove --backend risc0 --verbose

# Run without proof generation
cargo run --release --features risc0 --bin runner run input.toml
```

### Generate Zero-Knowledge Proofs (Legacy)

```bash
# Navigate to RISC Zero directory
cd determinisk-risc0

# Generate a single proof
cargo run --package host --bin host

# Generate multiple proofs with different scenarios
cargo run --package host --example verify_physics_full
```

### Run Physics Examples (without proofs)

```bash
# Build the core library
cargo build

# Run tests
cargo test

# Run examples
cargo run --example simple_drop        # Basic gravity demo
cargo run --example multiple_balls     # Multiple balls with different properties
cargo run --example projectile         # Projectile motion at various angles
cargo run --example pendulum          # Pendulum with position constraints
cargo run --example energy_conservation # Demonstrate energy conservation
cargo run --example determinism_proof  # Prove bit-exact determinism
```

## Examples

### Available Simulations

1. **Simple Drop** - A ball falls under gravity and stops at ground
2. **Multiple Balls** - Several balls with different masses and sizes falling
3. **Projectile Motion** - Launch projectiles at different angles, compare with theory
4. **Pendulum** - Constraint-based pendulum showing energy conservation
5. **Energy Conservation** - Track kinetic/potential energy through simulation
6. **Determinism Proof** - Verify bit-identical results across multiple runs
7. **Benchmark** - Performance testing with various numbers of circles
8. **Orbit Simple** - Demonstration of circular orbital motion

### Performance

With the current implementation (no collision detection):
- 10 circles: ~96 million steps/second
- 100 circles: ~9 million steps/second  
- 500 circles: ~2 million steps/second

All benchmarks run on standard hardware in release mode.

### Example Output

The determinism proof shows perfect reproducibility:
```
✓ SUCCESS: All runs produced identical results!

Bit-level precision check:
  Run 1: x_bits = 0xff775fc0, y_bits = 0x00030000
  Run 2: x_bits = 0xff775fc0, y_bits = 0x00030000
  Run 3: x_bits = 0xff775fc0, y_bits = 0x00030000
  Run 4: x_bits = 0xff775fc0, y_bits = 0x00030000
  Run 5: x_bits = 0xff775fc0, y_bits = 0x00030000
```

## Key Features

- **Deterministic**: Bit-identical results across all platforms
- **Fixed-Point Math**: No floating-point operations
- **zkVM Ready**: Optimized for proving in RISC Zero and SP1
- **Observable**: Human-readable state with comprehensive event logging

## Development

See [CLAUDE.md](./CLAUDE.md) for detailed development instructions and commands.