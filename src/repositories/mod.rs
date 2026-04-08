//! Data access layer using Diesel DSL.
//!
//! This module owns all database interactions. Services delegate persistence
//! here and must not import Diesel or reference SQL directly.

pub mod player_repository;
