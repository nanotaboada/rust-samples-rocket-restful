//! Player CRUD endpoint handlers.
//!
//! Handles HTTP requests for player management operations, delegating business
//! logic to the player service layer and converting results to HTTP responses.

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
/// [{"id": 1, "firstName": "Lionel", "squadNumber": 10, ...}, ...]
/// ```
#[get("/players")]
fn get_all_players(players: &State<PlayerCollection>) -> Result<Json<Vec<PlayerResponse>>, Status> {
    let players = players.lock().map_err(|_| Status::InternalServerError)?;
    let response = player_service::get_all(&players);
    Ok(Json(response))
}

/// GET /players/{id} - Retrieves a specific player by ID.
///
/// # Path Parameters
/// * `id` - Unique player identifier
///
/// # Returns
/// * `200 OK` - JSON object with player data
/// * `404 Not Found` - If player ID doesn't exist
///
/// # Example
/// `GET /players/10` returns Messi's data (if ID 10 exists)
#[get("/players/<id>")]
fn get_player_by_id(
    id: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().map_err(|_| Status::InternalServerError)?;
    player_service::get_by_id(&players, id)
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

/// POST /players - Creates a new player with auto-generated ID.
///
/// # Request Body
/// JSON object with player data (ID will be assigned automatically)
///
/// # Returns
/// * `201 Created` - JSON object with the created player including assigned ID
/// * `409 Conflict` - If squad number is already taken
///
/// # Validation
/// * Squad numbers must be unique across all players
/// * ID is auto-generated as max(existing IDs) + 1
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

/// PUT /players/{id} - Updates an existing player's information.
///
/// # Path Parameters
/// * `id` - ID of the player to update
///
/// # Request Body
/// JSON object with complete player data (ID in URL is preserved)
///
/// # Returns
/// * `200 OK` - JSON object with updated player data
/// * `404 Not Found` - If player ID doesn't exist
/// * `409 Conflict` - If new squad number is already taken by another player
///
/// # Validation
/// * Player can keep their current squad number without conflict
/// * Changing to another player's squad number triggers 409 Conflict
///
/// # Example
/// `PUT /players/1` with JSON body updates player with ID 1
#[put("/players/<id>", data = "<player_request>")]
fn update_player(
    id: u32,
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut players = players.lock().map_err(|_| Status::InternalServerError)?;

    match player_service::update(&mut players, id, player_request.into_inner()) {
        Ok(response) => Ok(Json(response)),
        Err(UpdateError::NotFound) => Err(Status::NotFound),
        Err(UpdateError::DuplicateSquadNumber) => Err(Status::Conflict),
    }
}

/// DELETE /players/{id} - Removes a player from the collection.
///
/// # Path Parameters
/// * `id` - ID of the player to delete
///
/// # Returns
/// * `204 No Content` - Player successfully deleted (no response body)
/// * `404 Not Found` - If player ID doesn't exist
///
/// # Note
/// Deletion is permanent and cannot be undone (in-memory storage).
#[delete("/players/<id>")]
fn delete_player(id: u32, players: &State<PlayerCollection>) -> Result<Status, Status> {
    let mut players = players.lock().map_err(|_| Status::InternalServerError)?;

    if player_service::delete(&mut players, id) {
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
