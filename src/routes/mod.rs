//! HTTP endpoint handlers.
//!
//! This module contains all route handlers for the API, organized by resource type.
//! Route handlers are thin wrappers that handle HTTP concerns (status codes, JSON
//! serialization) and delegate business logic to the services layer.

pub mod health;
pub mod players;
