// Integration tests for player route handlers
// Exercises the full HTTP request/response cycle using Rocket's blocking test
// client. Each test gets a fresh Rocket instance with the full 26-player seed.

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rust_samples_rocket_restful::{
    routes,
    state::player_collection::{PlayerCollection, initialize_players},
};

// Seed UUID for Lionel Messi — matches the value in player_collection.rs
const SEED_MESSI_ID: &str = "f10f398d-b2ff-40aa-acac-51f58d129bc7";

fn setup_client() -> Client {
    let players = PlayerCollection::new(initialize_players());
    let rocket = rocket::build()
        .manage(players)
        .mount("/", routes::health::routes())
        .mount("/", routes::players::routes());
    Client::tracked(rocket).expect("valid rocket instance")
}

// Test Fixture: squad 99 — not present in the 26-player seed, safe for POST tests
fn player_request_for_creation_json() -> serde_json::Value {
    serde_json::json!({
        "firstName": "Thiago",
        "middleName": "Ezequiel",
        "lastName": "Almada",
        "dateOfBirth": "2001-04-26T00:00:00.000Z",
        "squadNumber": 99,
        "position": "Attacking Midfield",
        "abbrPosition": "AM",
        "team": "Atlanta United FC",
        "league": "Major League Soccer",
        "starting11": false
    })
}

// Test Fixture: Emiliano Martínez — used for PUT tests targeting squad 23
fn player_request_for_update_json() -> serde_json::Value {
    serde_json::json!({
        "firstName": "Emiliano",
        "middleName": "",
        "lastName": "Martínez",
        "dateOfBirth": "1992-09-02T00:00:00.000Z",
        "squadNumber": 23,
        "position": "Goalkeeper",
        "abbrPosition": "GK",
        "team": "Aston Villa FC",
        "league": "Premier League",
        "starting11": true
    })
}

// GET /health -----------------------------------------------------------------

// GET /health returns 200 OK
#[test]
fn test_request_get_health_response_status_ok() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get("/health").dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
}

// GET /players ----------------------------------------------------------------

// GET /players returns 200 OK with all 26 players
#[test]
fn test_request_get_players_all_response_status_ok() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get("/players").dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(body.as_array().unwrap().len(), 26);
}

// GET /players returns a body where every element has the expected fields
#[test]
fn test_request_get_players_all_response_body_structure() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get("/players").dispatch();
    // Assert
    let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    let first = &body.as_array().unwrap()[0];
    assert!(first["id"].is_string());
    assert!(first["firstName"].is_string());
    assert!(first["middleName"].is_string());
    assert!(first["lastName"].is_string());
    assert!(first["dateOfBirth"].is_string());
    assert!(first["squadNumber"].is_number());
    assert!(first["position"].is_string());
    assert!(first["abbrPosition"].is_string());
    assert!(first["team"].is_string());
    assert!(first["league"].is_string());
    assert!(first["starting11"].is_boolean());
}

// GET /players/{id} -----------------------------------------------------------

// GET /players/{uuid} with existing UUID returns 200 OK with full player body
#[test]
fn test_request_get_player_by_id_existing_response_status_ok() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get(format!("/players/{SEED_MESSI_ID}")).dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(body["id"], SEED_MESSI_ID);
    assert_eq!(body["firstName"], "Lionel");
    assert_eq!(body["middleName"], "Andrés");
    assert_eq!(body["lastName"], "Messi");
    assert_eq!(body["dateOfBirth"], "1987-06-24T00:00:00.000Z");
    assert_eq!(body["squadNumber"], 10);
    assert_eq!(body["position"], "Right Winger");
    assert_eq!(body["abbrPosition"], "RW");
    assert_eq!(body["team"], "Inter Miami CF");
    assert_eq!(body["league"], "Major League Soccer");
    assert_eq!(body["starting11"], true);
}

// GET /players/{uuid} with nonexistent UUID returns 404 Not Found
#[test]
fn test_request_get_player_by_id_nonexistent_response_status_not_found() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client
        .get("/players/00000000-0000-0000-0000-000000000000")
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::NotFound);
}

// GET /players/squadnumber/{squad_number} -------------------------------------

// GET /players/squadnumber/{squad_number} with existing number returns 200 OK
#[test]
fn test_request_get_player_by_squadnumber_existing_response_status_ok() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get("/players/squadnumber/10").dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(body["squadNumber"], 10);
    assert_eq!(body["firstName"], "Lionel");
    assert_eq!(body["middleName"], "Andrés");
    assert_eq!(body["lastName"], "Messi");
    assert_eq!(body["dateOfBirth"], "1987-06-24T00:00:00.000Z");
    assert_eq!(body["position"], "Right Winger");
    assert_eq!(body["abbrPosition"], "RW");
    assert_eq!(body["team"], "Inter Miami CF");
    assert_eq!(body["league"], "Major League Soccer");
    assert_eq!(body["starting11"], true);
}

// GET /players/squadnumber/{squad_number} with nonexistent number returns 404
#[test]
fn test_request_get_player_by_squadnumber_nonexistent_response_status_not_found() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.get("/players/squadnumber/99").dispatch();
    // Assert
    assert_eq!(response.status(), Status::NotFound);
}

// POST /players ---------------------------------------------------------------

// POST /players with valid body returns 201 Created with full player response
#[test]
fn test_request_post_player_body_valid_response_status_created() {
    // Arrange
    let client = setup_client();
    let body = player_request_for_creation_json();
    // Act
    let response = client
        .post("/players")
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::Created);
    let response_body: serde_json::Value =
        serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert!(!response_body["id"].as_str().unwrap().is_empty());
    assert_eq!(response_body["id"].as_str().unwrap().len(), 36); // UUID v4
    assert_eq!(response_body["firstName"], "Thiago");
    assert_eq!(response_body["middleName"], "Ezequiel");
    assert_eq!(response_body["lastName"], "Almada");
    assert_eq!(response_body["dateOfBirth"], "2001-04-26T00:00:00.000Z");
    assert_eq!(response_body["squadNumber"], 99);
    assert_eq!(response_body["position"], "Attacking Midfield");
    assert_eq!(response_body["abbrPosition"], "AM");
    assert_eq!(response_body["team"], "Atlanta United FC");
    assert_eq!(response_body["league"], "Major League Soccer");
    assert_eq!(response_body["starting11"], false);
}

// POST /players with duplicate squad number returns 409 Conflict
#[test]
fn test_request_post_player_body_duplicate_response_status_conflict() {
    // Arrange
    let client = setup_client();
    let body = serde_json::json!({
        "firstName": "Duplicate",
        "middleName": "",
        "lastName": "Player",
        "dateOfBirth": "1990-01-01T00:00:00.000Z",
        "squadNumber": 10, // Messi's number — already in the seed
        "position": "Forward",
        "abbrPosition": "FW",
        "team": "Some Club",
        "league": "Some League",
        "starting11": false
    });
    // Act
    let response = client
        .post("/players")
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::Conflict);
}

// PUT /players/squadnumber/{squad_number} -------------------------------------

// PUT /players/squadnumber/{squad_number} with existing number returns 200 OK
#[test]
fn test_request_put_player_squadnumber_existing_response_status_ok() {
    // Arrange
    let client = setup_client();
    let body = player_request_for_update_json();
    // Act
    let response = client
        .put("/players/squadnumber/23")
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let response_body: serde_json::Value =
        serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(response_body["squadNumber"], 23); // immutable — preserved from route
    assert!(!response_body["id"].as_str().unwrap().is_empty()); // UUID preserved
    assert_eq!(response_body["firstName"], "Emiliano");
    assert_eq!(response_body["middleName"], "");
    assert_eq!(response_body["lastName"], "Martínez");
    assert_eq!(response_body["dateOfBirth"], "1992-09-02T00:00:00.000Z");
    assert_eq!(response_body["position"], "Goalkeeper");
    assert_eq!(response_body["abbrPosition"], "GK");
    assert_eq!(response_body["team"], "Aston Villa FC");
    assert_eq!(response_body["league"], "Premier League");
    assert_eq!(response_body["starting11"], true);
}

// PUT /players/squadnumber/{squad_number} with nonexistent number returns 404
#[test]
fn test_request_put_player_squadnumber_nonexistent_response_status_not_found() {
    // Arrange
    let client = setup_client();
    let body = player_request_for_update_json();
    // Act
    let response = client
        .put("/players/squadnumber/999")
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::NotFound);
}

// DELETE /players/squadnumber/{squad_number} ----------------------------------

// DELETE /players/squadnumber/{squad_number} with existing number returns 204
#[test]
fn test_request_delete_player_squadnumber_existing_response_status_no_content() {
    // Arrange
    let client = setup_client();
    // Act — Alejandro Gómez wears squad_number 17
    let response = client.delete("/players/squadnumber/17").dispatch();
    // Assert
    assert_eq!(response.status(), Status::NoContent);
}

// DELETE /players/squadnumber/{squad_number} with nonexistent number returns 404
#[test]
fn test_request_delete_player_squadnumber_nonexistent_response_status_not_found() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.delete("/players/squadnumber/999").dispatch();
    // Assert
    assert_eq!(response.status(), Status::NotFound);
}
