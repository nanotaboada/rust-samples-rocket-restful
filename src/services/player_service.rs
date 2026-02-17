//! Player business logic and CRUD operations.
//!
//! Provides pure functions for player management including validation,
//! ID generation, and CRUD operations. These functions are framework-agnostic
//! and operate on borrowed data structures.

use crate::models::player::{Player, PlayerRequest, PlayerResponse};

/// Error types for player creation
#[derive(Debug)]
pub enum CreateError {
    DuplicateSquadNumber,
}

/// Error types for player updates
#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    DuplicateSquadNumber,
}

/// Get all players as responses
pub fn get_all(players: &[Player]) -> Vec<PlayerResponse> {
    players
        .iter()
        .map(|p| PlayerResponse::from(p.clone()))
        .collect()
}

/// Find a player by ID
pub fn get_by_id(players: &[Player], id: u32) -> Option<PlayerResponse> {
    players
        .iter()
        .find(|p| p.id == id)
        .map(|p| PlayerResponse::from(p.clone()))
}

/// Find a player by squad number
pub fn get_by_squad_number(players: &[Player], squad_number: u32) -> Option<PlayerResponse> {
    players
        .iter()
        .find(|p| p.squad_number == squad_number)
        .map(|p| PlayerResponse::from(p.clone()))
}

/// Create a new player with validation
pub fn create(
    players: &mut Vec<Player>,
    request: PlayerRequest,
) -> Result<PlayerResponse, CreateError> {
    // Check for duplicate squad number
    if players
        .iter()
        .any(|p| p.squad_number == request.squad_number)
    {
        return Err(CreateError::DuplicateSquadNumber);
    }

    // Generate new ID (max + 1)
    let new_id = players.iter().map(|p| p.id).max().unwrap_or(0) + 1;

    // Create and add new player
    let new_player = request.to_player(new_id);
    let response = PlayerResponse::from(new_player.clone());
    players.push(new_player);

    Ok(response)
}

/// Update an existing player with validation
pub fn update(
    players: &mut Vec<Player>,
    id: u32,
    request: PlayerRequest,
) -> Result<PlayerResponse, UpdateError> {
    // Find the player index
    let player_index = players
        .iter()
        .position(|p| p.id == id)
        .ok_or(UpdateError::NotFound)?;

    // Check for duplicate squad number (excluding current player)
    if players
        .iter()
        .enumerate()
        .any(|(idx, p)| idx != player_index && p.squad_number == request.squad_number)
    {
        return Err(UpdateError::DuplicateSquadNumber);
    }

    // Update player
    let updated_player = request.to_player(id);
    let response = PlayerResponse::from(updated_player.clone());
    players[player_index] = updated_player;

    Ok(response)
}

/// Delete a player by ID, returns true if deleted
pub fn delete(players: &mut Vec<Player>, id: u32) -> bool {
    let initial_len = players.len();
    players.retain(|p| p.id != id);
    players.len() < initial_len
}
