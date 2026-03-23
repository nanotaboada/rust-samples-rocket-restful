// Integration tests for player service
// Following Rust conventions: integration tests go in tests/ directory
// Uses real Argentina squad data from state module for realistic testing

use rust_samples_rocket_restful::models::player::{Player, PlayerRequest};
use rust_samples_rocket_restful::services::player_service::{self, CreateError, UpdateError};
use rust_samples_rocket_restful::state::player_collection::initialize_players;

// Returns 25 Argentina players (excluding Thiago Almada, reserved for creation tests)
fn players_except_player_for_creation() -> Vec<Player> {
    initialize_players()
        .into_iter()
        .filter(|p| p.squad_number != 16)
        .collect()
}

// Test Fixture: Thiago Almada - Used for POST (create) tests
fn player_request_for_creation() -> PlayerRequest {
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

// Test Fixture: Emiliano Martínez - Used for PUT (update) tests
// squad_number 23 matches the seeded Damián Martínez (squad_number: 23)
fn player_request_for_update() -> PlayerRequest {
    PlayerRequest {
        first_name: "Emiliano".to_string(),
        middle_name: "".to_string(),
        last_name: "Martínez".to_string(),
        date_of_birth: "1992-09-02T00:00:00.000Z".to_string(),
        squad_number: 23,
        position: "Goalkeeper".to_string(),
        abbr_position: "GK".to_string(),
        team: "Aston Villa FC".to_string(),
        league: "Premier League".to_string(),
        starting11: true,
    }
}

// GET /players/ ---------------------------------------------------------------

// GET /players/ returns 200 OK
#[test]
fn test_request_get_players_all_response_body_players() {
    // Arrange
    let players = initialize_players();
    // Act
    let result = player_service::get_all(&players);
    // Assert
    assert_eq!(result.len(), 26);
    for player in result.iter() {
        assert!(!player.id.is_empty());
    }
}

// Seed UUID for Lionel Messi — matches the value in player_collection.rs
const SEED_MESSI_ID: &str = "f10f398d-b2ff-40aa-acac-51f58d129bc7";

// GET /players/{uuid} ---------------------------------------------------------

// GET /players/{uuid} with existing UUID returns 200 OK
#[test]
fn test_request_get_player_id_existing_response_body_player() {
    // Arrange
    let players = initialize_players();
    // Act
    let result = player_service::get_by_id(&players, SEED_MESSI_ID);
    // Assert
    assert!(result.is_some());
    let player = result.unwrap();
    assert_eq!(player.id, SEED_MESSI_ID);
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.middle_name, "Andrés");
    assert_eq!(player.last_name, "Messi");
    assert_eq!(player.date_of_birth, "1987-06-24T00:00:00.000Z");
    assert_eq!(player.squad_number, 10);
    assert_eq!(player.position, "Right Winger");
    assert_eq!(player.abbr_position, "RW");
    assert_eq!(player.team, "Inter Miami CF");
    assert_eq!(player.league, "Major League Soccer");
    assert!(player.starting11);
}

// GET /players/{uuid} with nonexistent UUID returns 404 Not Found
#[test]
fn test_request_get_player_id_nonexistent_response_body_none() {
    // Arrange
    let players = initialize_players();
    // Act
    let result = player_service::get_by_id(&players, "00000000-0000-0000-0000-000000000000");
    // Assert
    assert!(result.is_none());
}

// GET /players/squadnumber/{squad_number} ------------------------------------

// GET /players/squadnumber/{squad_number} with existing number returns 200 OK
#[test]
fn test_request_get_player_squadnumber_existing_response_body_player() {
    // Arrange
    let players = initialize_players();
    // Act
    let result = player_service::get_by_squad_number(&players, 10);
    // Assert
    assert!(result.is_some());
    let player = result.unwrap();
    assert!(!player.id.is_empty());
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.middle_name, "Andrés");
    assert_eq!(player.last_name, "Messi");
    assert_eq!(player.date_of_birth, "1987-06-24T00:00:00.000Z");
    assert_eq!(player.squad_number, 10);
    assert_eq!(player.position, "Right Winger");
    assert_eq!(player.abbr_position, "RW");
    assert_eq!(player.team, "Inter Miami CF");
    assert_eq!(player.league, "Major League Soccer");
    assert!(player.starting11);
}

// GET /players/squadnumber/{squad_number} with nonexistent number returns 404 Not Found
#[test]
fn test_request_get_player_squadnumber_nonexistent_response_body_none() {
    // Arrange
    let players = initialize_players();
    // Act
    let result = player_service::get_by_squad_number(&players, 99);
    // Assert
    assert!(result.is_none());
}

// POST /players/ --------------------------------------------------------------

// POST /players/ with valid body returns 201 Created
#[test]
fn test_request_post_player_body_valid_response_body_created() {
    // Arrange
    let mut players = players_except_player_for_creation();
    let request = player_request_for_creation();
    // Act
    let result = player_service::create(&mut players, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(response.first_name, "Thiago");
    assert_eq!(response.middle_name, "Ezequiel");
    assert_eq!(response.last_name, "Almada");
    assert_eq!(response.date_of_birth, "2001-04-26T00:00:00.000Z");
    assert_eq!(response.squad_number, 16);
    assert_eq!(response.position, "Attacking Midfield");
    assert_eq!(response.abbr_position, "AM");
    assert_eq!(response.team, "Atlanta United FC");
    assert_eq!(response.league, "Major League Soccer");
    assert!(!response.starting11);
    assert_eq!(players.len(), 26);
}

// POST /players/ with duplicate squad number returns 409 Conflict
#[test]
fn test_request_post_player_body_duplicate_response_status_conflict() {
    // Arrange
    let mut players = initialize_players();
    let request = player_request_for_creation();
    // Act
    let result = player_service::create(&mut players, request);
    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CreateError::DuplicateSquadNumber
    ));
    assert_eq!(players.len(), 26);
}

// POST /players/ with valid body assigns a non-empty UUID
#[test]
fn test_request_post_player_body_valid_response_body_uuid_assigned() {
    // Arrange
    let mut players = players_except_player_for_creation();
    let request = player_request_for_creation();
    // Act
    let result = player_service::create(&mut players, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(response.id.len(), 36); // UUID v4 string length
}

// POST /players/ to empty collection returns 201 Created with a UUID
#[test]
fn test_request_post_player_body_valid_empty_collection_response_body_created() {
    // Arrange
    let mut players: Vec<Player> = vec![];
    let request = player_request_for_creation();
    // Act
    let result = player_service::create(&mut players, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(players.len(), 1);
}

// PUT /players/squadnumber/{squad_number} -------------------------------------

// PUT /players/squadnumber/{squad_number} with existing squad number returns 200 OK
#[test]
fn test_request_put_player_squadnumber_existing_body_valid_response_body_updated() {
    // Arrange
    let mut players = initialize_players();
    let original = player_service::get_by_squad_number(&players, 23).unwrap();
    let request = player_request_for_update();
    // Act
    let result = player_service::update(&mut players, 23, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, original.id); // UUID is preserved
    assert_eq!(response.squad_number, 23); // squad_number is immutable
    assert_eq!(response.first_name, "Emiliano");
    assert_eq!(response.middle_name, "");
    assert_eq!(response.last_name, "Martínez");
    assert_eq!(response.date_of_birth, "1992-09-02T00:00:00.000Z");
    assert_eq!(response.position, "Goalkeeper");
    assert_eq!(response.abbr_position, "GK");
    assert_eq!(response.team, "Aston Villa FC");
    assert_eq!(response.league, "Premier League");
    assert!(response.starting11);
}

// PUT /players/squadnumber/{squad_number} with unknown squad number returns 404 Not Found
#[test]
fn test_request_put_player_squadnumber_nonexistent_body_valid_response_status_not_found() {
    // Arrange
    let mut players = initialize_players();
    let request = player_request_for_update();
    // Act
    let result = player_service::update(&mut players, 999, request);
    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UpdateError::NotFound));
}

// PUT /players/squadnumber/{squad_number} preserves squad number from the route param
#[test]
fn test_request_put_player_squadnumber_existing_body_squad_number_immutable() {
    // Arrange
    let mut players = initialize_players();
    let mut request = player_request_for_update();
    request.squad_number = 99; // attempt to change squad number via body
    // Act
    let result = player_service::update(&mut players, 23, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.squad_number, 23); // route param wins, body ignored
}

// DELETE /players/squadnumber/{squad_number} ----------------------------------

// DELETE /players/squadnumber/{squad_number} with existing squad number returns 204 No Content
#[test]
fn test_request_delete_player_squadnumber_existing_response_status_ok() {
    // Arrange
    let mut players = initialize_players();
    // Act — Alejandro Gómez wears squad_number 17
    let result = player_service::delete(&mut players, 17);
    // Assert
    assert!(result);
    assert_eq!(players.len(), 25);
    assert!(player_service::get_by_squad_number(&players, 17).is_none());
}

// DELETE /players/squadnumber/{squad_number} with unknown squad number returns 404 Not Found
#[test]
fn test_request_delete_player_squadnumber_nonexistent_response_status_not_found() {
    // Arrange
    let mut players = initialize_players();
    // Act
    let result = player_service::delete(&mut players, 999);
    // Assert
    assert!(!result);
    assert_eq!(players.len(), 26);
}

// DELETE /players/squadnumber/{squad_number} last remaining player returns 204 No Content
#[test]
fn test_request_delete_player_squadnumber_existing_last_response_status_ok() {
    // Arrange — only keep Alejandro Gómez (squad_number 17)
    let mut players: Vec<Player> = initialize_players()
        .into_iter()
        .filter(|p| p.squad_number == 17)
        .collect();
    // Act
    let result = player_service::delete(&mut players, 17);
    // Assert
    assert!(result);
    assert_eq!(players.len(), 0);
}

// DELETE /players/squadnumber/{squad_number} from empty collection returns 404 Not Found
#[test]
fn test_request_delete_player_squadnumber_nonexistent_empty_collection_response_status_not_found() {
    // Arrange
    let mut players: Vec<Player> = vec![];
    // Act
    let result = player_service::delete(&mut players, 10);
    // Assert
    assert!(!result);
    assert_eq!(players.len(), 0);
}
