//! Player business logic and CRUD operations.
//!
//! Provides pure functions for player management including validation,
//! ID generation, and CRUD operations. These functions are framework-agnostic
//! and operate on borrowed data structures.

use crate::models::player::{Player, PlayerRequest, PlayerResponse};

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
    /// The player with the given ID does not exist
    NotFound,
    /// The squad number is already assigned to a different player
    DuplicateSquadNumber,
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

/// Finds a player by their unique ID.
///
/// # Arguments
/// * `players` - Slice of all players in the collection
/// * `id` - Unique identifier of the player to find
///
/// # Returns
/// * `Some(PlayerResponse)` if a player with the given ID exists
/// * `None` if no player has the specified ID
pub fn get_by_id(players: &[Player], id: u32) -> Option<PlayerResponse> {
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

/// Creates a new player with automatic ID assignment and validation.
///
/// Validates that the squad number is not already in use, generates a new unique ID
/// (max current ID + 1), and adds the player to the collection.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `request` - Player data from the API request (without ID)
///
/// # Returns
/// * `Ok(PlayerResponse)` with the newly created player including assigned ID
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
    let new_player = request.into_player(new_id);
    let response = PlayerResponse::from(&new_player);
    players.push(new_player);

    Ok(response)
}

/// Updates an existing player's information with validation.
///
/// Validates that the player exists and that the new squad number (if changed)
/// is not already assigned to another player. Preserves the player's original ID.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `id` - ID of the player to update
/// * `request` - New player data from the API request
///
/// # Returns
/// * `Ok(PlayerResponse)` with the updated player data
/// * `Err(UpdateError::NotFound)` if no player with the given ID exists
/// * `Err(UpdateError::DuplicateSquadNumber)` if the new squad number is taken by another player
///
/// # Note
/// A player can keep their current squad number without triggering a duplicate error.
pub fn update(
    players: &mut [Player],
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
    let updated_player = request.into_player(id);
    let response = PlayerResponse::from(&updated_player);
    players[player_index] = updated_player;

    Ok(response)
}

/// Deletes a player from the collection by their ID.
///
/// Uses `retain` to remove the player matching the given ID.
///
/// # Arguments
/// * `players` - Mutable reference to the player collection
/// * `id` - ID of the player to delete
///
/// # Returns
/// * `true` if a player was found and deleted
/// * `false` if no player with the given ID existed
pub fn delete(players: &mut Vec<Player>, id: u32) -> bool {
    let initial_len = players.len();
    players.retain(|p| p.id != id);
    players.len() < initial_len
}
