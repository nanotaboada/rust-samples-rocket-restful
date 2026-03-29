// Shared test fixtures for integration tests.
//
// Included as `mod common;` in each test binary that needs it. Items marked
// `#[allow(dead_code)]` suppress warnings for fixtures that are only used by a
// subset of test binaries.

#![allow(dead_code)]

use rust_samples_rocket_restful::{
    models::player::{Player, PlayerRequest},
    state::player_collection::initialize_players,
};

// Seed UUID for Lionel Messi — matches the value in player_collection.rs
pub const SEED_MESSI_ID: &str = "acc433bf-d505-51fe-831e-45eb44c4d43c";

// Test Fixture: Giovani Lo Celso — squad 27, reserved for POST (create) and DELETE tests.
// Lo Celso was in Argentina's preliminary squad for Qatar 2022 before injury.
// Squad 27 sits outside the seeded 1–26 range, so creation never conflicts with seed data.
pub fn player_request_for_creation() -> PlayerRequest {
    PlayerRequest {
        first_name: "Giovani".to_string(),
        middle_name: "".to_string(),
        last_name: "Lo Celso".to_string(),
        date_of_birth: "1996-07-09T00:00:00.000Z".to_string(),
        squad_number: 27,
        position: "Central Midfield".to_string(),
        abbr_position: "CM".to_string(),
        team: "Real Betis Balompié".to_string(),
        league: "La Liga".to_string(),
        starting11: false,
    }
}

// Returns the full 26-player seed minus squad 27, so POST tests can create
// Lo Celso (squad 27) without hitting a duplicate-squad-number conflict.
pub fn players_except_player_for_creation() -> Vec<Player> {
    initialize_players()
        .into_iter()
        .filter(|p| p.squad_number != 27)
        .collect()
}
