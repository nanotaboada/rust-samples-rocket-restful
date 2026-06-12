# ADR-0001: Adopt Rocket as REST API Framework

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a Rust web framework to implement a CRUD REST API for
football players. The Rust web ecosystem offers several mature options:

- **Axum** — tower-based, ergonomic, strong ecosystem momentum, handler
  functions are plain async `fn`; routing is composed programmatically.
- **Actix-web** — actor-model roots, high raw throughput benchmarks, but
  introduces more concepts (actors, `Data<T>` extractors) than a simple CRUD
  service needs.
- **Warp** — filter combinators compose routes; elegant but the type signature
  explosion in complex routes makes compiler errors hard to read.
- **Poem** — OpenAPI-first, macro-driven; less community adoption at the time
  of evaluation.
- **Rocket 0.5** — attribute-based routing (`#[get]`, `#[post]`, `#[put]`,
  `#[delete]`), `#[launch]` macro minimises `main.rs` boilerplate, async
  support via Tokio, strong type-safety for extractors (guards) and managed
  state.

## Decision

We will use Rocket 0.5.1 with async support via Tokio as the HTTP framework.

## Consequences

- **Positive**: Attribute macros on handler functions keep routes declarative
  and close to HTTP semantics; handler signatures self-document which HTTP verb
  and path they handle. `#[launch]` reduces boilerplate in `main.rs` to a
  single annotated function. Rocket's `FromRequest` guard trait provides
  type-safe, composable request extraction with clear error paths.
- **Negative**: Rocket has less ecosystem momentum than Axum as of 2025;
  community middleware and integrations are fewer. Rocket requires `nightly`
  prior to 0.5 — 0.5 finally stabilised on stable, but some documentation
  examples still reference nightly features.
- **Neutral**: Async runtime is Tokio; Diesel's synchronous connection pool
  (r2d2) is used inside `spawn_blocking` wrappers implicitly via Rocket's
  managed state and is not a source of runtime conflicts.
