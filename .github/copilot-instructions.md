# Copilot Instructions

## Stack
- Rust 2024 Edition (rust-toolchain.toml enforced)
- Rocket 0.5 (async web framework)
- Serde (JSON serialization)
- In-memory storage: Mutex<Vec<Player>>
- Future: SQLite (Issue #23)

## Project Patterns
- **Layered Architecture**: Routes → Services → State
  - Routes: HTTP concerns only (controllers)
  - Services: Business logic, validation, future caching
  - State: Data access layer (thread-safe)
- **Dependency Management**: Rocket state injection via `State<T>`
- **Error Handling**: `Result<T, CustomError>` with domain-specific error types
- **Async**: All route handlers are async by default

## Code Conventions
- **Naming**: snake_case (functions/vars), PascalCase (types/traits)
- **Ownership**: Minimize clones, prefer references
- **DTOs**: Separate PlayerRequest (input), Player (storage), PlayerResponse (output)
- **Modules**: Each layer in own directory (routes/, services/, state/, models/)
- **Safety**: Never unwrap() in handlers - use Result or Option

## Testing
- **Location**: Integration tests in tests/ directory
- **Pattern**: Arrange/Act/Assert with section comments
- **Naming**: `test_request_{method}_{endpoint}_{condition}_response_{verification}`
- **Fixtures**: Use dedicated fixture functions (not stubs/fakes)
- **Assertions**: Verify complete response objects, not random fields

## Avoid
- `unwrap()` or `expect()` in production paths
- Unnecessary `.clone()` calls
- Blocking operations in async handlers
- Missing error propagation with `?`
- Global mutable state without Mutex
- Inline comments between AAA test sections

---

**For detailed workflows**: Reference `#file:AGENTS.md`
