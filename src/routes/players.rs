//! Player CRUD endpoint handlers.
//!
//! Handles HTTP requests for player management operations, delegating business
//! logic to the player service layer and converting results to HTTP responses.
//!
//! ## Key design
//!
//! | Concern         | Key used        | Notes                              |
//! |-----------------|-----------------|-------------------------------------|
//! | Surrogate key   | UUID (`id`)     | Admin route: `GET /players/{uuid}` |
//! | Natural key     | `squad_number`  | All mutation routes use this       |

use crate::models::player::{PlayerRequest, PlayerResponse};
use crate::services::player_service::{self, CreateError, UpdateError};
use crate::state::player_collection::PlayerCollection;
use rocket::{State, delete, get, http::Status, post, put, routes, serde::json::Json};

/// GET /players - Retrieves all players in the collection.
///
/// # Returns
/// * `200 OK` - JSON array of all players
///
/// # Example Response
/// ```json
/// [{"id": "f10f398d-b2ff-40aa-acac-51f58d129bc7", "firstName": "Lionel", "squadNumber": 10, ...}, ...]
/// ```
#[get("/players")]
fn get_all_players(players: &State<PlayerCollection>) -> Result<Json<Vec<PlayerResponse>>, Status> {
    let players = players.lock().map_err(|_| Status::InternalServerError)?;
    let response = player_service::get_all(&players);
    Ok(Json(response))
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
#[get("/players/<id>")]
fn get_player_by_id(
    id: String,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().map_err(|_| Status::InternalServerError)?;
    player_service::get_by_id(&players, &id)
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
#[get("/players/squadnumber/<squad_number>")]
fn get_player_by_squad_number(
    squad_number: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().map_err(|_| Status::InternalServerError)?;
    player_service::get_by_squad_number(&players, squad_number)
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
#[post("/players", data = "<player_request>")]
fn create_player(
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<(Status, Json<PlayerResponse>), Status> {
    let mut players = players.lock().map_err(|_| Status::InternalServerError)?;

    match player_service::create(&mut players, player_request.into_inner()) {
        Ok(response) => Ok((Status::Created, Json(response))),
        Err(CreateError::DuplicateSquadNumber) => Err(Status::Conflict),
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
/// * `200 OK` - JSON object with updated player data
/// * `404 Not Found` - If no player has that squad number
///
/// # Example
/// `PUT /players/squadnumber/10` with JSON body updates the player wearing jersey #10
#[put("/players/squadnumber/<squad_number>", data = "<player_request>")]
fn update_player(
    squad_number: u32,
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut players = players.lock().map_err(|_| Status::InternalServerError)?;

    match player_service::update(&mut players, squad_number, player_request.into_inner()) {
        Ok(response) => Ok(Json(response)),
        Err(UpdateError::NotFound) => Err(Status::NotFound),
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
///
/// # Note
/// Deletion is permanent and cannot be undone (in-memory storage).
#[delete("/players/squadnumber/<squad_number>")]
fn delete_player(squad_number: u32, players: &State<PlayerCollection>) -> Result<Status, Status> {
    let mut players = players.lock().map_err(|_| Status::InternalServerError)?;

    if player_service::delete(&mut players, squad_number) {
        Ok(Status::NoContent)
    } else {
        Err(Status::NotFound)
    }
}

/// Returns all player-related routes for mounting in Rocket.
///
/// Collects all player endpoint handlers into a vector for registration
/// with Rocket's routing system.
///
/// # Usage
/// ```ignore
/// rocket::build().mount("/", routes::players::routes())
/// ```
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_all_players,
        get_player_by_id,
        get_player_by_squad_number,
        create_player,
        update_player,
        delete_player,
    ]
}
