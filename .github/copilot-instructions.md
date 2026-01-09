# GitHub Copilot Instructions

This file provides context and guidelines for GitHub Copilot when working on this project.

## Project Overview

**rust-samples-rocket-restful** is a Proof of Concept RESTful API built with Rust and Rocket framework. It demonstrates CRUD operations for managing football player data using thread-safe in-memory storage.

### Key Technologies

- **Language**: Rust 2024 Edition (1.88.0)
- **Web Framework**: Rocket 0.5.1
- **Serialization**: Serde + serde_json
- **Storage**: In-memory (`Mutex<Vec<Player>>`)

## Architecture & Design Patterns

### Single-File PoC Approach

The project intentionally uses a single-file architecture (`src/main.rs`) for simplicity and learning purposes. This follows the philosophy of "start small" before introducing complexity.

**Current state**: All code in `main.rs` (models, routes, handlers, state)
**Future enhancement**: Can be refactored into modules (`models.rs`, `routes.rs`, `data.rs`)

### Layer Separation

The project enforces separation between internal storage and external API contracts:

- **`Player`** - Internal storage entity (not exposed directly)
- **`PlayerRequest`** - API input for POST/PUT (no ID field)
- **`PlayerResponse`** - API output (includes ID)

**When suggesting code**: Always maintain this separation. Never expose `Player` directly in route handlers.

### Thread Safety

All state access uses `Mutex<Vec<Player>>` for thread-safe operations. When adding new routes:

- Always acquire lock: `players.lock().unwrap()`
- Keep critical sections short
- Release lock by letting it go out of scope

## Code Style & Conventions

### Naming Conventions

- **Types**: PascalCase (`Player`, `PlayerRequest`)
- **Functions**: snake_case (`load_players`, `get_player_by_id`)
- **Constants**: SCREAMING_SNAKE_CASE (if needed)
- **Variables**: snake_case (`new_id`, `player_request`)

### Branch Naming

Follow [Conventional Branch](https://conventional-branch.github.io/) specification, aligned with [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):
- `fix/<description>` - Bug fixes
- `chore/<description>` - Maintenance tasks
- `ci/<description>` - CI/CD changes
- `docs/<description>` - Documentation updates
- `refactor/<description>` - Code refactoring
- `test/<description>` - Adding/updating tests

**Examples**: `feat/15-players-crud-api`, `ci/separate-workflow-jobs`, `docs/update-readme`

**Rationale**: This project uses a simplified workflow (single `master` branch) without full Gitflow. Branch names align with conventional commits for consistency.

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):
- Header: Max 80 characters
- Body: Max 80 characters per line, blank line after header
- Footer: Reference issues (e.g., `Closes #15`)

Enforced via commitlint in CI pipeline.

### Error Handling

- Use `Result<T, Status>` for fallible operations
- Return appropriate HTTP status codes:
  - `200 OK` - Successful GET/PUT
  - `201 Created` - Successful POST
  - `204 No Content` - Successful DELETE
  - `404 Not Found` - Resource not found
  - `409 Conflict` - Duplicate/constraint violation

### Serde Configuration

Always use `#[serde(rename_all = "camelCase")]` for JSON serialization to match the `players.json` data format.

## Business Rules

### Squad Number Uniqueness

Squad numbers must be **globally unique** across the entire player collection (not team-scoped). This represents a single squad roster (e.g., national team).

**When implementing validation**: Check for duplicates across all players, not just within a team.

### ID Generation

New player IDs are auto-generated using `max(existing_ids) + 1`. Never accept ID in POST requests.

## Common Patterns

### Route Handler Template

```rust
#[get("/players/<id>")]
fn get_player_by_id(
    id: u32,
    players: &State<PlayerCollection>
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().unwrap();
    players
        .iter()
        .find(|p| p.id == id)
        .map(|p| Json(PlayerResponse::from(p.clone())))
        .ok_or(Status::NotFound)
}
```

### Validation Pattern

```rust
// Check for duplicates before insert/update
if players.iter().any(|p| p.squad_number == new_squad_number) {
    return Err(Status::Conflict);
}
```

## Testing Guidelines

- Tests should be added to `src/main.rs` using `#[cfg(test)]` module
- Use Rocket's testing framework for integration tests
- Test all CRUD operations and edge cases
- Verify proper HTTP status codes

## CI/CD

The project uses GitHub Actions with sequential job execution (fail-fast):

1. **Format**: `cargo fmt --all -- --check` (runs first)
2. **Lint**: `cargo clippy --all-targets --all-features -- -D warnings` (requires format to pass)
3. **Build**: `cargo build --verbose` (requires lint to pass)
4. **Test**: `cargo test --verbose` (requires build to pass)

Jobs use `needs` dependencies to ensure logical flow. If format fails, subsequent jobs are skipped.

**Before committing**: Run `cargo fmt`, `cargo clippy`, and `cargo test` locally.

## Configuration

### Rocket Configuration

- Server runs on `http://127.0.0.1:9000` (localhost only)
- Configuration in `Rocket.toml` with separate profiles
- **Security**: Always bind to localhost (`127.0.0.1`) for development

### Data Loading

- `players.json` must exist in the working directory
- Application panics on startup if file is missing (fail-fast)
- Run with `cargo run` from project root

## Future Enhancements (Not Yet Implemented)

- SQLite persistence (to replace in-memory storage)
- Modular file structure
- Docker containerization
- Swagger/OpenAPI documentation
- Test coverage
- Authentication/Authorization

**When suggesting features**: Keep changes incremental and focused. Don't over-engineer solutions.

## Philosophy

This project follows the Linus Torvalds philosophy of starting small and solving immediate needs. Suggestions should:

- Be simple and practical
- Solve real problems (not hypothetical ones)
- Avoid premature abstraction
- Maintain the learning-focused nature of the PoC

## References

- [Rocket Documentation](https://rocket.rs/)
- [Serde Documentation](https://serde.rs/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
