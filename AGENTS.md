# Agent Instructions

## Project Overview

Educational REST API demonstrating Rocket web framework with layered architecture. Manages Argentina national football squad data with in-memory storage. Three-layer design (Routes → Services → State) prepares for future enhancements like caching, validation, and database persistence while teaching clean architecture principles.

## Structure

```tree
/src                - application code
  /models           - data structures (Player, PlayerRequest, PlayerResponse)
  /routes           - HTTP handlers/controllers
  /services         - business logic layer
  /state            - data access layer
/tests              - integration tests
/players.json       - seed data (26 players)
/.github            - CI/CD workflows (rust.yml)
/Rocket.toml        - server configuration
/rust-toolchain.toml - Rust 2024 edition lock
```

## Architecture Rationale

**Layered vs Minimal**: This project uses three layers instead of minimal API style (routes directly accessing state) for educational purposes. The service layer will house future caching, validation, metrics, and complex business logic. This demonstrates enterprise patterns while keeping current CRUD operations simple.

**Layer Responsibilities**:

- **Routes**: HTTP-only concerns (parse requests, map to status codes)
- **Services**: Pure business logic (no HTTP knowledge, returns Result types)
- **State**: Thread-safe data access (Mutex<Vec<Player>>, future Repository pattern)

## Common Workflows

### Adding a new endpoint

1. Add route handler in `src/routes/players.rs` or new file
2. Add service function in `src/services/player_service.rs`
3. Add integration test in `tests/player_service_tests.rs`
4. Update doc comments (///) with HTTP details and examples

### Modifying player data structure

1. Update model in `src/models/player.rs`
2. Update affected service functions in `src/services/`
3. Update route handlers in `src/routes/`
4. Add/update tests with new fields
5. Update `players.json` seed data if needed

### Adding database persistence (planned)

1. Create `src/repositories/player_repository.rs`
2. Replace Mutex<Vec<Player>> with Repository trait
3. Update State layer to use repository
4. Add migration management (SQLx or Diesel)
5. Update tests to use test database

### Running tests

- All tests: `cargo test`
- Specific test: `cargo test test_request_get_players`
- With output: `cargo test -- --nocapture`
- CI validation: `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test`

### Pre-commit checks

1. `cargo fmt` - auto-format code (required)
2. `cargo clippy -- -D warnings` - must pass with zero warnings
3. `cargo test` - all tests must pass
4. `cargo build` - must compile successfully

## Autonomy Levels

### Proceed freely

- Route handlers and HTTP response mapping
- Service layer business logic
- Integration tests following AAA pattern
- Doc comments (///) on public items
- README and documentation updates
- Bug fixes in existing code

### Ask before changing

- Architecture (adding/removing layers)
- Dependencies in Cargo.toml
- CI/CD configuration (.github/workflows/)
- Docker setup (Dockerfile, docker-compose.yml)
- Rocket.toml server configuration
- rust-toolchain.toml

### Never modify

- `.env` files (if they exist)
- `players.json` seed data (without discussion)
- Production deployment configurations

## Configuration

**Server**: Rocket.toml defines address (0.0.0.0) and port (8000)
**Override**: Use environment variables (ROCKET_PORT=9000, ROCKET_ADDRESS=127.0.0.1)
**Toolchain**: Rust 2024 edition enforced via rust-toolchain.toml

## Testing Strategy

**Current**: Integration tests calling service layer directly (no HTTP mocking needed)
**Pattern**: Arrange/Act/Assert with section comments
**Fixtures**: Dedicated functions for test data (not hardcoded)
**Why service-level**: Tests business logic independently of HTTP, enables future endpoint tests with Rocket::local::blocking::Client

## Troubleshooting

### Port already in use

```bash
lsof -ti:8000 | xargs kill -9
```

### Compilation errors after dependency update

```bash
cargo clean && cargo build
```

### Rust toolchain mismatch

```bash
rustup update
rustup show  # Verify matches rust-toolchain.toml
```

### JSON validation

```bash
cat players.json | jq .
```

## API Testing

### Quick smoke test

```bash
# Health check
curl http://localhost:8000/health

# Get all players
curl http://localhost:8000/players

# Get by ID
curl http://localhost:8000/players/1
```

### CRUD operations

```bash
# Create (POST)
curl -X POST http://localhost:8000/players \
  -H "Content-Type: application/json" \
  -d '{"firstName":"Lionel","lastName":"Messi","club":"Inter Miami","nationality":"Argentina","dateOfBirth":"1987-06-24","squadNumber":10}'

# Update (PUT)
curl -X PUT http://localhost:8000/players/1 \
  -H "Content-Type: application/json" \
  -d '{"firstName":"Emiliano","lastName":"Martínez","club":"Aston Villa","nationality":"Argentina","dateOfBirth":"1992-09-02","squadNumber":23}'

# Delete
curl -X DELETE http://localhost:8000/players/1
```

## Important Constraints

- **In-memory storage**: Data resets on restart (intentional for simplicity)
- **Thread safety**: Mutex required for concurrent access
- **Squad number uniqueness**: Enforced at service layer (returns CreateError/UpdateError)
- **Auto-incrementing IDs**: Generated in-memory, not persistent
- **No authentication**: Educational PoC - not production-ready
