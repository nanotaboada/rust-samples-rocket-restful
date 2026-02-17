//! Health check endpoint.
//!
//! Provides a simple endpoint for API health monitoring.

use rocket::{get, http::Status, routes};

#[get("/health")]
/// Health check route returning 200 OK if the service is running
fn health() -> Status {
    Status::Ok
}

/// Returns health check route
pub fn routes() -> Vec<rocket::Route> {
    routes![health]
}
