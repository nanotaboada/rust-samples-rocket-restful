# CLAUDE.md

## Overview

REST API for managing football players built with Rust and Rocket. Implements CRUD operations with SQLite persistence (Diesel ORM, r2d2 connection pool, bundled libsqlite3), a four-layer architecture, version-controlled migrations, and Serde JSON serialization.

## Architecture

**Layer rule**: `Routes → Services → Repository → State`. Routes must not contain business logic. Services must not reference Diesel or SQL directly. Repositories own all data access. State manages the connection pool.

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
- **Test fixtures**: use `initialize_test_database()` (defined in `src/state/player_collection.rs`) for the full 26-player seeded in-memory pool (`max_size(1)`); use `initialize_empty_test_database()` for an in-memory pool with schema only (runs first migration, no seed rows); call `pool.get().expect("pool connection")` to obtain a `&mut SqliteConnection` for service calls; use `player_request_for_creation()`, `player_request_for_update()`, and `EXISTING_PLAYER_ID` from `tests/common` for request bodies and UUID-based GET tests — never hardcode the UUID string inline
- **Avoid**: `unwrap()`/`expect()` in production, unnecessary `.clone()`, blocking in async handlers, missing `?` propagation, inline comments between AAA test sections

## Commands

### Pre-commit Checks

1. `cargo fmt`
2. `cargo clippy --all-targets --all-features -- -D warnings` — must pass clean
3. `cargo build` — must succeed
4. `cargo test` — all tests must pass
5. Commit message follows Conventional Commits format (enforced by commitlint)
6. If this commit introduces or changes an architectural decision, update `CLAUDE.md` and create or amend the relevant ADR in `docs/adr/`.
7. If this commit adds, removes, or changes an endpoint, a dependency, or how the project is run, update `README.md` and the relevant path instructions in `.coderabbit.yaml`.

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

- Seed data in `migrations/` (without discussion)
- The migration files directly — schema changes go through new versioned migrations
- Port configuration (9000)
- `rust-toolchain.toml` toolchain version
- The surrogate/natural key design (UUID for GET, squad number for PUT/DELETE)

### Creating Issues

This project uses Spec-Driven Development (SDD): discuss in Plan mode first, create a GitHub Issue as the spec artifact, then implement. Always offer to draft an issue before writing code. Issue templates live in the `create-issue` skill.

### Key workflows

**Add an endpoint**: Add route handler in `src/routes/players.rs` → add service function in `src/services/player_service.rs` with `Result<T, CustomError>` return → add repository function in `src/repositories/player_repository.rs` using Diesel DSL → add integration test in `tests/` following naming convention → update doc comments → run pre-commit checks.

**After completing work**: Suggest a branch name (e.g. `feat/add-player-stats`) and a commit message following Conventional Commits including co-author line:

```text
feat(scope): description (#issue)

Co-Authored-By: <the line Claude Code provides for the current model>
```

## Invariants (never change without explicit discussion)

- Port: 9000
- API contract: endpoints, HTTP status codes, and response shapes are fixed;
  do not change them without explicit discussion
- Commit format: `type(scope): description (#issue)` — max 80 chars
- Conventional Commits types: feat fix chore docs test refactor ci perf
- CHANGELOG.md `[Unreleased]` section must be updated before every commit

## Architecture Decision Records

Architectural decisions are documented in [`docs/adr/`](docs/adr/README.md). When proposing structural
changes, check the ADR index first. When a decision changes, mark the existing
ADR as `Deprecated` or `Superseded by ADR-XXXX` and write a new one.

## Claude Code

- Run `/pre-commit` to execute the full pre-commit checklist for this project.
