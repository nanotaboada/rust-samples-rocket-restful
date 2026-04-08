//! Library crate root for `rust-samples-rocket-restful`.
//!
//! Exposes the application's modules as a library so they can be imported by
//! integration tests in the `tests/` directory without duplicating code.
//!
//! ## Rust note: binary vs library crates
//! Rust allows a project to have both `main.rs` (binary crate — produces an
//! executable) and `lib.rs` (library crate — exposes reusable modules). The
//! binary imports from the library, and integration tests target the library
//! directly. This is the standard Rocket testing pattern: tests instantiate
//! the app via the library without starting a real HTTP server.

pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;
pub mod services;
pub mod state;
