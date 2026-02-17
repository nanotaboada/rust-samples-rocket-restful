//! Sample REST API with Rust and Rocket
//!
//! This is the application entry point that initializes the Rocket web framework,
//! initializes player data, and mounts all API routes.
//!
//! The application follows a modular architecture:
//! - `models`: Data structures and conversions
//! - `state`: Thread-safe application state management
//! - `services`: Pure business logic functions
//! - `routes`: HTTP endpoint handlers
//!
//! For more details, see the project README.

#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod services;
mod state;

use state::player_collection::{initialize_players, PlayerCollection};

// ============================================================================
// ROCKET LAUNCH
// ============================================================================

#[launch]
fn rocket() -> _ {
    let players = PlayerCollection::new(initialize_players());
    rocket::build()
        .manage(players)
        .mount("/", routes::health::routes())
        .mount("/", routes::players::routes())
}
