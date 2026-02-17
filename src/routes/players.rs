//! Player CRUD endpoint handlers.
//!
//! Handles HTTP requests for player management operations, delegating business
//! logic to the player service layer and converting results to HTTP responses.

use crate::models::player::{PlayerRequest, PlayerResponse};
use crate::services::player_service::{self, CreateError, UpdateError};
use crate::state::player_collection::PlayerCollection;
use rocket::{State, delete, get, http::Status, post, put, routes, serde::json::Json};

#[get("/players")]
fn get_all_players(players: &State<PlayerCollection>) -> Json<Vec<PlayerResponse>> {
    let players = players.lock().unwrap();
    let response = player_service::get_all(&players);
    Json(response)
}

#[get("/players/<id>")]
fn get_player_by_id(
    id: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().unwrap();
    player_service::get_by_id(&players, id)
        .map(Json)
        .ok_or(Status::NotFound)
}

#[get("/players/squadnumber/<squad_number>")]
fn get_player_by_squad_number(
    squad_number: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().unwrap();
    player_service::get_by_squad_number(&players, squad_number)
        .map(Json)
        .ok_or(Status::NotFound)
}

#[post("/players", data = "<player_request>")]
fn create_player(
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<(Status, Json<PlayerResponse>), Status> {
    let mut players = players.lock().unwrap();

    match player_service::create(&mut players, player_request.into_inner()) {
        Ok(response) => Ok((Status::Created, Json(response))),
        Err(CreateError::DuplicateSquadNumber) => Err(Status::Conflict),
    }
}

#[put("/players/<id>", data = "<player_request>")]
fn update_player(
    id: u32,
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut players = players.lock().unwrap();

    match player_service::update(&mut players, id, player_request.into_inner()) {
        Ok(response) => Ok(Json(response)),
        Err(UpdateError::NotFound) => Err(Status::NotFound),
        Err(UpdateError::DuplicateSquadNumber) => Err(Status::Conflict),
    }
}

#[delete("/players/<id>")]
fn delete_player(id: u32, players: &State<PlayerCollection>) -> Status {
    let mut players = players.lock().unwrap();

    if player_service::delete(&mut players, id) {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

/// Returns all player-related routes
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
