# Use NVIDIA CUDA base image with Ubuntu 22.04
FROM nvidia/cuda:12.4.0-devel-ubuntu22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_BACKTRACE=1
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
ENV RISC0_PROVER=cuda
ENV CUDA_HOME=/usr/local/cuda

# Install dependencies and GCC 13
RUN apt-get update && apt-get install -y \
    software-properties-common \
    curl \
    git \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    && add-apt-repository ppa:ubuntu-toolchain-r/test \
    && apt-get update \
    && apt-get install -y gcc-13 g++-13 \
    && update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-13 100 \
    && update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-13 100 \
    && update-alternatives --install /usr/bin/cc cc /usr/bin/gcc 100 \
    && update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++ 100 \
    && rm -rf /var/lib/apt/lists/*

# Verify GCC version
RUN gcc --version && g++ --version

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rustup default stable

# Install rzup and RISC Zero toolchain
RUN curl -L https://risczero.com/install | bash \
    && . $HOME/.bashrc \
    && rzup install

# Install sccache for build caching (disabled for CUDA compatibility)
# RUN cargo install sccache
# ENV RUSTC_WRAPPER=sccache

# Create workspace directory
WORKDIR /workspace

# Set CUDA host compiler for nvcc
ENV CUDAHOSTCXX=/usr/bin/g++-13

# Default command
CMD ["/bin/bash"]