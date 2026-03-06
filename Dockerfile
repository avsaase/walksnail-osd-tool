# syntax=docker/dockerfile:1

# ---------- Build stage ----------
FROM rust:1.77-slim AS builder

# Install build tools & libraries needed by egui / eframe and other deps
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        libx11-dev \
        libxi-dev \
        libxcursor-dev \
        libxrandr-dev \
        libxinerama-dev \
        libxft-dev \
        libfontconfig-dev \
        libxcb-shape0-dev \
        libxcb-xfixes0-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifest files first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY ui/Cargo.toml ui/Cargo.toml

# Create the sub-crate directories so the previous COPY succeeds even if we
# haven't copied their sources yet (they can be empty for now)
RUN mkdir -p backend ui/src backend/src

# Copy the main entry point so Cargo metadata validates
COPY ui/src/main.rs ui/src/main.rs
COPY backend/src/lib.rs backend/src/lib.rs

# Fetch dependencies in a separate step so they are cached between builds
RUN cargo fetch

# Copy the actual source tree
COPY . .

# Allow Git inside container to operate on mounted repo safely (needed for vergen)
RUN git config --global --add safe.directory /app

# Build the release binary
RUN cargo build --release --bin walksnail-osd-tool

# ---------- Runtime stage ----------
FROM debian:bookworm-slim AS runtime
LABEL org.opencontainers.image.source="https://github.com/avsaase/walksnail-osd-tool"

# Install the minimal set of runtime libraries required by the app
RUN apt-get update && apt-get install -y --no-install-recommends \
        libssl3 \
        libx11-6 \
        libxcb-shape0 \
        libxcb-xfixes0 \
        libxi6 \
        libxrandr2 \
        libxinerama1 \
        libxcursor1 \
        libfontconfig1 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/walksnail-osd-tool /usr/local/bin/walksnail-osd-tool

ENTRYPOINT ["walksnail-osd-tool"] 