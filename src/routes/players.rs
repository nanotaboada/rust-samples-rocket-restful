//! Player CRUD endpoint handlers.
//!
//! Handles HTTP requests for player management operations, delegating business
//! logic to the player service layer and converting results to HTTP responses.
//!
//! ## Key design
//!
//! | Concern         | Key used        | Notes                              |
//! |-----------------|-----------------|------------------------------------|
//! | Surrogate key   | UUID (`id`)     | Admin route: `GET /players/{uuid}` |
//! | Natural key     | `squad_number`  | All mutation routes use this       |

use crate::models::player::{PlayerRequest, PlayerResponse};
use crate::services::player_service::{self, CreateError, UpdateError};
use crate::state::player_collection::PlayerCollection;
use rocket::{State, delete, get, http::Status, post, put, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes_spec};

/// GET /players - Retrieves all players in the collection.
///
/// # Returns
/// * `200 OK` - JSON array of all players
///
/// # Example Response
/// ```json
/// [{"id": "f10f398d-b2ff-40aa-acac-51f58d129bc7", "firstName": "Lionel", "squadNumber": 10, ...}, ...]
/// ```
#[openapi(tag = "Players")]
#[get("/players")]
fn get_all_players(players: &State<PlayerCollection>) -> Result<Json<Vec<PlayerResponse>>, Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;
    player_service::get_all(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

/// GET /players/{id} - Retrieves a specific player by UUID (admin route).
///
/// # Path Parameters
/// * `id` - UUID surrogate key of the player
///
/// # Returns
/// * `200 OK` - JSON object with player data
/// * `404 Not Found` - If no player has that UUID
///
/// # Example
/// `GET /players/f10f398d-b2ff-40aa-acac-51f58d129bc7` returns Messi's data
#[openapi(tag = "Players")]
#[get("/players/<id>")]
fn get_player_by_id(
    id: String,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;
    player_service::get_by_id(&mut connection, &id)
        .map_err(|_| Status::InternalServerError)?
        .map(Json)
        .ok_or(Status::NotFound)
}

/// GET /players/squadnumber/{squad_number} - Retrieves a player by squad number.
///
/// # Path Parameters
/// * `squad_number` - Jersey number of the player (e.g., 10 for Messi)
///
/// # Returns
/// * `200 OK` - JSON object with player data
/// * `404 Not Found` - If no player has that squad number
///
/// # Example
/// `GET /players/squadnumber/10` finds the player wearing jersey #10
#[openapi(tag = "Players")]
#[get("/players/squadnumber/<squad_number>")]
fn get_player_by_squad_number(
    squad_number: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;
    player_service::get_by_squad_number(&mut connection, squad_number)
        .map_err(|_| Status::InternalServerError)?
        .map(Json)
        .ok_or(Status::NotFound)
}

/// POST /players - Creates a new player with an auto-generated UUID.
///
/// # Request Body
/// JSON object with player data (ID will be assigned automatically)
///
/// # Returns
/// * `201 Created` - JSON object with the created player including assigned UUID
/// * `409 Conflict` - If squad number is already taken
///
/// # Validation
/// * Squad numbers must be unique across all players
/// * UUID is auto-generated via `uuid::Uuid::new_v4()`
///
/// # Example Request
/// ```json
/// {"firstName": "Diego", "squadNumber": 10, ...}
/// ```
#[openapi(tag = "Players")]
#[post("/players", data = "<player_request>")]
fn create_player(
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<(Status, Json<PlayerResponse>), Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;

    match player_service::create(&mut connection, player_request.into_inner()) {
        Ok(response) => Ok((Status::Created, Json(response))),
        Err(CreateError::DuplicateSquadNumber) => Err(Status::Conflict),
        Err(CreateError::Database(_)) => Err(Status::InternalServerError),
    }
}

/// PUT /players/squadnumber/{squad_number} - Updates an existing player's information.
///
/// Uses `squad_number` as the natural key to look up the player. The squad
/// number and UUID are immutable — they are preserved from the existing record
/// regardless of what the request body contains.
///
/// # Path Parameters
/// * `squad_number` - Squad number (natural key) of the player to update
///
/// # Request Body
/// JSON object with complete player data
///
/// # Returns
/// * `204 No Content` - Player updated successfully, no body
/// * `404 Not Found` - If no player has that squad number
///
/// # Example
/// `PUT /players/squadnumber/10` with JSON body updates the player wearing jersey #10
#[openapi(tag = "Players")]
#[put("/players/squadnumber/<squad_number>", data = "<player_request>")]
fn update_player(
    squad_number: u32,
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<Status, Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;

    match player_service::update(&mut connection, squad_number, player_request.into_inner()) {
        Ok(_) => Ok(Status::NoContent),
        Err(UpdateError::NotFound) => Err(Status::NotFound),
        Err(UpdateError::Database(_)) => Err(Status::InternalServerError),
    }
}

/// DELETE /players/squadnumber/{squad_number} - Removes a player from the collection.
///
/// Uses `squad_number` as the natural key to look up the player.
///
/// # Path Parameters
/// * `squad_number` - Squad number (natural key) of the player to delete
///
/// # Returns
/// * `204 No Content` - Player successfully deleted (no response body)
/// * `404 Not Found` - If no player has that squad number
#[openapi(tag = "Players")]
#[delete("/players/squadnumber/<squad_number>")]
fn delete_player(squad_number: u32, players: &State<PlayerCollection>) -> Result<Status, Status> {
    let mut connection = players.get().map_err(|_| Status::InternalServerError)?;

    match player_service::delete(&mut connection, squad_number) {
        Ok(true) => Ok(Status::NoContent),
        Ok(false) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Returns all player-related routes for mounting in Rocket.
///
/// Returns all player routes and their OpenAPI spec for mounting.
///
/// # Usage
/// ```ignore
/// mount_endpoints_and_merged_docs! {
///     server, "/".to_owned(), settings,
///     "/" => routes::players::get_routes_and_docs(&settings),
/// }
/// ```
pub fn get_routes_and_docs(
    settings: &rocket_okapi::settings::OpenApiSettings,
) -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    openapi_get_routes_spec![
        settings:
        get_all_players,
        get_player_by_id,
        get_player_by_squad_number,
        create_player,
        update_player,
        delete_player,
    ]
}

/// Returns all player routes without OpenAPI types for callers that do not need
/// documentation (e.g. lightweight test setups).
#[allow(dead_code)]
pub fn routes() -> Vec<rocket::Route> {
    get_routes_and_docs(&rocket_okapi::settings::OpenApiSettings::default()).0
}
