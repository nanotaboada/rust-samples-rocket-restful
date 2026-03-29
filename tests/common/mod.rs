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
pub const SEED_MESSI_ID: &str = "f10f398d-b2ff-40aa-acac-51f58d129bc7";

// Test Fixture: Thiago Almada — squad 16, reserved for POST (create) tests
pub fn player_request_for_creation() -> PlayerRequest {
    PlayerRequest {
        first_name: "Thiago".to_string(),
        middle_name: "Ezequiel".to_string(),
        last_name: "Almada".to_string(),
        date_of_birth: "2001-04-26T00:00:00.000Z".to_string(),
        squad_number: 16,
        position: "Attacking Midfield".to_string(),
        abbr_position: "AM".to_string(),
        team: "Atlanta United FC".to_string(),
        league: "Major League Soccer".to_string(),
        starting11: false,
    }
}

// Returns the full 26-player seed minus squad 16, so POST tests can create
// Thiago Almada (squad 16) without hitting a duplicate-squad-number conflict.
pub fn players_except_player_for_creation() -> Vec<Player> {
    initialize_players()
        .into_iter()
        .filter(|p| p.squad_number != 16)
        .collect()
}
