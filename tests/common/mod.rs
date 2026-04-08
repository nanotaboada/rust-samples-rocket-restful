// Shared test fixtures for integration tests.
//
// Included as `mod common;` in each test binary that needs it. Items marked
// `#[allow(dead_code)]` suppress warnings for fixtures that are only used by a
// subset of test binaries.

#![allow(dead_code)]

use rust_samples_rocket_restful::models::player::PlayerRequest;

/// UUID of an existing player (Lionel Messi, squad 10) — matches the value seeded in player_collection.rs.
/// Used in GET-by-UUID tests to verify a successful 200 OK lookup.
pub const EXISTING_PLAYER_ID: &str = "acc433bf-d505-51fe-831e-45eb44c4d43c";

/// A well-formed UUID guaranteed not to exist in any seeded dataset.
/// Used in POST flows to represent a player absent from the database with a valid shape for creation.
pub const NONEXISTENT_PLAYER_ID: &str = "00000000-0000-0000-0000-000000000000";

/// A valid UUID format that is simply absent from the database.
/// Used in GET/PUT/DELETE 404-by-lookup scenarios to verify Not Found handling.
pub const UNKNOWN_PLAYER_ID: &str = "f47ac10b-58cc-4372-a567-0e02b2c3d479";

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

// Test Fixture: Emiliano Martínez — used for PUT (update) tests.
// squad_number 23 matches the seeded Damián Martínez (squad_number: 23).
// squadNumber in the body is deliberately set to 99 (≠ 23) to prove the route
// param wins and the natural key remains immutable.
pub fn player_request_for_update() -> PlayerRequest {
    PlayerRequest {
        first_name: "Emiliano".to_string(),
        middle_name: "".to_string(),
        last_name: "Martínez".to_string(),
        date_of_birth: "1992-09-02T00:00:00.000Z".to_string(),
        squad_number: 99,
        position: "Goalkeeper".to_string(),
        abbr_position: "GK".to_string(),
        team: "Aston Villa FC".to_string(),
        league: "Premier League".to_string(),
        starting11: true,
    }
}
