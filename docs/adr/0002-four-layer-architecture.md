# ADR-0002: Four-Layer Architecture

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a module structure that keeps HTTP concerns, business logic,
and data access clearly separated. Options considered:

- **Two-layer (routes + data)** — common in small services; routes contain
  business logic and query the database directly. Fast to write, hard to test
  independently, and business rules scatter across handler functions.
- **Hexagonal / ports-and-adapters** — clean boundaries via traits and
  adapters; strong theoretical separation but adds indirection (trait objects,
  mock adapters) that is disproportionate for a single-resource CRUD service.
- **Flat module structure** — everything in one module or one file per concern;
  suitable for trivial CLIs, not for a multi-endpoint API that should be
  independently maintainable.
- **Four-layer (Routes → Services → Repositories → State)** — each module owns
  a single, named concern; layer skipping is prohibited by code review
  convention and is naturally discouraged by Rust's module privacy rules.

## Decision

We will use a strict four-layer architecture:

```text
Routes → Services → Repositories → State
```

- **Routes** (`src/routes/`): async Rocket handlers; HTTP concerns only
  (request parsing, response mapping). No business logic, no Diesel imports.
- **Services** (`src/services/`): pure business logic; returns `Result<T, CustomError>`;
  no `rocket` or `diesel` imports.
- **Repositories** (`src/repositories/`): all Diesel DSL queries; no HTTP
  knowledge.
- **State** (`src/state/`): r2d2 connection pool initialisation; runs
  `embed_migrations!()` at startup.

## Consequences

- **Positive**: Each layer can be reasoned about and tested in isolation.
  Business logic in services has no HTTP or SQL surface area, making it the
  easiest layer to unit-test if needed. Repository functions group all SQL in
  one place, making schema migrations' impact immediately visible.
- **Negative**: A simple field rename touches four files (model, repository,
  service, route). There is more boilerplate than a two-layer approach.
- **Neutral**: Rust's module system does not enforce layer boundaries at compile
  time, but the convention is clear enough that accidental layer skipping is
  caught in code review.
