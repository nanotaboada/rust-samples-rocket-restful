//! Business logic layer.
//!
//! This module contains pure business logic functions that handle player operations
//! independently of the HTTP framework. Services receive data from routes and return
//! Results, keeping framework concerns separate from domain logic.

pub mod player_service;
