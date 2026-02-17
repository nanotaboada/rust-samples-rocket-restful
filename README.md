# 🧪 RESTful API with Rust and Rocket

[![Rust CI](https://github.com/nanotaboada/rust-samples-rocket-restful/actions/workflows/rust.yml/badge.svg)](https://github.com/nanotaboada/rust-samples-rocket-restful/actions/workflows/rust.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)

Proof of Concept for a RESTful API built with [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs/). Manage football player data with thread-safe in-memory storage using Mutex.

## Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [API Reference](#api-reference)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Testing](#testing)
- [Command Summary](#command-summary)
- [Contributing](#contributing)
- [Legal](#legal)

## Features

- 🔌 **RESTful CRUD operations** for football player data
- 🩺 **Health check endpoint** for monitoring
- 🔒 **Thread-safe state management** with Mutex
- ✅ **Type-safe request/response models**
- 🎯 **Squad number uniqueness validation**
- 📦 **Modular architecture** with clear separation of concerns

## Tech Stack

| Category | Technology |
| -------- | ---------- |
| **Language** | [Rust 2024 Edition](https://www.rust-lang.org/) |
| **Web Framework** | [Rocket 0.5.1](https://rocket.rs/) |
| **Serialization** | [Serde](https://serde.rs/) |
| **Storage** | In-memory (`Mutex<Vec<Player>>`) |

## Project Structure

```tree
/
├── src/
│   ├── main.rs                      # Application entry point (~35 lines)
│   ├── models/
│   │   ├── mod.rs                   # Module exports
│   │   └── player.rs                # Player, PlayerRequest, PlayerResponse structs
│   ├── routes/
│   │   ├── mod.rs                   # Module exports
│   │   ├── health.rs                # Health check endpoint handler
│   │   └── players.rs               # Player CRUD route handlers
│   ├── services/
│   │   ├── mod.rs                   # Module exports
│   │   └── player_service.rs        # Business logic (CRUD operations, validation)
│   └── state/
│       ├── mod.rs                   # Module exports
│       └── player_collection.rs     # Thread-safe state (Mutex<Vec<Player>>)
├── players.json                     # Pre-seeded player data (26 players)
├── Cargo.toml                       # Rust dependencies
└── rust-toolchain.toml              # Rust version configuration
```

### Module Responsibilities

| Module | Responsibility |
| ------ | -------------- |
| **models** | Data structures for the player domain (Player, PlayerRequest, PlayerResponse) and conversions between them |
| **state** | Thread-safe application state management using Mutex for concurrent access |
| **services** | Pure business logic functions for CRUD operations, validation, and ID generation |
| **routes** | HTTP endpoint handlers that delegate to services and handle HTTP concerns (status codes, JSON) |
| **main.rs** | Application initialization, route mounting, and data loading |

## Architecture

**Modular Layered Architecture:**

```text
HTTP Request → Routes → Services → State (Mutex) → In-Memory Storage → Response
```

**Dependency Flow:**

- Routes → Services → State (unidirectional)
- Services contain pure business logic, framework-agnostic
- Routes handle HTTP concerns (status codes, JSON serialization)
- State management is isolated from business logic

**Data Flow:**

- HTTP requests are received by Rocket route handlers in `routes/`
- Route handlers acquire locks and delegate to pure functions in `services/`
- Services perform business logic (validation, CRUD) on borrowed data
- Services return Results that routes convert to HTTP responses
- Thread-safe state access via `Mutex<Vec<Player>>`

**Type Safety:**

- **Player** - Internal storage entity (in `models/player.rs`)
- **PlayerRequest** - API input for `POST`/`PUT` (no ID, system-generated)
- **PlayerResponse** - API output (includes ID)

This separation provides type safety, testability, and prevents accidental exposure of internal implementation details.

## API Reference

### Endpoints

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET` | `/health` | Health check |
| `GET` | `/players` | List all players |
| `GET` | `/players/:id` | Get player by ID |
| `GET` | `/players/squadnumber/:squadnumber` | Get player by squad number |
| `POST` | `/players` | Create new player |
| `PUT` | `/players/:id` | Update player |
| `DELETE` | `/players/:id` | Remove player |

### Response Codes

| Code | Description |
| ---- | ----------- |
| `200 OK` | Successful GET/PUT |
| `201 Created` | Successful POST |
| `204 No Content` | Successful DELETE |
| `404 Not Found` | Player not found |
| `409 Conflict` | Duplicate squad number |

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 2024 Edition or higher** (managed via `rust-toolchain.toml`)
- **Cargo** (comes with Rust)

## Quick Start

### Clone the repository

```bash
git clone https://github.com/nanotaboada/rust-samples-rocket-restful.git
cd rust-samples-rocket-restful
```

### Install dependencies

```bash
cargo build
```

### Start the development server

```bash
cargo run
```

The server will start on `http://localhost:9000`.

### Access the application

- **API:** `http://localhost:9000`
- **Health Check:** `http://localhost:9000/health`

### Test the API

```bash
# Get all players
curl http://localhost:9000/players

# Get player by ID
curl http://localhost:9000/players/1

# Get player by squad number
curl http://localhost:9000/players/squadnumber/10

# Create a new player
curl -X POST http://localhost:9000/players \
  -H "Content-Type: application/json" \
  -d '{
    "firstName": "Test First Name",
    "middleName": "",
    "lastName": "Test Last Name",
    "dateOfBirth": "2000-01-01",
    "squadNumber": 99,
    "position": "Test Position",
    "abbrPosition": "AP",
    "team": "Test Team",
    "league": "Test League",
    "starting11": false
  }'

# Update a player (requires full object)
curl -X PUT http://localhost:9000/players/1 \
  -H "Content-Type: application/json" \
  -d '{
    "firstName": "Emiliano",
    "middleName": "",
    "lastName": "Martínez",
    "dateOfBirth": "1992-09-02",
    "squadNumber": 23,
    "position": "Goalkeeper",
    "abbrPosition": "GK",
    "team": "Aston Villa",
    "league": "Premier League",
    "starting11": true
  }'

# Delete a player
curl -X DELETE http://localhost:9000/players/21
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests with detailed output
cargo test -- --show-output
```

## CI/CD

The project uses GitHub Actions with sequential job execution:

### Pipeline

Format → Lint → Build → Test

Each job depends on the previous one succeeding. This provides fail-fast feedback and saves CI resources.

## Command Summary

| Command | Description |
| ------- | ----------- |
| `cargo run` | Start development server |
| `cargo build` | Build the application |
| `cargo build --release` | Build optimized release version |
| `cargo test` | Run all tests |
| `cargo fmt` | Format code |
| `cargo clippy` | Run linter |
| `cargo clean` | Clean build artifacts |

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the code of conduct and the process for submitting pull requests.

**Key guidelines:**

- Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages
- Run `cargo fmt` and `cargo clippy` before committing
- Ensure all tests pass (`cargo test`)
- Keep changes small and focused

## Legal

This project is provided for educational and demonstration purposes and may be used in production environments at your discretion. All referenced trademarks, service marks, product names, company names, and logos are the property of their respective owners and are used solely for identification or illustrative purposes.
