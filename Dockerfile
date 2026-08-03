# Dockerfile for claud-agent distributed system
# Multi-stage build for optimized production image

# Build stage
FROM rust:1.94-slim AS builder

# Install protobuf compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libssl-dev \
    pkg-config\
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/claud-agent

# Copy all source files
COPY . .

RUN export OPENSSL_DIR=/usr/lib/ssl
# Build the project in release mode
RUN cargo build --release

# Production stage
FROM debian:12-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    protobuf-compiler \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN export OPENSSL_DIR=/usr/lib/ssl
#RUN openssl version -d
# Copy binary from builder
COPY --from=builder /usr/src/claud-agent/target/release/claud-agent /usr/local/bin/claud-agent

# Copy protobuf definitions (optional, for reference)
COPY --from=builder /usr/src/claud-agent/proto /app/proto

# Create a non-root user for security
RUN useradd -r -s /bin/false claud-agent

# Switch to non-root user
USER claud-agent

# Set working directory
WORKDIR /app

# Expose ports (gRPC on 8338, REST on 8330)
EXPOSE 8338 8330

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/claud-agent"]

# Default command (can be overridden)
CMD []
