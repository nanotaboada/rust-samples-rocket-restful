#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{State, http::Status};
use std::sync::Mutex;

// ============================================================================
// DATA MODELS
// ============================================================================

/// Internal Player entity for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Player {
    id: u32,
    first_name: String,
    middle_name: String,
    last_name: String,
    date_of_birth: String,
    squad_number: u32,
    position: String,
    abbr_position: String,
    team: String,
    league: String,
    starting11: bool,
}

/// Request model for creating/updating players (no ID)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerRequest {
    first_name: String,
    middle_name: String,
    last_name: String,
    date_of_birth: String,
    squad_number: u32,
    position: String,
    abbr_position: String,
    team: String,
    league: String,
    starting11: bool,
}

/// Response model for API output (includes ID)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerResponse {
    id: u32,
    first_name: String,
    middle_name: String,
    last_name: String,
    date_of_birth: String,
    squad_number: u32,
    position: String,
    abbr_position: String,
    team: String,
    league: String,
    starting11: bool,
}

// ============================================================================
// CONVERSIONS
// ============================================================================

impl PlayerRequest {
    fn to_player(&self, id: u32) -> Player {
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

// ============================================================================
// STATE MANAGEMENT
// ============================================================================

type PlayerCollection = Mutex<Vec<Player>>;

/// Load players from JSON file at startup.
///
/// # Panics
///
/// Panics if `players.json` is missing or malformed. This is intentional
/// fail-fast behavior - the application cannot function without player data.
///
/// # Path Requirements
///
/// Expects `players.json` in the current working directory. When running:
/// - Development: `cargo run` from project root
/// - Production: Execute binary from directory containing `players.json`
fn load_players() -> Vec<Player> {
    let data = std::fs::read_to_string("players.json")
        .expect("Failed to read players.json - ensure file exists in working directory");
    serde_json::from_str(&data).expect("Failed to parse players.json - check JSON syntax")
}

// ============================================================================
// ROUTE HANDLERS
// ============================================================================

#[get("/")]
fn index() -> &'static str {
    "Sample REST API with Rust and Rocket"
}

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

#[get("/players")]
fn get_all_players(players: &State<PlayerCollection>) -> Json<Vec<PlayerResponse>> {
    let players = players.lock().unwrap();
    let response: Vec<PlayerResponse> = players
        .iter()
        .map(|p| PlayerResponse::from(p.clone()))
        .collect();
    Json(response)
}

#[get("/players/<id>")]
fn get_player_by_id(
    id: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().unwrap();
    players
        .iter()
        .find(|p| p.id == id)
        .map(|p| Json(PlayerResponse::from(p.clone())))
        .ok_or(Status::NotFound)
}

#[get("/players/squadnumber/<squad_number>")]
fn get_player_by_squad_number(
    squad_number: u32,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let players = players.lock().unwrap();
    players
        .iter()
        .find(|p| p.squad_number == squad_number)
        .map(|p| Json(PlayerResponse::from(p.clone())))
        .ok_or(Status::NotFound)
}

#[post("/players", data = "<player_request>")]
fn create_player(
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<(Status, Json<PlayerResponse>), Status> {
    let mut players = players.lock().unwrap();

    // Check for duplicate squad number
    if players
        .iter()
        .any(|p| p.squad_number == player_request.squad_number)
    {
        return Err(Status::Conflict);
    }

    // Generate new ID (max + 1)
    let new_id = players.iter().map(|p| p.id).max().unwrap_or(0) + 1;

    // Create and add new player
    let new_player = player_request.into_inner().to_player(new_id);
    let response = PlayerResponse::from(new_player.clone());
    players.push(new_player);

    Ok((Status::Created, Json(response)))
}

#[put("/players/<id>", data = "<player_request>")]
fn update_player(
    id: u32,
    player_request: Json<PlayerRequest>,
    players: &State<PlayerCollection>,
) -> Result<Json<PlayerResponse>, Status> {
    let mut players = players.lock().unwrap();

    // Find the player index
    let player_index = players
        .iter()
        .position(|p| p.id == id)
        .ok_or(Status::NotFound)?;

    // Check for duplicate squad number (excluding current player)
    if players
        .iter()
        .enumerate()
        .any(|(idx, p)| idx != player_index && p.squad_number == player_request.squad_number)
    {
        return Err(Status::Conflict);
    }

    // Update player
    let updated_player = player_request.into_inner().to_player(id);
    let response = PlayerResponse::from(updated_player.clone());
    players[player_index] = updated_player;

    Ok(Json(response))
}

#[delete("/players/<id>")]
fn delete_player(id: u32, players: &State<PlayerCollection>) -> Status {
    let mut players = players.lock().unwrap();

    let initial_len = players.len();
    players.retain(|p| p.id != id);

    if players.len() < initial_len {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

// ============================================================================
// ROCKET LAUNCH
// ============================================================================

#[launch]
fn rocket() -> _ {
    let players = PlayerCollection::new(load_players());
    rocket::build().manage(players).mount(
        "/",
        routes![
            index,
            health,
            get_all_players,
            get_player_by_id,
            get_player_by_squad_number,
            create_player,
            update_player,
            delete_player,
        ],
    )
}
