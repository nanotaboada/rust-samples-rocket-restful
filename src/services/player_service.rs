//! Player business logic and CRUD operations.
//!
//! Provides pure functions for player management including validation,
//! UUID generation, and CRUD operations. These functions are framework-agnostic
//! and operate on a `rusqlite::Connection`.

use crate::models::player::{PlayerRequest, PlayerResponse};
use rusqlite::Connection;
use uuid::Uuid;

/// Error types for player creation operations.
///
/// Represents validation failures that can occur when creating a new player.
#[derive(Debug)]
#[allow(dead_code)]
pub enum CreateError {
    /// The squad number is already assigned to another player
    DuplicateSquadNumber,
    /// An unexpected database error occurred
    Database(rusqlite::Error),
}

/// Error types for player update operations.
///
/// Represents failures that can occur when updating an existing player.
#[derive(Debug)]
#[allow(dead_code)]
pub enum UpdateError {
    /// No player with the given squad number exists
    NotFound,
    /// An unexpected database error occurred
    Database(rusqlite::Error),
}

/// Maps a `rusqlite::Row` to a `PlayerResponse`.
fn row_to_response(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlayerResponse> {
    let starting11_int: i32 = row.get(10)?;
    Ok(PlayerResponse {
        id: row.get(0)?,
        first_name: row.get(1)?,
        middle_name: row.get(2)?,
        last_name: row.get(3)?,
        date_of_birth: row.get(4)?,
        squad_number: row.get(5)?,
        position: row.get(6)?,
        abbr_position: row.get(7)?,
        team: row.get(8)?,
        league: row.get(9)?,
        starting11: starting11_int != 0,
    })
}

/// Retrieves all players and converts them to response format.
pub fn get_all(conn: &Connection) -> Result<Vec<PlayerResponse>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, middle_name, last_name, date_of_birth, squad_number,
                position, abbr_position, team, league, starting11
         FROM players",
    )?;
    let rows = stmt.query_map([], row_to_response)?;
    rows.collect()
}

/// Finds a player by their UUID (surrogate key, admin route).
pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<PlayerResponse>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, middle_name, last_name, date_of_birth, squad_number,
                position, abbr_position, team, league, starting11
         FROM players WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], row_to_response)?;
    rows.next().transpose()
}

/// Finds a player by their squad number (jersey number).
pub fn get_by_squad_number(
    conn: &Connection,
    squad_number: u32,
) -> Result<Option<PlayerResponse>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, middle_name, last_name, date_of_birth, squad_number,
                position, abbr_position, team, league, starting11
         FROM players WHERE squad_number = ?1",
    )?;
    let mut rows = stmt.query_map([squad_number], row_to_response)?;
    rows.next().transpose()
}

/// Creates a new player with an auto-generated UUID and validation.
///
/// Validates that the squad number is not already in use, generates a new UUID v4,
/// and inserts the player into the database.
pub fn create(conn: &Connection, request: PlayerRequest) -> Result<PlayerResponse, CreateError> {
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM players WHERE squad_number = ?1)",
            [request.squad_number],
            |row| row.get(0),
        )
        .map_err(CreateError::Database)?;

    if exists {
        return Err(CreateError::DuplicateSquadNumber);
    }

    let new_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO players (id, first_name, middle_name, last_name, date_of_birth, squad_number, position, abbr_position, team, league, starting11)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            new_id,
            request.first_name,
            request.middle_name,
            request.last_name,
            request.date_of_birth,
            request.squad_number,
            request.position,
            request.abbr_position,
            request.team,
            request.league,
            i32::from(request.starting11),
        ],
    )
    .map_err(CreateError::Database)?;

    get_by_id(conn, &new_id)
        .map_err(CreateError::Database)?
        .ok_or_else(|| CreateError::Database(rusqlite::Error::QueryReturnedNoRows))
}

/// Updates an existing player's information by squad number.
///
/// The UUID and squad number are immutable — they are preserved from the
/// existing record regardless of what the request body contains.
pub fn update(
    conn: &Connection,
    squad_number: u32,
    request: PlayerRequest,
) -> Result<PlayerResponse, UpdateError> {
    let existing = get_by_squad_number(conn, squad_number)
        .map_err(UpdateError::Database)?
        .ok_or(UpdateError::NotFound)?;

    conn.execute(
        "UPDATE players
         SET first_name = ?1, middle_name = ?2, last_name = ?3, date_of_birth = ?4,
             position = ?5, abbr_position = ?6, team = ?7, league = ?8, starting11 = ?9
         WHERE squad_number = ?10",
        rusqlite::params![
            request.first_name,
            request.middle_name,
            request.last_name,
            request.date_of_birth,
            request.position,
            request.abbr_position,
            request.team,
            request.league,
            i32::from(request.starting11),
            squad_number,
        ],
    )
    .map_err(UpdateError::Database)?;

    Ok(PlayerResponse {
        id: existing.id,
        squad_number: existing.squad_number,
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

/// Deletes a player from the database by their squad number (natural key).
///
/// Returns `true` if a row was deleted, `false` if no match was found.
pub fn delete(conn: &Connection, squad_number: u32) -> Result<bool, rusqlite::Error> {
    let affected = conn.execute(
        "DELETE FROM players WHERE squad_number = ?1",
        [squad_number],
    )?;
    Ok(affected > 0)
}
