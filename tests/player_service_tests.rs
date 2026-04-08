// Integration tests for player service
// Following Rust conventions: integration tests go in tests/ directory
// Uses in-memory SQLite seeded with the full Argentina 2022 World Cup squad.

mod common;

use rust_samples_rocket_restful::services::player_service::{self, CreateError, UpdateError};
use rust_samples_rocket_restful::state::player_collection::{
    initialize_empty_test_database, initialize_test_database,
};

// GET /players/ ---------------------------------------------------------------

// GET /players/ returns all 26 players
#[test]
fn test_request_get_players_all_response_body_players() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_all(&mut conn);
    // Assert
    assert!(result.is_ok());
    let players = result.unwrap();
    assert_eq!(players.len(), 26);
    for player in players.iter() {
        assert!(!player.id.is_empty());
    }
}

// GET /players/{uuid} ---------------------------------------------------------

// GET /players/{uuid} with existing UUID returns 200 OK
#[test]
fn test_request_get_player_id_existing_response_body_player() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_by_id(&mut conn, common::EXISTING_PLAYER_ID);
    // Assert
    assert!(result.is_ok());
    let player = result.unwrap();
    assert!(player.is_some());
    let player = player.unwrap();
    assert_eq!(player.id, common::EXISTING_PLAYER_ID);
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.middle_name, "Andrés");
    assert_eq!(player.last_name, "Messi");
    assert_eq!(player.date_of_birth, "1987-06-24T00:00:00.000Z");
    assert_eq!(player.squad_number, 10);
    assert_eq!(player.position, "Right Winger");
    assert_eq!(player.abbr_position, "RW");
    assert_eq!(player.team, "Paris Saint-Germain");
    assert_eq!(player.league, "Ligue 1");
    assert!(player.starting11);
}

// GET /players/{uuid} with nonexistent UUID returns 404 Not Found
#[test]
fn test_request_get_player_id_nonexistent_response_body_none() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_by_id(&mut conn, common::NONEXISTENT_PLAYER_ID);
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// GET /players/{uuid} with unknown UUID (valid format, absent from DB) returns None
#[test]
fn test_request_get_player_id_unknown_response_body_none() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_by_id(&mut conn, common::UNKNOWN_PLAYER_ID);
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// GET /players/squadnumber/{squad_number} ------------------------------------

// GET /players/squadnumber/{squad_number} with existing number returns 200 OK
#[test]
fn test_request_get_player_squadnumber_existing_response_body_player() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_by_squad_number(&mut conn, 10);
    // Assert
    assert!(result.is_ok());
    let player = result.unwrap();
    assert!(player.is_some());
    let player = player.unwrap();
    assert!(!player.id.is_empty());
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.middle_name, "Andrés");
    assert_eq!(player.last_name, "Messi");
    assert_eq!(player.date_of_birth, "1987-06-24T00:00:00.000Z");
    assert_eq!(player.squad_number, 10);
    assert_eq!(player.position, "Right Winger");
    assert_eq!(player.abbr_position, "RW");
    assert_eq!(player.team, "Paris Saint-Germain");
    assert_eq!(player.league, "Ligue 1");
    assert!(player.starting11);
}

// GET /players/squadnumber/{squad_number} with nonexistent number returns 404 Not Found
#[test]
fn test_request_get_player_squadnumber_nonexistent_response_body_none() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::get_by_squad_number(&mut conn, 99);
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// POST /players/ --------------------------------------------------------------

// POST /players/ with valid body returns 201 Created
#[test]
fn test_request_post_player_body_valid_response_body_created() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let request = common::player_request_for_creation();
    // Act
    let result = player_service::create(&mut conn, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(response.first_name, "Giovani");
    assert_eq!(response.middle_name, "");
    assert_eq!(response.last_name, "Lo Celso");
    assert_eq!(response.date_of_birth, "1996-07-09T00:00:00.000Z");
    assert_eq!(response.squad_number, 27);
    assert_eq!(response.position, "Central Midfield");
    assert_eq!(response.abbr_position, "CM");
    assert_eq!(response.team, "Real Betis Balompié");
    assert_eq!(response.league, "La Liga");
    assert!(!response.starting11);
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 27);
}

// POST /players/ with duplicate squad number returns 409 Conflict
#[test]
fn test_request_post_player_body_duplicate_response_status_conflict() {
    // Arrange — insert Lo Celso first, then attempt a second creation
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    player_service::create(&mut conn, common::player_request_for_creation()).unwrap();
    let request = common::player_request_for_creation();
    // Act
    let result = player_service::create(&mut conn, request);
    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CreateError::DuplicateSquadNumber
    ));
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 27);
}

// POST /players/ with valid body assigns a non-empty UUID
#[test]
fn test_request_post_player_body_valid_response_body_uuid_assigned() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let request = common::player_request_for_creation();
    // Act
    let result = player_service::create(&mut conn, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(response.id.len(), 36); // UUID v4 string length
}

// POST /players/ to empty collection returns 201 Created with a UUID
#[test]
fn test_request_post_player_body_valid_empty_collection_response_body_created() {
    // Arrange — empty in-memory DB (no seed)
    let pool = initialize_empty_test_database();
    let mut conn = pool.get().expect("pool connection");
    let request = common::player_request_for_creation();
    // Act
    let result = player_service::create(&mut conn, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.id.is_empty());
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 1);
}

// PUT /players/squadnumber/{squad_number} -------------------------------------

// PUT /players/squadnumber/{squad_number} with existing squad number returns 200 OK
#[test]
fn test_request_put_player_squadnumber_existing_body_valid_response_body_updated() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let original = player_service::get_by_squad_number(&mut conn, 23)
        .unwrap()
        .unwrap();
    let request = common::player_request_for_update();
    // Act
    let result = player_service::update(&mut conn, 23, request);
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
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let request = common::player_request_for_update();
    // Act
    let result = player_service::update(&mut conn, 999, request);
    // Assert
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UpdateError::NotFound));
}

// PUT /players/squadnumber/{squad_number} preserves squad number from the route param
#[test]
fn test_request_put_player_squadnumber_existing_body_squad_number_immutable() {
    // Arrange — body carries squadNumber 99; route param (23) must win
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let request = common::player_request_for_update(); // squad_number = 99 in body
    // Act
    let result = player_service::update(&mut conn, 23, request);
    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.squad_number, 23); // route param wins, body ignored
}

// DELETE /players/squadnumber/{squad_number} ----------------------------------

// DELETE /players/squadnumber/{squad_number} with existing squad number returns 204 No Content
#[test]
fn test_request_delete_player_squadnumber_existing_response_status_ok() {
    // Arrange — insert Lo Celso (squad 27) first, then delete by squad number
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    player_service::create(&mut conn, common::player_request_for_creation()).unwrap();
    // Act
    let result = player_service::delete(&mut conn, 27);
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 26);
    assert!(
        player_service::get_by_squad_number(&mut conn, 27)
            .unwrap()
            .is_none()
    );
}

// DELETE /players/squadnumber/{squad_number} with unknown squad number returns 404 Not Found
#[test]
fn test_request_delete_player_squadnumber_nonexistent_response_status_not_found() {
    // Arrange
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::delete(&mut conn, 999);
    // Assert
    assert!(result.is_ok());
    assert!(!result.unwrap());
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 26);
}

// DELETE /players/squadnumber/{squad_number} last remaining player returns 204 No Content
#[test]
fn test_request_delete_player_squadnumber_existing_last_response_status_ok() {
    // Arrange — only keep Alejandro Gómez (squad_number 17)
    let pool = initialize_test_database();
    let mut conn = pool.get().expect("pool connection");
    let all = player_service::get_all(&mut conn).unwrap();
    for p in &all {
        if p.squad_number != 17 {
            player_service::delete(&mut conn, p.squad_number).unwrap();
        }
    }
    // Act
    let result = player_service::delete(&mut conn, 17);
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 0);
}

// DELETE /players/squadnumber/{squad_number} from empty collection returns 404 Not Found
#[test]
fn test_request_delete_player_squadnumber_nonexistent_empty_collection_response_status_not_found() {
    // Arrange — empty in-memory DB (no seed)
    let pool = initialize_empty_test_database();
    let mut conn = pool.get().expect("pool connection");
    // Act
    let result = player_service::delete(&mut conn, 10);
    // Assert
    assert!(result.is_ok());
    assert!(!result.unwrap());
    assert_eq!(player_service::get_all(&mut conn).unwrap().len(), 0);
}
