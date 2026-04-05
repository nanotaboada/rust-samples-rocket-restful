# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- SQLite persistence via `rusqlite` (bundled feature); database stored at `storage/players-sqlite3.db`
- `initialize_database()` opens the committed pre-seeded DB, or creates and seeds it if absent
- `initialize_test_database()` opens an in-memory SQLite database for use in all test suites

### Changed
- Rename `.claude/commands/precommit.md` to `pre-commit.md`; update `CLAUDE.md` reference from `/precommit` to `/pre-commit`
- `PlayerCollection` type alias changed from `Mutex<Vec<Player>>` to `Mutex<rusqlite::Connection>`
- All service functions now accept `&rusqlite::Connection` instead of `&[Player]` / `&mut Vec<Player>`
- All tests updated to use in-memory SQLite (`Connection::open_in_memory()`) instead of a `Vec<Player>` seed
- Removed `Player` internal storage struct (no longer needed; SQL rows map directly to `PlayerResponse`)
- Normalize player dataset: correct Fernández/Mac Allister/Messi team data, replace hardcoded random UUIDs with deterministic UUID v5 values (namespace FIFA_WORLD_CUP_QATAR_2022_ARGENTINA_SQUAD)
- Align CRUD test fixtures: Lo Celso (squad 27) for Create and Delete, Messi (squad 10) for Retrieve, Damián Martínez (squad 23) for Update

## [0.1.0] - 2025-01-01

### Added
- Initial release: REST API for managing football players built with Rust and Rocket
- CRUD operations with in-memory thread-safe storage (`Mutex<Vec<Player>>`)
- Argentina 2022 World Cup squad seed data (26 players)
- Layered architecture: routes → services → state
- Integration tests following Arrange/Act/Assert pattern
