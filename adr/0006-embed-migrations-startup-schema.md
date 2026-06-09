# ADR-0006: Embedded Migrations at Startup

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a schema management strategy for the SQLite database. Options
considered:

- **Manual SQL scripts** — `schema.sql` applied by an operator or CI step
  before the server starts. No version tracking; prone to drift between
  environments.
- **Runtime migration file loading** — Diesel reads migration files from the
  filesystem at startup. Requires the `migrations/` directory to be present at
  the deployment path; complicates Docker images and CI environments.
- **No migrations** — schema is created on first run from inline DDL. No
  history, no incremental schema evolution, hard to coordinate with seed data.
- **`embed_migrations!()`** — Diesel compiles all migration files into the
  binary at build time. At startup, `embedded_migrations::run(&conn)` applies
  any pending migrations before the first request is served.

## Decision

We will use Diesel's `embed_migrations!()` macro in `src/state/player_collection.rs`.
All pending migrations are applied at startup before the connection pool is
handed to Rocket's managed state.

## Consequences

- **Positive**: Migrations are baked into the binary — no external files
  required at runtime. The server fails fast at startup if a migration cannot
  be applied, preventing requests from reaching a partially-migrated schema.
  Behaviour is identical in local development, CI, and Docker without extra
  configuration. The seed data migration (`up.sql` in the second migration)
  runs automatically in a fresh environment.
- **Negative**: Binary size increases slightly because SQL migration content is
  embedded as static strings. Adding a new migration requires rebuilding and
  redeploying the binary rather than running a migration tool against a live
  database.
- **Neutral**: The `diesel_migrations` crate provides idempotent migration
  tracking via a `__diesel_schema_migrations` table; re-running the binary on
  an already-migrated database is safe.
