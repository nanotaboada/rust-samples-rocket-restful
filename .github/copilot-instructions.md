# Custom Instructions

## Overview

REST API for managing football players built with Rust and Rocket. Implements CRUD operations with SQLite persistence (`rusqlite`, bundled), a layered architecture, and Serde JSON serialization. Part of a cross-language comparison study (.NET, Go, Java, Python, TypeScript).

## Tech Stack

- **Language**: Rust 2024 Edition (enforced via `rust-toolchain.toml`)
- **Framework**: Rocket 0.5 (async)
- **Serialization**: Serde (JSON)
- **Unique IDs**: uuid (v4 + serde features)
- **Storage**: SQLite via `rusqlite` (bundled + trace features), `Mutex<Connection>`
- **Testing**: Rust built-in test framework

## Structure

```text
src/
├── models/             — data structures: PlayerRequest (input), PlayerResponse (output)
├── routes/             — async Rocket handlers; HTTP concerns only                         [HTTP layer]
├── services/           — pure business logic; no HTTP knowledge; returns Result types      [business layer]
└── state/              — thread-safe data access via Mutex<Connection>                     [data layer]
tests/                  — integration tests (Arrange/Act/Assert pattern)
storage/
└── players-sqlite3.db  — pre-seeded SQLite database (committed)
Rocket.toml             — server configuration (address, port)
rust-toolchain.toml     — Rust 2024 edition lock
```

**Layer rule**: `Routes → Services → State`. Routes must not contain business logic. Services must not have HTTP knowledge. State handles all data access.

## Key Design: Surrogate vs. Natural Key

| Concern | Key | Route |
| ------- | --- | ----- |
| Surrogate key | UUID (`id`) | `GET /players/{uuid}` — admin lookup only |
| Natural key | `squad_number` | All mutation routes: `PUT` and `DELETE /players/squadnumber/{squad_number}` |

Both `id` (UUID) and `squad_number` are **immutable once set**. On `PUT`, the UUID and squad number from the existing record are always preserved — the request body values for these fields are ignored.

## Coding Guidelines

- **Naming**: snake_case (functions/variables/files), PascalCase (types/traits/structs)
- **Ownership**: minimize `.clone()` calls; prefer references
- **Errors**: `Result<T, CustomError>` with domain-specific error types; never `unwrap()` or `expect()` in production paths; always propagate with `?`
- **Safety**: no blocking operations in async handlers; no global mutable state without `Mutex`
- **Tests**: integration tests in `tests/`; Arrange/Act/Assert with section comments; fixture functions for test data (not stubs); naming `test_request_{method}_{endpoint}_{condition}_response_{verification}`; verify complete response objects
- **Test fixtures**: use `initialize_test_database()` (defined in `src/state/player_collection.rs`) for the full 26-player seeded in-memory DB; use `initialize_empty_test_database()` (defined in `src/state/player_collection.rs`) for an empty schema with no rows; use `player_request_for_creation()`, `player_request_for_update()`, and `SEED_MESSI_ID` from `tests/common` for request bodies and UUID-based GET tests — never hardcode the UUID string inline
- **Avoid**: `unwrap()`/`expect()` in production, unnecessary `.clone()`, blocking in async handlers, missing `?` propagation, inline comments between AAA test sections

## Commands

### Quick Start

```bash
cargo build
cargo run                               # starts on port 9000
cargo test
cargo test test_request_get_players     # run specific test
cargo test -- --nocapture               # with output
```

### Pre-commit Checks

1. `cargo fmt`
2. `cargo clippy --all-targets --all-features -- -D warnings` — must pass clean
3. `cargo build` — must succeed
4. `cargo test` — all tests must pass
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
- `Rocket.toml` server configuration
- `rust-toolchain.toml`

### Never modify

- Seed data in `src/state/player_collection.rs` (without discussion)
- `storage/players-sqlite3.db` directly — regenerate by deleting and running `cargo run`
- Port configuration (9000)
- `rust-toolchain.toml` toolchain version
- The surrogate/natural key design (UUID for GET, squad number for PUT/DELETE)

### Creating Issues

This project uses Spec-Driven Development (SDD): discuss in Plan mode first, create a GitHub Issue as the spec artifact, then implement. Always offer to draft an issue before writing code.

**Feature request** (`enhancement` label):
- **Problem**: the pain point being solved
- **Proposed Solution**: expected behavior and functionality
- **Suggested Approach** *(optional)*: implementation plan if known
- **Acceptance Criteria**: at minimum — behaves as proposed, tests added/updated, no regressions
- **References**: related issues, docs, or examples

**Bug report** (`bug` label):
- **Description**: clear summary of the bug
- **Steps to Reproduce**: numbered, minimal steps
- **Expected / Actual Behavior**: one section each
- **Environment**: runtime versions + OS
- **Additional Context**: logs, screenshots, stack traces
- **Possible Solution** *(optional)*: suggested fix or workaround

### Key workflows

**Add an endpoint**: Add route handler in `src/routes/players.rs` → add service function in `src/services/player_service.rs` with `Result<T, CustomError>` return → add integration test in `tests/` following naming convention → update doc comments → run pre-commit checks.

**After completing work**: Suggest a branch name (e.g. `feat/add-player-stats`) and a commit message following Conventional Commits including co-author line:

```text
feat(scope): description (#issue)

Co-authored-by: Claude Sonnet 4.6 <noreply@anthropic.com>
```
