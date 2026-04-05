# ------------------------------------------------------------------------------
# Stage 1: Builder
# This stage builds the application and its dependencies.
# ------------------------------------------------------------------------------
FROM rust:1.88-slim-bookworm AS builder

# Install build dependencies required by rusqlite (bundled feature compiles
# SQLite from source via the cc crate and needs a C compiler)
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests first to leverage layer caching
COPY Cargo.toml Cargo.lock ./

# Stub out a minimal src/main.rs so Cargo can resolve and compile dependencies
# without the real application sources — this layer is only invalidated when
# Cargo.toml or Cargo.lock change.
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

# Overlay with the real application sources
COPY src/ ./src/
COPY Rocket.toml ./

# Touch main.rs so Cargo detects the change, rebuild only app code, then copy
# the binary out of the cache-mounted target/ into the image layer.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    touch src/main.rs && \
    cargo build --release && \
    cp target/release/rust-samples-rocket-restful /app/rust-samples-rocket-restful

# ------------------------------------------------------------------------------
# Stage 2: Runtime
# This stage creates the final, minimal image to run the application.
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install curl for health check
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Metadata labels for the image. These are useful for registries and inspection.
LABEL org.opencontainers.image.title="🧪 RESTful API with Rust and Rocket"
LABEL org.opencontainers.image.description="Proof of Concept for a RESTful API made with Rust and Rocket"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/nanotaboada/rust-samples-rocket-restful"
LABEL org.sonarsource.docker.dockerfile="/Dockerfile"

# https://rules.sonarsource.com/docker/RSPEC-6504/

# Copy application binary and Rocket configuration
COPY --from=builder /app/rust-samples-rocket-restful .
COPY --from=builder /app/Rocket.toml                 ./Rocket.toml

# Copy metadata docs for container registries (e.g.: GitHub Container Registry)
COPY --chmod=444    README.md                        ./

# Copy entrypoint and healthcheck scripts
COPY --chmod=555    scripts/entrypoint.sh            ./entrypoint.sh
COPY --chmod=555    scripts/healthcheck.sh           ./healthcheck.sh

# The 'hold' is our storage compartment within the image. Here, we copy a
# pre-seeded SQLite database file, which Compose will mount as a persistent
# 'storage' volume when the container starts up.
COPY --chmod=555    storage/                         ./hold/

# Add system user and prepare volume mount point
RUN addgroup --system rocket && \
    adduser --system --ingroup rocket rocket && \
    mkdir -p /storage && \
    chown -R rocket:rocket /storage

ENV STORAGE_PATH=/storage/players-sqlite3.db

USER rocket

EXPOSE 9000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD ["./healthcheck.sh"]

ENTRYPOINT ["./entrypoint.sh"]
CMD ["./rust-samples-rocket-restful"]
