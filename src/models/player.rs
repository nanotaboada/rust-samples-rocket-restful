//! Player domain models and type mappings.
//!
//! This module defines the three representations of a player used across the
//! application layers, and the mapping logic between them.
//!
//! ## Representations
//!
//! | Type              | Purpose                                     | Layer  |
//! |-------------------|---------------------------------------------|--------|
//! | [`Player`]        | Internal storage entity (includes ID)       | state  |
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

/// Internal entity stored in the in-memory collection.
///
/// This is the canonical representation used inside the application.
/// It is never sent directly over the wire — API responses use [`PlayerResponse`].
///
/// ## Rust note: `Clone` + `Serialize` + `Deserialize`
/// `Clone` is required because [`PlayerCollection`](crate::state::player_collection::PlayerCollection)
/// is shared across threads via [`std::sync::Mutex`], and individual players
/// are sometimes cloned when building responses. `Serialize` + `Deserialize`
/// support reading and writing JSON (used for seed data).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
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

// These implementations describe how to map between player representations.
// In Rust, mapping between types is done via the standard `From` trait, which
// the language refers to as "type conversions" — but the term "mappings" is
// used here to align with conventions familiar from other languages (e.g.
// AutoMapper in .NET, MapStruct in Java).
//
// Two variants exist for the Player → PlayerResponse mapping:
//
//   From<Player>   — takes *ownership* of the source (moves it, no `.clone()` needed)
//   From<&Player>  — *borrows* the source (reads without consuming it, `.clone()`
//                    required for `String` fields since they cannot be moved out
//                    of a shared reference)

impl PlayerRequest {
    /// Maps a [`PlayerRequest`] into a [`Player`] by consuming it.
    ///
    /// The `id` parameter is the externally assigned UUID (auto-generated on
    /// creation). The `squad_number` stored on the resulting [`Player`] comes
    /// from the request; callers that need to enforce immutability (e.g. `PUT`)
    /// are responsible for overwriting it with the existing value afterwards.
    ///
    /// ## Rust note: `self` means ownership (move semantics)
    /// `self` (without `&`) means the method takes *ownership* of the
    /// `PlayerRequest`. The caller gives up the value — it cannot be used after
    /// this call. In return, `String` fields are *moved* directly into the new
    /// `Player` without any heap allocation, making this a zero-cost operation.
    /// The borrow checker enforces this at compile time.
    pub fn into_player(self, id: String) -> Player {
        Player {
            id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            squad_number: self.squad_number,
            position: self.position,
            abbr_position: self.abbr_position,
            team: self.team,
            league: self.league,
            starting11: self.starting11,
        }
    }
}

impl From<Player> for PlayerResponse {
    /// Maps an owned [`Player`] into a [`PlayerResponse`].
    ///
    /// ## Rust note: owned mapping (move semantics, no `.clone()`)
    /// `player` is passed by value, so Rust *moves* ownership into this
    /// function. Each `String` field transfers its heap allocation directly
    /// to the new `PlayerResponse` — no copying occurs. The original `Player`
    /// is consumed and cannot be used after this call.
    /// The primitive field (`u32`) and `bool` implement the `Copy` trait, so
    /// they are duplicated automatically without any explicit `.clone()`.
    ///
    /// Use this variant when the original `Player` is no longer needed after
    /// the mapping (e.g. after removing it from the collection on update).
    fn from(player: Player) -> Self {
        PlayerResponse {
            id: player.id,
            first_name: player.first_name,
            middle_name: player.middle_name,
            last_name: player.last_name,
            date_of_birth: player.date_of_birth,
            squad_number: player.squad_number,
            position: player.position,
            abbr_position: player.abbr_position,
            team: player.team,
            league: player.league,
            starting11: player.starting11,
        }
    }
}

impl From<&Player> for PlayerResponse {
    /// Maps a borrowed [`Player`] reference into a [`PlayerResponse`].
    ///
    /// ## Rust note: borrow mapping (`.clone()` required for `String` fields)
    /// `player` is a shared reference (`&Player`), so the original value is
    /// *not* consumed — it remains usable after this call. However, because
    /// Rust does not allow moving a value out of a reference that doesn't own
    /// it, each `String` field must be `.clone()`d to allocate a new
    /// independent copy on the heap.
    /// The primitive field (`u32`) and `bool` implement `Copy` and are
    /// duplicated implicitly — no `.clone()` needed for them.
    ///
    /// Use this variant when the original `Player` must remain usable after
    /// the mapping (e.g. when iterating over the collection to build a list).
    fn from(player: &Player) -> Self {
        PlayerResponse {
            id: player.id.clone(),
            first_name: player.first_name.clone(),
            middle_name: player.middle_name.clone(),
            last_name: player.last_name.clone(),
            date_of_birth: player.date_of_birth.clone(),
            squad_number: player.squad_number,
            position: player.position.clone(),
            abbr_position: player.abbr_position.clone(),
            team: player.team.clone(),
            league: player.league.clone(),
            starting11: player.starting11,
        }
    }
}
