// Integration tests for player route handlers
// Exercises the full HTTP request/response cycle using Rocket's blocking test
// client. Each test gets a fresh Rocket instance backed by an in-memory SQLite
// database seeded with the full 26-player Argentina 2022 World Cup squad.

mod common;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rust_samples_rocket_restful::{
    routes,
    state::player_collection::{PlayerCollection, initialize_test_database},
};

// Full 26-player seed — used by all tests except POST creation
fn setup_client() -> Client {
    let database = PlayerCollection::new(initialize_test_database());
    let rocket = rocket::build()
        .manage(database)
        .mount("/", routes::health::routes())
        .mount("/", routes::players::routes());
    Client::tracked(rocket).expect("valid rocket instance")
}

// Standard 26-player seed (squads 1–26) — used by POST creation tests.
// Squad 27 (Lo Celso fixture) is not in the seed, so POST creation succeeds.
fn setup_client_for_post() -> Client {
    use rust_samples_rocket_restful::services::player_service;

    let connection = initialize_test_database();
    // Delete squad 27 if it was somehow seeded; Lo Celso is not in the 26-player
    // seed, so this is a no-op — but kept for symmetry with the old Vec approach.
    player_service::delete(&connection, 27).ok();
    let database = PlayerCollection::new(connection);
    let rocket = rocket::build()
        .manage(database)
        .mount("/", routes::health::routes())
        .mount("/", routes::players::routes());
    Client::tracked(rocket).expect("valid rocket instance")
}

// JSON mirror of common::player_request_for_creation() — Giovani Lo Celso, squad 27
fn player_request_for_creation_json() -> serde_json::Value {
    serde_json::json!({
        "firstName": "Giovani",
        "middleName": "",
        "lastName": "Lo Celso",
        "dateOfBirth": "1996-07-09T00:00:00.000Z",
        "squadNumber": 27,
        "position": "Central Midfield",
        "abbrPosition": "CM",
        "team": "Real Betis Balompié",
        "league": "La Liga",
        "starting11": false
    })
}

// PUT fixture for Emiliano Martínez targeting squad 23.
// squadNumber is deliberately set to 99 (≠ 23) to prove the route param wins.
fn player_request_for_update_json() -> serde_json::Value {
    serde_json::json!({
        "firstName": "Emiliano",
        "middleName": "",
        "lastName": "Martínez",
        "dateOfBirth": "1992-09-02T00:00:00.000Z",
        "squadNumber": 99,
        "position": "Goalkeeper",
        "abbrPosition": "GK",
        "team": "Aston Villa FC",
        "league": "Premier League",
        "starting11": true
    })
}

// GET /openapi.json -----------------------------------------------------------

// GET /openapi.json returns 200 OK with a valid OpenAPI 3 payload
#[test]
fn test_request_get_openapi_json_response_status_ok() {
    // Arrange
    use rocket_okapi::mount_endpoints_and_merged_docs;
    use rocket_okapi::settings::OpenApiSettings;
    let database = PlayerCollection::new(initialize_test_database());
    let settings = OpenApiSettings::default();
    let mut server = rocket::build().manage(database);
    mount_endpoints_and_merged_docs! {
        server, "/".to_owned(), settings,
        "/" => routes::health::get_routes_and_docs(&settings),
        "/" => routes::players::get_routes_and_docs(&settings),
    };
    let client = Client::tracked(server).expect("valid rocket instance");
    // Act
    let response = client.get("/openapi.json").dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("\"openapi\""));
}

// GET /health -----------------------------------------------------------------

// GET /health returns 200 OK
// @coderabbitai: no body assertion — health() returns `Status` only (no
// response body). `into_string()` yields `None` for a bodyless response and
// `.unwrap()` would panic. If the handler is ever updated to return a payload
// (e.g. JSON health object), this test should be extended accordingly.
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
    let response = client
        .get(format!("/players/{}", common::EXISTING_PLAYER_ID))
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::Ok);
    let body: serde_json::Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(body["id"], common::EXISTING_PLAYER_ID);
    assert_eq!(body["firstName"], "Lionel");
    assert_eq!(body["middleName"], "Andrés");
    assert_eq!(body["lastName"], "Messi");
    assert_eq!(body["dateOfBirth"], "1987-06-24T00:00:00.000Z");
    assert_eq!(body["squadNumber"], 10);
    assert_eq!(body["position"], "Right Winger");
    assert_eq!(body["abbrPosition"], "RW");
    assert_eq!(body["team"], "Paris Saint-Germain");
    assert_eq!(body["league"], "Ligue 1");
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

// GET /players/{uuid} with unknown UUID (valid format, absent from DB) returns 404 Not Found
#[test]
fn test_request_get_player_by_id_unknown_response_status_not_found() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client
        .get(format!("/players/{}", common::UNKNOWN_PLAYER_ID))
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
    assert_eq!(body["team"], "Paris Saint-Germain");
    assert_eq!(body["league"], "Ligue 1");
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
    // Arrange — 26-player seed (no squad 27), so Lo Celso can be created
    let client = setup_client_for_post();
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
    assert_eq!(response_body["firstName"], "Giovani");
    assert_eq!(response_body["middleName"], "");
    assert_eq!(response_body["lastName"], "Lo Celso");
    assert_eq!(response_body["dateOfBirth"], "1996-07-09T00:00:00.000Z");
    assert_eq!(response_body["squadNumber"], 27);
    assert_eq!(response_body["position"], "Central Midfield");
    assert_eq!(response_body["abbrPosition"], "CM");
    assert_eq!(response_body["team"], "Real Betis Balompié");
    assert_eq!(response_body["league"], "La Liga");
    assert_eq!(response_body["starting11"], false);
}

// POST /players with duplicate squad number returns 409 Conflict
#[test]
fn test_request_post_player_body_duplicate_response_status_conflict() {
    // Arrange — POST Lo Celso once, then attempt a second creation (squad 27 now exists)
    let client = setup_client();
    let body = player_request_for_creation_json();
    client
        .post("/players")
        .header(ContentType::JSON)
        .body(body.to_string())
        .dispatch();
    // Act
    let response = client
        .post("/players")
        .header(ContentType::JSON)
        .body(player_request_for_creation_json().to_string())
        .dispatch();
    // Assert
    assert_eq!(response.status(), Status::Conflict);
}

// PUT /players/squadnumber/{squad_number} -------------------------------------

// PUT /players/squadnumber/{squad_number} returns 204 No Content on success
#[test]
fn test_request_put_player_squadnumber_existing_response_status_no_content() {
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
    assert_eq!(response.status(), Status::NoContent);
    let persisted = client.get("/players/squadnumber/23").dispatch();
    let persisted_body: serde_json::Value =
        serde_json::from_str(&persisted.into_string().unwrap()).unwrap();
    assert_eq!(persisted_body["firstName"], "Emiliano");
    assert_eq!(persisted_body["lastName"], "Martínez");
    assert_eq!(persisted_body["team"], "Aston Villa FC");
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

// PUT /players/squadnumber/{squad_number} with unknown number (valid format, absent from DB) returns 404
#[test]
fn test_request_put_player_squadnumber_unknown_response_status_not_found() {
    // Arrange
    let client = setup_client();
    let body = player_request_for_update_json();
    // Act
    let response = client
        .put("/players/squadnumber/28")
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
    // Arrange — POST Lo Celso (squad 27) first, then delete by squad number
    let client = setup_client();
    client
        .post("/players")
        .header(ContentType::JSON)
        .body(player_request_for_creation_json().to_string())
        .dispatch();
    // Act
    let response = client.delete("/players/squadnumber/27").dispatch();
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

// DELETE /players/squadnumber/{squad_number} with unknown number (valid format, absent from DB) returns 404
#[test]
fn test_request_delete_player_squadnumber_unknown_response_status_not_found() {
    // Arrange
    let client = setup_client();
    // Act
    let response = client.delete("/players/squadnumber/28").dispatch();
    // Assert
    assert_eq!(response.status(), Status::NotFound);
}
