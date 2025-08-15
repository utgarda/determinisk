#!/usr/bin/env zsh
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo "${YELLOW}[WARNING]${NC} $1"
}

# Check for NVIDIA GPU
check_gpu() {
    if ! command -v nvidia-smi &> /dev/null; then
        print_error "nvidia-smi not found. Please install NVIDIA drivers."
        exit 1
    fi
    
    print_status "GPU detected:"
    nvidia-smi --query-gpu=name --format=csv,noheader
}

# Build Docker image
build_image() {
    print_status "Building Docker image with CUDA support..."
    docker-compose build
}

# Run build in container
run_build() {
    print_status "Running build in container..."
    print_status "Building with CUDA support for host execution..."
    docker-compose run --rm risc0-cuda bash -c "
        export RISC0_PROVER=cuda && \
        cd /workspace && \
        cargo build --release && \
        cd determinisk-runner && \
        cargo build --release --features risc0 --bin visual && \
        cargo build --release --features risc0 --bin runner
    "
    print_status "Build complete! Binaries available at:"
    print_status "  - target/release/visual"
    print_status "  - target/release/runner"
    print_status ""
    print_status "Run locally with:"
    print_status "  RISC0_PROVER=cuda ./target/release/visual input.toml"
}

# Run visual demo with RISC Zero proof
run_visual() {
    print_status "Running visual demo with RISC Zero proof..."
    docker-compose run --rm risc0-cuda bash -c "
        cd /workspace/determinisk-runner && \
        cargo run --release --features risc0 --bin visual
    "
}

# Interactive shell
shell() {
    print_status "Starting interactive shell in container..."
    docker-compose run --rm risc0-cuda
}

# Clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    docker-compose run --rm risc0-cuda bash -c "cd /workspace && cargo clean"
}

# Stop and remove containers
down() {
    print_status "Stopping and removing containers..."
    docker-compose down
}

# Main menu
case "${1:-help}" in
    build-image)
        build_image
        ;;
    build)
        check_gpu
        build_image
        run_build
        ;;
    visual)
        check_gpu
        run_visual
        ;;
    shell)
        check_gpu
        shell
        ;;
    clean)
        clean
        ;;
    down)
        down
        ;;
    help|*)
        echo "Usage: $0 {build-image|build|visual|shell|clean|down|help}"
        echo ""
        echo "Commands:"
        echo "  build-image  - Build the Docker image"
        echo "  build        - Build the project with CUDA support"
        echo "  visual       - Run the visual demo with RISC Zero proof"
        echo "  shell        - Start an interactive shell in the container"
        echo "  clean        - Clean build artifacts"
        echo "  down         - Stop and remove containers"
        echo "  help         - Show this help message"
        ;;
esac