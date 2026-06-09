# ADR-0007: Integration-Only Test Strategy

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a testing strategy that provides meaningful coverage without
excessive setup cost. Options considered:

- **Unit tests with mocked services** — each layer tested in isolation using
  mock implementations; highest granularity, but mocks must be kept in sync
  with real implementations and may diverge silently.
- **Unit tests + integration tests** — full coverage pyramid; appropriate for
  large systems, but for a single-resource CRUD API the unit/integration split
  doubles the maintenance surface without proportional benefit.
- **TestContainers with PostgreSQL** — identical to production DB engine, but
  adds container orchestration overhead, increases CI startup time, and moves
  away from the SQLite storage decision.
- **Integration tests against in-memory SQLite** — Rocket's `local::Client`
  drives full HTTP-level requests; an in-memory pool with `max_size(1)` gives
  real schema enforcement and constraint checking with near-zero setup cost.

## Decision

We will write integration tests exclusively in the `tests/` directory. Each
test uses either `initialize_test_database()` (26-player seeded pool) or
`initialize_empty_test_database()` (schema only, no rows), calls the service or
issues HTTP requests via fixtures, and follows the Arrange/Act/Assert pattern
with `// Arrange`, `// Act`, `// Assert` section comments.

Test naming convention: `test_request_{method}_{endpoint}_{condition}_response_{verification}`.

No mocked services. No unit tests of individual functions.

## Consequences

- **Positive**: In-memory SQLite (`max_size(1)`) provides real schema and
  constraint enforcement with millisecond setup time. Tests are isolated without
  any cleanup step — each test gets its own in-memory database. Rocket's
  `local::Client` tests the full HTTP stack, catching route and serialisation
  issues that a service-only unit test would miss.
- **Negative**: Individual service functions and repository queries have no
  dedicated unit test coverage. A bug in a repository function is only caught by
  a test that exercises the full route → service → repository path, which can
  make root-cause analysis slower.
- **Neutral**: The in-memory pool uses `max_size(1)` to prevent connection
  contention in single-threaded test runs. This is not a production pool
  configuration.
