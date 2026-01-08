# 🧪 RESTful API with Rust and Rocket

[![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)

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
- 📦 **Single-file implementation** (easy to learn and understand)

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
│   └── main.rs            # Complete application (models, routes, handlers, state)
├── players.json           # Pre-seeded player data (26 players)
├── Cargo.toml             # Rust dependencies
└── rust-toolchain.toml    # Rust version configuration
```

## Architecture

**Single-File Layered Architecture:**

```text
HTTP Request → Route Handler → State (Mutex) → In-Memory Storage → Response
```

**Data Flow:**

- HTTP requests are received by Rocket route handlers
- Route handlers access shared state via `State<PlayerCollection>`
- Mutex ensures thread-safe access to the player collection
- Responses use dedicated DTOs (PlayerRequest/PlayerResponse)
- Internal storage uses the Player entity

**Layer Separation:**

- **Player** - Internal storage entity
- **PlayerRequest** - API input (`POST`/`PUT`, no ID)
- **PlayerResponse** - API output (includes ID)

This separation provides type safety and prevents accidental exposure of internal implementation details.

## API Reference

### Endpoints

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET` | `/` | API welcome message |
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
