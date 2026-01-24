# AGENTS.md

> **⚡ Token Efficiency Note**: This file contains complete operational instructions (~2,500 tokens).  
> **Auto-loaded**: NO (load explicitly with `#file:AGENTS.md` when you need detailed procedures)  
> **When to load**: Complex workflows, troubleshooting, CI/CD setup, detailed architecture questions  
> **Related files**: See `#file:.github/copilot-instructions.md` for quick context (auto-loaded, ~500 tokens)

---

## Quick Start

```bash
# Build and run in development mode
cargo run
# Server starts on http://localhost:8000

# Build release version
cargo build --release

# Run release binary
./target/release/rust-samples-rocket-restful
```

## Rust Version

This project uses **Rust 2024 Edition** with toolchain specified in `rust-toolchain.toml`.

Rust will automatically use the correct version when you enter the project directory.

## Development Workflow

### Running Tests

```bash
# Run all tests with verbose output
cargo test -- --nocapture

# Run tests without output capture (shows print statements)
cargo test -- --show-output

# Run specific test
cargo test test_get_players -- --nocapture

# Run tests with release optimizations
cargo test --release
```

**Note**: This project uses in-memory storage (Mutex<Vec<Player>>), so tests are currently integration-style endpoint tests.

### Code Quality

```bash
# Check code without building (fast feedback)
cargo check

# Format code (auto-fix, must run before commit)
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run linter (clippy)
cargo clippy

# Run clippy with warnings as errors (matches CI)
cargo clippy -- -D warnings
```

**Pre-commit checklist**:
1. Run `cargo fmt` - auto-format all code
2. Run `cargo clippy -- -D warnings` - must pass with no warnings
3. Run `cargo test` - all tests must pass
4. Run `cargo build` - must compile successfully

### Build Variants

```bash
# Development build (fast compile, slower runtime)
cargo build

# Release build (slow compile, fast runtime)
cargo build --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

## Architecture

**Single-File Architecture**: All code lives in `src/main.rs` for simplicity.

**Layers within main.rs**:
```rust
// Data Models
struct Player              // Internal storage entity (with id)
struct PlayerRequest       // API input (no id)
struct PlayerResponse      // API output (with id)

// Application State
struct PlayerCollection    // Mutex<Vec<Player>> for thread safety

// Route Handlers
#[get("/")] index          // Welcome message
#[get("/health")] health   // Health check
#[get("/players")] get_players
#[get("/players/<id>")] get_player_by_id
#[post("/players")] create_player
#[put("/players/<id>")] update_player
#[delete("/players/<id>")] delete_player

// Main
#[launch] rocket           // Rocket initialization with state
```

**Data Flow**:
```
HTTP Request → Rocket Router → Handler Function → Mutex<State> → Response
```

**Key Design Decisions**:
- In-memory storage (no database) - data resets on restart
- Mutex ensures thread-safe concurrent access
- Separate DTOs for requests/responses (type safety)
- JSON serialization via Serde
- Auto-incrementing IDs generated in-memory

## Configuration

### Rocket.toml

```toml
[default]
address = "0.0.0.0"
port = 8000
```

You can override settings with environment variables:
```bash
ROCKET_PORT=9000 cargo run
ROCKET_ADDRESS=127.0.0.1 cargo run
```

## Data Seeding

**Seed data**: `players.json` - Contains 26 pre-configured football players (A-Z)

Data loads on application startup from this JSON file into in-memory storage.

**To modify seed data**: Edit `players.json` and restart the server.

## CI/CD Pipeline

### Continuous Integration (rust.yml)

**Trigger**: Push to `main`/`master` or PR

**Jobs**:
1. **Setup**: Rust toolchain installation (from rust-toolchain.toml)
2. **Format Check**: `cargo fmt -- --check`
3. **Lint**: `cargo clippy -- -D warnings`
4. **Build**: `cargo build --verbose`
5. **Test**: `cargo test --verbose`

**Local validation** (run this before pushing):
```bash
# Matches CI exactly
cargo fmt -- --check && \
cargo clippy -- -D warnings && \
cargo build --verbose && \
cargo test --verbose
```

**Quick pre-commit validation**:
```bash
# Auto-fix formatting, then validate
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Troubleshooting

### Port already in use
```bash
# Kill process on port 8000
lsof -ti:8000 | xargs kill -9
```

### Compilation errors after update
```bash
# Clean and rebuild
cargo clean
cargo build
```

### Dependency conflicts
```bash
# Update Cargo.lock
cargo update

# Or reset to exact versions
rm Cargo.lock
cargo build
```

### Rust toolchain issues
```bash
# Update Rust toolchain
rustup update

# Verify toolchain matches rust-toolchain.toml
rustup show
```

### JSON parsing errors
```bash
# Validate players.json syntax
cat players.json | jq .

# If jq not installed, check manually or use online JSON validator
```

## Testing the API

### Using curl
```bash
# Health check
curl http://localhost:8000/health

# Get all players
curl http://localhost:8000/players

# Get player by ID
curl http://localhost:8000/players/1

# Create player (ID auto-generated)
curl -X POST http://localhost:8000/players \
  -H "Content-Type: application/json" \
  -d '{"firstName":"Pele","lastName":"Nascimento","club":"Santos","nationality":"Brazil","dateOfBirth":"1940-10-23","squadNumber":10}'

# Update player
curl -X PUT http://localhost:8000/players/1 \
  -H "Content-Type: application/json" \
  -d '{"firstName":"Diego","lastName":"Maradona","club":"Napoli","nationality":"Argentina","dateOfBirth":"1960-10-30","squadNumber":10}'

# Delete player
curl -X DELETE http://localhost:8000/players/1
```

### Response Formats

**Success responses**:
- `200 OK` - Successful GET/PUT/DELETE
- `201 Created` - Successful POST

**Error responses**:
- `404 Not Found` - Player ID doesn't exist
- `400 Bad Request` - Invalid JSON or duplicate squad number

## Important Notes

- **In-memory storage**: All data is lost when the server restarts
- **Thread safety**: Mutex protects concurrent access to player collection
- **Squad number uniqueness**: Enforced - duplicate squad numbers are rejected
- **No database**: This is intentional for simplicity - production apps should use persistent storage
- **Single file**: All code in `src/main.rs` for learning clarity
- **Auto-incrementing IDs**: Generated in-memory, not guaranteed unique across restarts
- **Cargo.lock**: Committed to ensure reproducible builds
- **Rust edition**: Uses 2024 edition features
