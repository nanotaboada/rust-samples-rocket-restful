# ADR-0008: Docker and Compose Strategy

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a containerisation strategy for local development and
deployment. Options considered:

- **Single-stage Dockerfile** — one `FROM rust` image builds and runs the
  binary. Simple, but ships the full Rust toolchain (~2 GB) in the runtime
  image.
- **Multi-stage without Compose** — smaller image, but no persistent volume
  for the SQLite file; the database is lost on container restart.
- **No Docker** — local-only; works for a developer PoC but prevents
  consistent environment setup across machines and CI.
- **Multi-stage Dockerfile + Docker Compose** — build stage compiles the
  release binary; minimal runtime stage (Debian slim) ships only the binary and
  its linked libraries. Compose adds a named volume for SQLite persistence and
  exposes port 9000.

## Decision

We will use a multi-stage Dockerfile:

1. **Builder stage** (`rust:slim`): runs `cargo build --release`.
2. **Runtime stage** (`debian:bookworm-slim`): copies the compiled binary;
   includes only `libssl` and CA certificates.

Docker Compose provides a named volume (`sqlite_data`) mounted at `/data` so
the SQLite file persists across container restarts. The container listens on
port 9000 (the project invariant).

## Consequences

- **Positive**: The runtime image contains no Rust toolchain, reducing image
  size significantly compared to a single-stage build. Named volume ensures
  data durability without bind-mounting a host directory. Compose makes local
  development a single `docker compose up` command.
- **Negative**: Multi-stage builds increase build time slightly (two image
  layers). Any change to system dependencies (e.g., adding a C library) must be
  reflected in both stages.
- **Neutral**: The `bundled` libsqlite3 feature (see ADR-0003) means the
  runtime image does not need a system SQLite package. The SQLite file path
  is configured via `Rocket.toml` and the `DATABASE_URL` environment variable.
