# Build stage
FROM rust:1.77-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy only the files needed for dependency resolution
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to satisfy cargo
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Download and compile dependencies
RUN cargo build --release

# Copy the actual source code
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Copy the binary from builder
COPY --from=builder --chown=appuser:appuser /usr/src/app/target/release/rust-compressor /usr/local/bin/compressor

# Switch to non-root user
USER appuser

# Set the entrypoint
ENTRYPOINT ["compressor"]