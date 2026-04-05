//! Player domain models and type mappings.
//!
//! This module defines the two representations of a player used across the
//! application layers.
//!
//! ## Representations
//!
//! | Type              | Purpose                                     | Layer  |
//! |-------------------|---------------------------------------------|--------|
//! | [`PlayerRequest`] | Inbound API payload (ID excluded)           | routes |
//! | [`PlayerResponse`]| Outbound API payload (ID included)          | routes |
//!
//! ## Rust note: derive macros
//!
//! `#[derive(...)]` auto-generates standard trait implementations:
//! - `Debug` — enables `{:?}` formatting for logging and tests
//! - `Clone` — allows creating an independent copy of a value
//! - `Serialize` — converts the struct to JSON (for API responses)
//! - `Deserialize` — populates the struct from JSON (for API requests)
//! - `#[serde(rename_all = "camelCase")]` maps snake_case Rust fields to
//!   camelCase JSON keys (e.g. `first_name` → `"firstName"`)

use rocket::serde::{Deserialize, Serialize};

/// Inbound payload for creating or updating a player.
///
/// Received as the JSON request body for `POST /players` and
/// `PUT /players/squadnumber/{squad_number}`.
/// The `id` field is intentionally absent — it is assigned automatically on
/// creation. `squad_number` is the natural key and is immutable once set: on
/// `PUT` the value in the request body is ignored and the existing squad number
/// is preserved.
///
/// ## Rust note: missing `Clone` and `Serialize`
/// This struct only needs `Deserialize` because it is only ever read from JSON.
/// `Clone` and `Serialize` are omitted to keep the type minimal and make
/// incorrect usage a compile-time error.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRequest {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub squad_number: u32,
    pub position: String,
    pub abbr_position: String,
    pub team: String,
    pub league: String,
    pub starting11: bool,
}

/// Outbound payload returned in API responses.
///
/// Returned as a JSON object for single-player endpoints, or as an element
/// inside a `Vec<PlayerResponse>` for the list endpoint.
///
/// ## Rust note: missing `Deserialize`
/// This struct only needs `Serialize` because it is only ever written to JSON.
/// `Deserialize` is omitted since we never parse a response back into Rust.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub squad_number: u32,
    pub position: String,
    pub abbr_position: String,
    pub team: String,
    pub league: String,
    pub starting11: bool,
}
