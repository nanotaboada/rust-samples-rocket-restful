// Integration tests for player service
// Following Rust conventions: integration tests go in tests/ directory
// Uses real Argentina squad data from state module for realistic testing

use rust_samples_rocket_restful::models::player::{Player, PlayerRequest};
use rust_samples_rocket_restful::services::player_service::{self, CreateError, UpdateError};
use rust_samples_rocket_restful::state::player_collection::initialize_players;

// Returns 25 Argentina players (excluding Thiago Almada, reserved for creation tests)
fn players_without_almada() -> Vec<Player> {
    initialize_players()
        .into_iter()
        .filter(|p| p.id != 24) // Almada's ID
        .collect()
}

// Test Stub: Thiago Almada - Used for POST (create) tests
fn create_player_request_stub() -> PlayerRequest {
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

// Test Stub: Emiliano Martínez - Used for PUT (update) tests
// Updates first_name to "Emiliano" (his preferred name) and empties middle_name
fn update_player_request_stub() -> PlayerRequest {
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

// GET all players tests
#[test]
fn test_request_get_players_all_response_body_players() {
    let players = initialize_players();

    let result = player_service::get_all(&players);

    assert_eq!(result.len(), 26); // Full Argentina squad
    assert_eq!(result[0].id, 1); // Damián Martínez
    assert_eq!(result[9].id, 10); // Lionel Messi
    assert_eq!(result[0].first_name, "Damián");
    assert_eq!(result[9].first_name, "Lionel");
}

#[test]
fn test_request_get_players_empty_response_body_empty() {
    let players: Vec<Player> = vec![];
    let result = player_service::get_all(&players);
    assert_eq!(result.len(), 0);
}

// GET by ID tests
#[test]
fn test_request_get_player_id_existing_response_body_player() {
    let players = initialize_players();

    let result = player_service::get_by_id(&players, 10); // Lionel Messi

    assert!(result.is_some());
    let player = result.unwrap();
    assert_eq!(player.id, 10);
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.last_name, "Messi");
    assert_eq!(player.squad_number, 10);
}

#[test]
fn test_request_get_player_id_nonexistent_response_body_none() {
    let players = initialize_players();
    let result = player_service::get_by_id(&players, 999); // Non-existent ID
    assert!(result.is_none());
}

// GET by squad number tests
#[test]
fn test_request_get_player_squadnumber_existing_response_body_player() {
    let players = initialize_players();

    let result = player_service::get_by_squad_number(&players, 10); // Messi's squad number

    assert!(result.is_some());
    let player = result.unwrap();
    assert_eq!(player.squad_number, 10);
    assert_eq!(player.first_name, "Lionel");
    assert_eq!(player.last_name, "Messi");
}

#[test]
fn test_request_get_player_squadnumber_nonexistent_response_body_none() {
    let players = initialize_players();
    let result = player_service::get_by_squad_number(&players, 99); // Non-existent squad number
    assert!(result.is_none());
}

// POST (create) tests
#[test]
fn test_request_post_player_body_valid_response_body_created() {
    let mut players = players_without_almada(); // 25 players (Almada reserved for creation)
    let request = create_player_request_stub();

    let result = player_service::create(&mut players, request);

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, 27); // Next ID after max (26)
    assert_eq!(response.first_name, "Thiago");
    assert_eq!(response.last_name, "Almada");
    assert_eq!(response.squad_number, 16);
    assert_eq!(players.len(), 26); // Now 26 with Almada
}

#[test]
fn test_request_post_player_body_duplicate_response_status_conflict() {
    let mut players = initialize_players(); // All 26 players including Almada
    let request = create_player_request_stub(); // Try to create Almada again

    let result = player_service::create(&mut players, request);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CreateError::DuplicateSquadNumber
    ));
    assert_eq!(players.len(), 26); // No new player added
}

#[test]
fn test_request_post_player_body_valid_response_body_correct_id() {
    let mut players = players_without_almada();
    let request = create_player_request_stub();

    let result = player_service::create(&mut players, request);

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, 27); // Max ID in collection (26) + 1
    assert_eq!(response.first_name, "Thiago");
    assert_eq!(response.last_name, "Almada");
}

#[test]
fn test_request_post_player_body_valid_empty_collection_response_body_created() {
    let mut players: Vec<Player> = vec![];
    let request = create_player_request_stub();

    let result = player_service::create(&mut players, request);

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, 1); // First ID should be 1
    assert_eq!(response.first_name, "Thiago");
    assert_eq!(response.last_name, "Almada");
    assert_eq!(players.len(), 1);
}

// PUT (update) tests
#[test]
fn test_request_put_player_id_existing_body_valid_response_body_updated() {
    let mut players = initialize_players();
    let request = update_player_request_stub(); // Update Martínez: Damián -> Emiliano, clear middle name

    let result = player_service::update(&mut players, 1, request); // Martínez's ID

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, 1); // ID unchanged
    assert_eq!(response.first_name, "Emiliano"); // Now uses his preferred name
    assert_eq!(response.middle_name, ""); // Middle name cleared
    assert_eq!(response.last_name, "Martínez");
    assert_eq!(response.squad_number, 23);
}

#[test]
fn test_request_put_player_id_nonexistent_body_valid_response_status_not_found() {
    let mut players = initialize_players();
    let request = update_player_request_stub();

    let result = player_service::update(&mut players, 999, request); // Non-existent ID

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UpdateError::NotFound));
}

#[test]
fn test_request_put_player_id_existing_body_duplicate_response_status_conflict() {
    let mut players = initialize_players();
    // Try to update Messi (ID 10) with Martínez's squad number (23)
    let mut request = update_player_request_stub();
    request.first_name = "Lionel".to_string();
    request.last_name = "Messi".to_string();

    let result = player_service::update(&mut players, 10, request);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        UpdateError::DuplicateSquadNumber
    ));
}

#[test]
fn test_request_put_player_id_existing_body_same_response_body_updated() {
    let mut players = initialize_players();
    // Update Martínez keeping his squad number 23
    let request = update_player_request_stub();

    let result = player_service::update(&mut players, 1, request);

    assert!(result.is_ok()); // Should allow keeping same squad number
    let response = result.unwrap();
    assert_eq!(response.squad_number, 23);
    assert_eq!(response.first_name, "Emiliano");
}

// DELETE tests
#[test]
fn test_request_delete_player_id_existing_response_status_ok() {
    let mut players = initialize_players();

    let result = player_service::delete(&mut players, 21); // Delete Alejandro Gómez

    assert!(result); // Deletion succeeded
    assert_eq!(players.len(), 25); // 26 - 1
    // Verify Gómez is gone
    assert!(player_service::get_by_id(&players, 21).is_none());
}

#[test]
fn test_request_delete_player_id_nonexistent_response_status_not_found() {
    let mut players = initialize_players();

    let result = player_service::delete(&mut players, 999); // Non-existent ID

    assert!(!result); // Deletion failed
    assert_eq!(players.len(), 26); // No change
}

#[test]
fn test_request_delete_player_id_existing_last_response_status_ok() {
    let mut players = vec![initialize_players()[20].clone()]; // Just Alejandro Gómez (index 20 = ID 21)

    let result = player_service::delete(&mut players, 21);

    assert!(result);
    assert_eq!(players.len(), 0); // Empty collection
}

#[test]
fn test_request_delete_player_id_nonexistent_empty_collection_response_status_not_found() {
    let mut players: Vec<Player> = vec![];

    let result = player_service::delete(&mut players, 1);

    assert!(!result);
    assert_eq!(players.len(), 0);
}
