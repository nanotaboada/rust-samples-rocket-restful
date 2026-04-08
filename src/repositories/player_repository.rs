//! Player data access functions using Diesel DSL.
//!
//! All queries against the `players` table live here. Functions receive a
//! `&mut SqliteConnection` obtained from the connection pool and return
//! `QueryResult<T>` (`Result<T, diesel::result::Error>`).

use crate::models::player::{NewPlayer, Player, PlayerRequest};
use crate::schema::players;
use diesel::prelude::*;

/// Retrieves all players ordered by squad number ascending.
pub fn get_all(conn: &mut SqliteConnection) -> QueryResult<Vec<Player>> {
    players::table
        .order(players::squad_number.asc())
        .select(Player::as_select())
        .load(conn)
}

/// Finds a player by UUID (surrogate key).
pub fn get_by_id(conn: &mut SqliteConnection, id: &str) -> QueryResult<Option<Player>> {
    players::table
        .find(id)
        .select(Player::as_select())
        .first(conn)
        .optional()
}

/// Finds a player by squad number (natural key).
pub fn get_by_squad_number(
    conn: &mut SqliteConnection,
    squad_number: i32,
) -> QueryResult<Option<Player>> {
    players::table
        .filter(players::squad_number.eq(squad_number))
        .select(Player::as_select())
        .first(conn)
        .optional()
}

/// Returns `true` if a player with the given squad number already exists.
pub fn exists_by_squad_number(conn: &mut SqliteConnection, squad_number: i32) -> QueryResult<bool> {
    use diesel::dsl::exists;
    use diesel::select;
    select(exists(
        players::table.filter(players::squad_number.eq(squad_number)),
    ))
    .get_result(conn)
}

/// Inserts a new player row.
pub fn insert(conn: &mut SqliteConnection, new_player: &NewPlayer) -> QueryResult<usize> {
    diesel::insert_into(players::table)
        .values(new_player)
        .execute(conn)
}

/// Updates mutable fields of an existing player identified by squad number.
///
/// `id` and `squad_number` are intentionally excluded — they are immutable
/// once set and must be preserved from the existing record.
pub fn update(
    conn: &mut SqliteConnection,
    squad_number: i32,
    request: &PlayerRequest,
) -> QueryResult<usize> {
    diesel::update(players::table.filter(players::squad_number.eq(squad_number)))
        .set((
            players::first_name.eq(&request.first_name),
            players::middle_name.eq(&request.middle_name),
            players::last_name.eq(&request.last_name),
            players::date_of_birth.eq(&request.date_of_birth),
            players::position.eq(&request.position),
            players::abbr_position.eq(&request.abbr_position),
            players::team.eq(&request.team),
            players::league.eq(&request.league),
            players::starting11.eq(i32::from(request.starting11)),
        ))
        .execute(conn)
}

/// Deletes a player by squad number. Returns the number of rows affected.
pub fn delete(conn: &mut SqliteConnection, squad_number: i32) -> QueryResult<usize> {
    diesel::delete(players::table.filter(players::squad_number.eq(squad_number))).execute(conn)
}
