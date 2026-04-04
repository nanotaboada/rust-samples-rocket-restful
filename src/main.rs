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

use state::player_collection::{PlayerCollection, initialize_database};

/// Configures and launches the Rocket web server.
///
/// ## Rust note: `#[launch]` macro
/// `#[launch]` is a Rocket procedural macro that generates the actual `main()`
/// function around this one. It sets up an async runtime (Tokio), calls
/// `rocket()` to build the configured instance, and starts the HTTP server.
///
/// The `-> _` return type lets the compiler infer the full generic `Rocket<Build>`
/// type, avoiding a verbose type annotation.
#[launch]
fn rocket() -> _ {
    let db = PlayerCollection::new(initialize_database());
    rocket::build()
        .manage(db)
        .mount("/", routes::health::routes())
        .mount("/", routes::players::routes())
}
