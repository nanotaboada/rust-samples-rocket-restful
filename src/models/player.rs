//! Player domain models and conversions.
//!
//! Defines the player entity structures used for storage, API requests, and responses.
//! Includes conversion implementations between different representations.

use rocket::serde::{Deserialize, Serialize};

/// Internal Player entity for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: u32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub squad_number: u32,
    pub position: String,
    pub abbr_position: String,
    pub team: String,
    pub league: String,
    pub starting11: bool,
}

/// Request model for creating/updating players (no ID)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRequest {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub squad_number: u32,
    pub position: String,
    pub abbr_position: String,
    pub team: String,
    pub league: String,
    pub starting11: bool,
}

/// Response model for API output (includes ID)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub id: u32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub squad_number: u32,
    pub position: String,
    pub abbr_position: String,
    pub team: String,
    pub league: String,
    pub starting11: bool,
}

// ============================================================================
// CONVERSIONS
// ============================================================================

impl PlayerRequest {
    /// Convert a PlayerRequest into a Player with the given ID
    pub fn to_player(&self, id: u32) -> Player {
        Player {
            id,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth.clone(),
            squad_number: self.squad_number,
            position: self.position.clone(),
            abbr_position: self.abbr_position.clone(),
            team: self.team.clone(),
            league: self.league.clone(),
            starting11: self.starting11,
        }
    }
}

impl From<Player> for PlayerResponse {
    fn from(player: Player) -> Self {
        PlayerResponse {
            id: player.id,
            first_name: player.first_name,
            middle_name: player.middle_name,
            last_name: player.last_name,
            date_of_birth: player.date_of_birth,
            squad_number: player.squad_number,
            position: player.position,
            abbr_position: player.abbr_position,
            team: player.team,
            league: player.league,
            starting11: player.starting11,
        }
    }
}
