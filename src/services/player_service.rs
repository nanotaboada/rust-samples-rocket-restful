//! Player business logic and CRUD operations.
//!
//! Pure functions that validate inputs, enforce invariants (duplicate squad
//! number check, UUID/squad-number immutability), and delegate all persistence
//! to [`crate::repositories::player_repository`]. No Diesel imports here.

use crate::models::player::{NewPlayer, PlayerRequest, PlayerResponse};
use crate::repositories::player_repository;
use diesel::SqliteConnection;
use uuid::Uuid;

/// Error types for player creation operations.
#[derive(Debug)]
pub enum CreateError {
    /// The squad number is already assigned to another player.
    DuplicateSquadNumber,
    /// An unexpected database error occurred.
    Database(#[allow(dead_code)] diesel::result::Error),
}

/// Error types for player update operations.
#[derive(Debug)]
pub enum UpdateError {
    /// No player with the given squad number exists.
    NotFound,
    /// An unexpected database error occurred.
    Database(#[allow(dead_code)] diesel::result::Error),
}

/// Retrieves all players ordered by squad number.
pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<PlayerResponse>, diesel::result::Error> {
    player_repository::get_all(conn)
        .map(|players| players.into_iter().map(PlayerResponse::from).collect())
}

/// Finds a player by UUID (surrogate key, admin route).
pub fn get_by_id(
    conn: &mut SqliteConnection,
    id: &str,
) -> Result<Option<PlayerResponse>, diesel::result::Error> {
    player_repository::get_by_id(conn, id).map(|opt| opt.map(PlayerResponse::from))
}

/// Finds a player by squad number (natural key).
pub fn get_by_squad_number(
    conn: &mut SqliteConnection,
    squad_number: u32,
) -> Result<Option<PlayerResponse>, diesel::result::Error> {
    player_repository::get_by_squad_number(conn, squad_number as i32)
        .map(|opt| opt.map(PlayerResponse::from))
}

/// Creates a new player with an auto-generated UUID.
///
/// Validates that the squad number is not already in use, generates a UUID v4,
/// inserts the row, and returns the persisted player.
pub fn create(
    conn: &mut SqliteConnection,
    request: PlayerRequest,
) -> Result<PlayerResponse, CreateError> {
    let exists = player_repository::exists_by_squad_number(conn, request.squad_number as i32)
        .map_err(CreateError::Database)?;

    if exists {
        return Err(CreateError::DuplicateSquadNumber);
    }

    let new_id = Uuid::new_v4().to_string();
    let new_player = NewPlayer {
        id: new_id.clone(),
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        date_of_birth: request.date_of_birth,
        squad_number: request.squad_number as i32,
        position: request.position,
        abbr_position: request.abbr_position,
        team: request.team,
        league: request.league,
        starting11: i32::from(request.starting11),
    };

    player_repository::insert(conn, &new_player).map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => CreateError::DuplicateSquadNumber,
        _ => CreateError::Database(e),
    })?;

    player_repository::get_by_id(conn, &new_id)
        .map_err(CreateError::Database)?
        .map(PlayerResponse::from)
        .ok_or(CreateError::Database(diesel::result::Error::NotFound))
}

/// Updates an existing player's information by squad number.
///
/// The UUID and squad number are immutable — they are preserved from the
/// existing record regardless of what the request body contains.
pub fn update(
    conn: &mut SqliteConnection,
    squad_number: u32,
    request: PlayerRequest,
) -> Result<PlayerResponse, UpdateError> {
    let existing = player_repository::get_by_squad_number(conn, squad_number as i32)
        .map_err(UpdateError::Database)?
        .ok_or(UpdateError::NotFound)?;

    player_repository::update(conn, squad_number as i32, &request)
        .map_err(UpdateError::Database)?;

    Ok(PlayerResponse {
        id: existing.id,
        squad_number: existing.squad_number as u32,
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        date_of_birth: request.date_of_birth,
        position: request.position,
        abbr_position: request.abbr_position,
        team: request.team,
        league: request.league,
        starting11: request.starting11,
    })
}

/// Deletes a player by squad number (natural key).
///
/// Returns `true` if a row was deleted, `false` if no match was found.
pub fn delete(
    conn: &mut SqliteConnection,
    squad_number: u32,
) -> Result<bool, diesel::result::Error> {
    player_repository::delete(conn, squad_number as i32).map(|affected| affected > 0)
}
