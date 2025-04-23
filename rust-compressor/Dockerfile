# Build stage
FROM rust:1.78-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy only the files needed for dependency resolution
COPY Cargo.toml ./

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

# Install runtime dependencies and debugging tools
RUN apt-get update && apt-get install -y \
    ca-certificates \
    file \
    strace \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user with the same UID/GID as the host user
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN groupadd -g $GROUP_ID appuser && \
    useradd -u $USER_ID -g $GROUP_ID -m appuser

# Create app directory and set permissions
RUN mkdir -p /app && chown -R appuser:appuser /app

# Copy the binary from builder
COPY --from=builder --chown=appuser:appuser /usr/src/app/target/release/compressor /usr/local/bin/compressor

# Switch to non-root user
USER appuser

# Set working directory
WORKDIR /app

# Set the entrypoint
ENTRYPOINT ["compressor"]