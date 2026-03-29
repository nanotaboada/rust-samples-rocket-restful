// Concurrency tests for PlayerCollection thread safety
// Tests the Mutex<Vec<Player>> state layer directly via Arc<PlayerCollection>
// and std::thread — blocking::Client is not Sync and cannot be shared across
// threads, so thread safety is validated at the service layer instead.

mod common;

use rust_samples_rocket_restful::{
    services::player_service,
    state::player_collection::{PlayerCollection, initialize_players},
};
use std::sync::Arc;
use std::thread;

// Concurrent reads ------------------------------------------------------------

// 10 threads reading simultaneously all observe the full 26-player collection
#[test]
fn test_concurrent_reads_all_players_no_data_races() {
    // Arrange
    let players = Arc::new(PlayerCollection::new(initialize_players()));
    let thread_count = 10;
    // Act
    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            let players = Arc::clone(&players);
            thread::spawn(move || {
                let guard = players.lock().unwrap();
                player_service::get_all(&guard).len()
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // Assert
    assert!(results.iter().all(|&count| count == 26));
}

// Concurrent creates with unique squad numbers --------------------------------

// 5 threads each creating a player with a distinct squad number all succeed
#[test]
fn test_concurrent_creates_unique_squad_numbers_all_succeed() {
    // Arrange
    let players = Arc::new(PlayerCollection::new(initialize_players()));
    let squad_numbers = [101u32, 102, 103, 104, 105];
    // Act
    let handles: Vec<_> = squad_numbers
        .iter()
        .map(|&squad_number| {
            let players = Arc::clone(&players);
            thread::spawn(move || {
                let mut request = common::player_request_for_creation();
                request.squad_number = squad_number;
                let mut guard = players.lock().unwrap();
                player_service::create(&mut guard, request).is_ok()
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // Assert
    assert!(results.iter().all(|&ok| ok));
    assert_eq!(players.lock().unwrap().len(), 31);
}

// Concurrent creates with duplicate squad number ------------------------------

// 5 threads all racing to create squad 99: exactly one succeeds, rest conflict
#[test]
fn test_concurrent_creates_duplicate_squad_number_one_succeeds() {
    // Arrange
    let players = Arc::new(PlayerCollection::new(initialize_players()));
    let thread_count = 5;
    // Act
    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            let players = Arc::clone(&players);
            thread::spawn(move || {
                let mut request = common::player_request_for_creation();
                request.squad_number = 99;
                let mut guard = players.lock().unwrap();
                player_service::create(&mut guard, request).is_ok()
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // Assert
    let successes = results.iter().filter(|&&ok| ok).count();
    assert_eq!(successes, 1);
    assert_eq!(players.lock().unwrap().len(), 27);
}

// Mixed concurrent reads and writes -------------------------------------------

// 5 readers + 5 writers running concurrently complete without panics
#[test]
fn test_concurrent_mixed_reads_and_writes_no_data_races() {
    // Arrange
    let players = Arc::new(PlayerCollection::new(initialize_players()));
    let base_squad = 201u32;
    // Act
    let mut handles = vec![];
    for _ in 0..5 {
        let players = Arc::clone(&players);
        handles.push(thread::spawn(move || {
            let guard = players.lock().unwrap();
            player_service::get_all(&guard).len() >= 26
        }));
    }
    for i in 0..5u32 {
        let players = Arc::clone(&players);
        handles.push(thread::spawn(move || {
            let mut request = common::player_request_for_creation();
            request.squad_number = base_squad + i;
            let mut guard = players.lock().unwrap();
            player_service::create(&mut guard, request).is_ok()
        }));
    }
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // Assert — all threads completed without panicking
    assert!(results.iter().all(|&ok| ok));
}
