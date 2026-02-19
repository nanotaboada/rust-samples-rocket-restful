# GitHub Copilot Instructions

## Overview

REST API for managing football players built with Rust and Rocket. Implements CRUD operations with in-memory thread-safe storage (`Mutex<Vec<Player>>`), a three-layer architecture, and Serde JSON serialization. Part of a cross-language comparison study (.NET, Go, Java, Python, TypeScript).

## Tech Stack

- **Language**: Rust 2024 Edition (enforced via `rust-toolchain.toml`)
- **Framework**: Rocket 0.5 (async)
- **Serialization**: Serde (JSON)
- **Storage**: In-memory `Mutex<Vec<Player>>` (SQLite planned — Issue #23)
- **Testing**: Rust built-in test framework
- **Containerization**: Docker

## Structure

```text
src/
├── models/             — data structures: Player (storage), PlayerRequest (input), PlayerResponse (output)
├── routes/             — async Rocket handlers; HTTP concerns only                         [HTTP layer]
├── services/           — pure business logic; no HTTP knowledge; returns Result types      [business layer]
└── state/              — thread-safe data access via Mutex<Vec<Player>>                    [data layer]
tests/                  — integration tests (Arrange/Act/Assert pattern)
players.json            — seed data (26 players)
Rocket.toml             — server configuration (address, port)
rust-toolchain.toml     — Rust 2024 edition lock
```

**Layer rule**: `Routes → Services → State`. Routes must not contain business logic. Services must not have HTTP knowledge. State handles all data access.

## Coding Guidelines

- **Naming**: snake_case (functions/variables/files), PascalCase (types/traits/structs)
- **Ownership**: minimize `.clone()` calls; prefer references
- **Slices**: use `&[T]`/`&mut [T]` instead of `&Vec<T>`/`&mut Vec<T>` when Vec-specific methods aren't needed
- **Errors**: `Result<T, CustomError>` with domain-specific error types; never `unwrap()` or `expect()` in production paths; always propagate with `?`
- **Safety**: no blocking operations in async handlers; no global mutable state without `Mutex`
- **Tests**: integration tests in `tests/`; Arrange/Act/Assert with section comments; fixture functions for test data (not stubs); naming `test_request_{method}_{endpoint}_{condition}_response_{verification}`; verify complete response objects
- **Avoid**: `unwrap()`/`expect()` in production, unnecessary `.clone()`, blocking in async handlers, missing `?` propagation, inline comments between AAA test sections, `&Vec<T>` when a slice suffices

## Commands

### Quick Start

```bash
cargo build
cargo run                               # starts on port 9000
cargo test
cargo test test_request_get_players     # run specific test
cargo test -- --nocapture               # with output
docker compose up --build
```

### Pre-commit Checks

1. `cargo fmt`
2. `cargo clippy --all-targets --all-features -- -D warnings` — must pass clean
3. `cargo test` — all tests must pass
4. `cargo build` — must succeed
5. Commit message follows Conventional Commits format (enforced by commitlint)

### Commits

Format: `type(scope): description (#issue)` — max 80 chars
Types: `feat` `fix` `chore` `docs` `test` `refactor` `ci` `perf`
Example: `feat(api): add player stats endpoint (#42)`

## Agent Mode

### Proceed freely

- Route handlers and HTTP response mapping
- Service layer business logic
- Integration tests (AAA pattern)
- Doc comments (`///`) on public items
- README and documentation updates
- Bug fixes within existing patterns

### Ask before changing

- Architecture (adding/removing layers)
- Dependencies (`Cargo.toml`)
- CI/CD configuration (`.github/workflows/`)
- Docker setup
- `Rocket.toml` server configuration
- `rust-toolchain.toml`

### Never modify

- `players.json` seed data (without discussion)
- Port configuration (9000)
- `rust-toolchain.toml` toolchain version
- Production deployment configurations

### Key workflows

**Add an endpoint**: Add route handler in `src/routes/players.rs` → add service function in `src/services/player_service.rs` with `Result<T, CustomError>` return → add integration test in `tests/` following naming convention → update doc comments → run pre-commit checks.

**After completing work**: Suggest a branch name (e.g. `feat/add-player-stats`) and a commit message following Conventional Commits including co-author line:

```text
feat(scope): description (#issue)

Co-authored-by: Copilot <175728472+Copilot@users.noreply.github.com>
```
