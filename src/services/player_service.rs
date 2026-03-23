//! Player business logic and CRUD operations.
//!
//! Provides pure functions for player management including validation,
//! UUID generation, and CRUD operations. These functions are framework-agnostic
//! and operate on borrowed data structures.

use crate::models::player::{Player, PlayerRequest, PlayerResponse};
use uuid::Uuid;

/// Error types for player creation operations.
///
/// Represents validation failures that can occur when creating a new player.
#[derive(Debug)]
pub enum CreateError {
    /// The squad number is already assigned to another player
    DuplicateSquadNumber,
}

/// Error types for player update operations.
///
/// Represents failures that can occur when updating an existing player.
#[derive(Debug)]
pub enum UpdateError {
    /// No player with the given squad number exists
    NotFound,
}

/// Retrieves all players and converts them to response format.
///
/// # Arguments
/// * `players` - Slice of all players in the collection
///
/// # Returns
/// Vector of player responses ready for JSON serialization
///
/// # Example
/// ```ignore
/// let players = vec![/* player data */];
/// let responses = get_all(&players);
/// ```
pub fn get_all(players: &[Player]) -> Vec<PlayerResponse> {
    players.iter().map(PlayerResponse::from).collect()
}

/// Finds a player by their UUID (surrogate key, admin route).
///
/// # Arguments
/// * `players` - Slice of all players in the collection
/// * `id` - UUID of the player to find
///
/// # Returns
/// * `Some(PlayerResponse)` if a player with the given UUID exists
/// * `None` if no player has the specified UUID
pub fn get_by_id(players: &[Player], id: &str) -> Option<PlayerResponse> {
    players
        .iter()
        .find(|p| p.id == id)
        .map(PlayerResponse::from)
}

/// Finds a player by their squad number (jersey number).
///
/// # Arguments
/// * `players` - Slice of all players in the collection
/// * `squad_number` - Jersey number of the player to find
///
/// # Returns
/// * `Some(PlayerResponse)` if a player with the given squad number exists
/// * `None` if no player has the specified squad number
pub fn get_by_squad_number(players: &[Player], squad_number: u32) -> Option<PlayerResponse> {
    players
        .iter()
        .find(|p| p.squad_number == squad_number)
        .map(PlayerResponse::from)
}

/// Creates a new player with an auto-generated UUID and validation.
///
/// Validates that the squad number is not already in use, generates a new UUID v4,
/// and adds the player to the collection.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `request` - Player data from the API request (without ID)
///
/// # Returns
/// * `Ok(PlayerResponse)` with the newly created player including assigned UUID
/// * `Err(CreateError::DuplicateSquadNumber)` if the squad number is already taken
///
/// # Example
/// ```ignore
/// let mut players = vec![];
/// let request = PlayerRequest { squad_number: 10, /* ... */ };
/// let result = create(&mut players, request);
/// ```
pub fn create(
    players: &mut Vec<Player>,
    request: PlayerRequest,
) -> Result<PlayerResponse, CreateError> {
    if players
        .iter()
        .any(|p| p.squad_number == request.squad_number)
    {
        return Err(CreateError::DuplicateSquadNumber);
    }

    let new_id = Uuid::new_v4().to_string();
    let new_player = request.into_player(new_id);
    let response = PlayerResponse::from(&new_player);
    players.push(new_player);

    Ok(response)
}

/// Updates an existing player's information by squad number.
///
/// Looks up the player by their squad number (natural key). The squad number
/// and UUID are immutable — they are preserved from the existing record
/// regardless of what the request body contains.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `squad_number` - Squad number (natural key) of the player to update
/// * `request` - New player data from the API request
///
/// # Returns
/// * `Ok(PlayerResponse)` with the updated player data
/// * `Err(UpdateError::NotFound)` if no player with the given squad number exists
pub fn update(
    players: &mut [Player],
    squad_number: u32,
    request: PlayerRequest,
) -> Result<PlayerResponse, UpdateError> {
    let player_index = players
        .iter()
        .position(|p| p.squad_number == squad_number)
        .ok_or(UpdateError::NotFound)?;

    // Preserve the existing UUID and squad number — both are immutable
    let existing_id = players[player_index].id.clone();
    let mut updated_player = request.into_player(existing_id);
    updated_player.squad_number = squad_number;

    let response = PlayerResponse::from(&updated_player);
    players[player_index] = updated_player;

    Ok(response)
}

/// Deletes a player from the collection by their squad number (natural key).
///
/// Uses `retain` to remove the player matching the given squad number.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `squad_number` - Squad number of the player to delete
///
/// # Returns
/// * `true` if a player was found and deleted
/// * `false` if no player with the given squad number existed
pub fn delete(players: &mut Vec<Player>, squad_number: u32) -> bool {
    let initial_len = players.len();
    players.retain(|p| p.squad_number != squad_number);
    players.len() < initial_len
}
