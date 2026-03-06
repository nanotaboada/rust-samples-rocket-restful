//! Health check endpoint.
//!
//! Provides a simple liveness probe so load balancers and monitoring tools
//! can verify that the API process is running and accepting connections.

use rocket::{get, http::Status, routes};

/// Returns `200 OK` if the service is running.
///
/// ## Rocket note: `#[get]` placement
/// The `#[get("/health")]` attribute must appear *directly above* the function
/// signature, with the doc comment (`///`) placed *above the attribute*. This
/// order — doc comment → attribute → `fn` — is the Rust convention for all
/// annotated items.
#[get("/health")]
fn health() -> Status {
    Status::Ok
}

/// Registers the health check route with Rocket.
///
/// Each route module exposes a `routes()` function that collects its handlers
/// for mounting in [`main`](crate). This keeps route registration centralised
/// and makes it easy to add or remove endpoints without touching `main.rs`.
pub fn routes() -> Vec<rocket::Route> {
    routes![health]
}
