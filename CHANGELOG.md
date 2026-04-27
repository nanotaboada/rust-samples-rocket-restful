# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Ballon d'Or Nominees 🏅

Release codenames follow an A-Z sequence using Ballon d'Or award nominees surnames:

| # | Tag Name | Player | Country | Notable Year |
|---|----------|--------|---------|--------------|
| A | `aguero` | Sergio Agüero | Argentina | 2011 |
| B | `benzema` | Karim Benzema | France | 2022 (Winner) |
| C | `cannavaro` | Fabio Cannavaro | Italy | 2006 (Winner) |
| D | `drogba` | Didier Drogba | Ivory Coast | 2010 |
| E | `etoo` | Samuel Eto'o | Cameroon | 2005, 2006 |
| F | `figo` | Luís Figo | Portugal | 2000 (Winner) |
| G | `griezmann` | Antoine Griezmann | France | 2016, 2018 |
| H | `haaland` | Erling Haaland | Norway | 2023 |
| I | `iniesta` | Andrés Iniesta | Spain | 2010, 2012 |
| J | `jorginho` | Jorginho | Italy | 2021 |
| K | `kaka` | Kaká | Brazil | 2007 (Winner) |
| L | `lewandowski` | Robert Lewandowski | Poland | 2020, 2021 |
| M | `messi` | Lionel Messi | Argentina | 8× Winner |
| N | `neymar` | Neymar | Brazil | 2015, 2017 |
| O | `owen` | Michael Owen | England | 2001 (Winner) |
| P | `pirlo` | Andrea Pirlo | Italy | 2007, 2011 |
| Q | `quaresma` | Ricardo Quaresma | Portugal | 2008 |
| R | `ronaldo` | Cristiano Ronaldo | Portugal | 5× Winner |
| S | `salah` | Mohamed Salah | Egypt | 2018, 2019 |
| T | `torres` | Fernando Torres | Spain | 2008 |
| U | `umtiti` | Samuel Umtiti | France | 2018 |
| V | `vandijk` | Virgil van Dijk | Netherlands | 2019 |
| W | `weah` | George Weah | Liberia | 1995 (Winner) |
| X | `xavi` | Xavi | Spain | 2009, 2010, 2011 |
| Y | `yayatoure` | Yaya Touré | Ivory Coast | 2011, 2013 |
| Z | `zlatan` | Zlatan Ibrahimović | Sweden | 2013, 2015 |

---

## [Unreleased]

### Added

- Field-level validation on `PlayerRequest` payloads using the `validator` crate (`#97`): all required string fields (`first_name`, `last_name`, `date_of_birth`, `position`, `abbr_position`, `team`, `league`) must be non-empty; `squad_number` must be in the range 1–99
- Six new route-level integration tests covering validation failure scenarios for POST and PUT endpoints (`#97`)

### Changed

- POST `/players` and PUT `/players/squadnumber/{squad_number}` now return `422 Unprocessable Entity` for field validation failures; `400 Bad Request` is reserved for malformed JSON / wrong `Content-Type` (`#94`, `#97`)

### Fixed

### Removed

---

## [1.1.1 - Cannavaro] - 2026-04-12

### Added

- CodeQL Advanced workflow (`.github/workflows/codeql.yml`) for static security analysis on push, pull request, and weekly schedule; covers `actions` and `rust` languages (#77)
- Coverage reporting with `cargo-tarpaulin` and Codecov integration (#42)
- `codecov.yml` with 80% minimum threshold on `src/services/`, `src/routes/`, `src/repositories/` (#42)
- Codecov badge in `README.md` (#42)

### Changed

- Consolidated `commitlint`, `format`, and `lint` CI jobs into a single `lint` job (#42)
- `[profile.release]` tuned in `Cargo.toml` with `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"` for smaller binary size (#60)
- `Dockerfile` builder stage updated to compile against `x86_64-unknown-linux-musl` via `musl-tools` for a fully static binary; runtime stage switched from `debian:bookworm-slim` to `alpine` (#57)
- `get_all`, `get_by_id`, `get_by_squad_number`, and `delete` in `player_service` now return `Result<T, PlayerServiceError>` instead of `Result<T, diesel::result::Error>`, aligning with the `CreateError`/`UpdateError` pattern (#56)
- `codecov.yml` `ignore` list extended with `src/**/mod.rs` to exclude module re-exports from coverage reporting (#78)
- `codecov.yml` comment updated to reflect goal of maximum coverage on business logic layers (#78)

### Fixed

- API Reference table in `README.md`: corrected `PUT /players/squadnumber/:squadnumber` response from `200 OK` to `204 No Content`

### Removed

- Pre-seeded `storage/players-sqlite3.db` file and `storage/` directory from repository; Diesel `embed_migrations!()` initialises and seeds the database on first start (#79)
- `storage/` and `.envrc` added to `.gitignore` to prevent runtime database files and local environment secrets from being tracked (#79)

---

## [1.1.0 - Benzema] - 2026-04-08

### Added

- Diesel ORM (SQLite backend, r2d2 connection pool) replacing `rusqlite` (#64)
- Three versioned migrations under `migrations/`: DDL (`20260407000001_create_players`), starting XI seed (`20260407000002_seed_starting_xi`), substitutes seed (`20260407000003_seed_substitutes`) (#64)
- `src/schema.rs` generated from the `players` table DDL (#64)
- `src/repositories/player_repository.rs` owning all Diesel DSL queries; services contain no SQL (#64)
- `Player` (Queryable/Selectable) and `NewPlayer` (Insertable) structs in `src/models/player.rs` (#64)
- `UNKNOWN_PLAYER_ID` constant in `tests/common/mod.rs` for valid-UUID-absent-from-DB 404 scenarios (#69)
- `unknown` test cases for GET `/players/{uuid}`, PUT and DELETE `/players/squadnumber/{n}` in `player_routes_tests.rs` (#69)
- `unknown` test case for `get_by_id` at the service layer in `player_service_tests.rs` (#69)
- `///` doc comments on all three player ID constants in `tests/common/mod.rs` documenting the `existing` / `nonexistent` / `unknown` vocabulary (#69)

### Changed

- `diesel` and `diesel_migrations` minimum version tightened from `"2.2"` to `"2.3"` (#64)
- `PlayerCollection` type changed from `Mutex<Connection>` to `r2d2::Pool<ConnectionManager<SqliteConnection>>` (#64)
- `src/state/player_collection.rs` now runs pending migrations via `embed_migrations!()` on startup; seed data moved from Rust code to SQL migration files (#64)
- `src/services/player_service.rs` delegates all persistence to the repository layer; error types use `diesel::result::Error` (#64)
- All route handlers updated to use `pool.get()` instead of `mutex.lock()` (#64)
- Four-layer architecture: `Routes → Services → Repository → State` (#64)
- All integration tests updated to obtain connections from the pool via `.get()` (#64)
- `README.md`: updated Tech Stack, Features, Docker notes, and Mermaid dependency diagram to reflect new architecture (#64)
- `.github/copilot-instructions.md`: updated Overview, Tech Stack, Structure, Layer rule, Test fixtures, Never modify, and Add an endpoint workflow (#64)
- `Dockerfile`: added `COPY migrations/` to builder stage; removed pre-seeded `hold/` copy from runtime stage (#64)
- `scripts/entrypoint.sh`: simplified — `hold/` seed copy logic removed; migrations handle first-run initialization (#64)
- `Cargo.toml` version aligned with release tag convention: `0.2.0` → `1.1.0` (#64)

### Fixed

- `service::update` now returns `UpdateError::NotFound` when `repository::update` affects zero rows, closing a TOCTOU gap where a deleted row between the existence check and the update would fabricate a success response (#64)

### Removed

- `rusqlite` dependency replaced by `diesel` + `diesel_migrations` + `libsqlite3-sys` (bundled) (#64)
- Hand-written `create_schema`, `seed`, and `is_empty` functions in `player_collection.rs` (#64)

---

## [1.0.0 - Agüero] - 2026-04-06

Initial release. See [README.md](README.md) for complete feature list and documentation.

---

<!-- Template for new releases:

## [X.Y.Z - SURNAME] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security vulnerability fixes

-->

[unreleased]: https://github.com/nanotaboada/rust-samples-rocket-restful/compare/v1.1.1-cannavaro...HEAD
[1.1.1 - Cannavaro]: https://github.com/nanotaboada/rust-samples-rocket-restful/compare/v1.1.0-benzema...v1.1.1-cannavaro
[1.1.0 - Benzema]: https://github.com/nanotaboada/rust-samples-rocket-restful/compare/v1.0.0-aguero...v1.1.0-benzema
[1.0.0 - Agüero]: https://github.com/nanotaboada/rust-samples-rocket-restful/releases/tag/v1.0.0-aguero
