# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `Dockerfile` (multi-stage: `rust:1.88-slim-bookworm` builder → `debian:bookworm-slim` runtime), `compose.yaml`, `.dockerignore`, `scripts/entrypoint.sh`, and `scripts/healthcheck.sh` for container support (#40)
- `rest/players.rest` with sample HTTP requests for all endpoints, compatible with VS Code REST Client and JetBrains HTTP Client (#40)
- `humao.rest-client` recommendation added to `.vscode/extensions.json` (#40)
- `.github/workflows/rust-cd.yml`: tag-based CD workflow triggered on `v*.*.*-*` tags; validates Ballon d'Or nominee name, builds and tests with pinned toolchain `1.88.0`, publishes three Docker tags (`:semver`, `:nominee`, `:latest`) to GHCR, and creates a GitHub Release with commit changelog (#26)
- README: added **Releases** section with release naming convention, step-by-step create-a-release workflow, pre-release checklist, and Docker pull instructions (#26)
- CHANGELOG: added Ballon d'Or nominees list (A-Z) as release codenames (#26)

### Changed
- `PUT /players/squadnumber/{squad_number}` now returns `204 No Content` with no body on success (#63)
- `initialize_database()` reads `STORAGE_PATH` environment variable for the database path, falling back to `storage/players-sqlite3.db` (#40)
- README: added **Containers** section (`docker compose up/down`), fixed `Test the API` curl examples to use correct UUID and fixture data (#40)
- SQLite persistence via `rusqlite` (bundled feature); database stored at `storage/players-sqlite3.db`
- `initialize_database()` opens the committed pre-seeded DB, or creates and seeds it if absent
- `initialize_test_database()` opens an in-memory SQLite database for use in all test suites
- Rename `.claude/commands/precommit.md` to `pre-commit.md`; update `CLAUDE.md` reference from `/precommit` to `/pre-commit`
- `PlayerCollection` type alias changed from `Mutex<Vec<Player>>` to `Mutex<rusqlite::Connection>`
- All service functions now accept `&rusqlite::Connection` instead of `&[Player]` / `&mut Vec<Player>`
- All tests updated to use in-memory SQLite (`Connection::open_in_memory()`) instead of a `Vec<Player>` seed
- Removed `Player` internal storage struct (no longer needed; SQL rows map directly to `PlayerResponse`)
- Normalize player dataset: correct Fernández/Mac Allister/Messi team data, replace hardcoded random UUIDs with deterministic UUID v5 values (namespace FIFA_WORLD_CUP_QATAR_2022_ARGENTINA_SQUAD)
- Align CRUD test fixtures: Lo Celso (squad 27) for Create and Delete, Messi (squad 10) for Retrieve, Damián Martínez (squad 23) for Update
- `.github/workflows/rust.yml` renamed to `rust-ci.yml`; CI badge in README updated accordingly (#26)

## [0.1.0] - 2025-01-01

### Added
- Initial release: REST API for managing football players built with Rust and Rocket
- CRUD operations with in-memory thread-safe storage (`Mutex<Vec<Player>>`)
- Argentina 2022 World Cup squad seed data (26 players)
- Layered architecture: routes → services → state
- Integration tests following Arrange/Act/Assert pattern

---

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
