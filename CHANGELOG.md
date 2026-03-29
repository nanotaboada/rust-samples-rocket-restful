# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Normalize player dataset: correct Fernández/Mac Allister/Messi team data, replace hardcoded random UUIDs with deterministic UUID v5 values (namespace FIFA_WORLD_CUP_QATAR_2022_ARGENTINA_SQUAD)
- Align CRUD test fixtures: Lo Celso (squad 27) for Create and Delete, Messi (squad 10) for Retrieve, Damián Martínez (squad 23) for Update

## [0.1.0] - 2025-01-01

### Added
- Initial release: REST API for managing football players built with Rust and Rocket
- CRUD operations with in-memory thread-safe storage (`Mutex<Vec<Player>>`)
- Argentina 2022 World Cup squad seed data (26 players)
- Layered architecture: routes → services → state
- Integration tests following Arrange/Act/Assert pattern
