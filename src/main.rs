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

use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::okapi::openapi3::{Info, OpenApi};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::swagger_ui::{SwaggerUIConfig, make_swagger_ui};
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
    let database = PlayerCollection::new(initialize_database());
    let settings = OpenApiSettings::default();

    let mut server = rocket::build().manage(database).mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    );

    mount_endpoints_and_merged_docs! {
        server, "/".to_owned(), settings,
        "/" => (vec![], OpenApi {
            openapi: "3.0.0".to_owned(),
            info: Info {
                title: "Players REST API".to_owned(),
                description: Some("Sample REST API with Rust and Rocket".to_owned()),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                ..Default::default()
            },
            ..Default::default()
        }),
        "/" => routes::health::get_routes_and_docs(&settings),
        "/" => routes::players::get_routes_and_docs(&settings),
    };

    server
}
