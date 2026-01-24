# GitHub Copilot Instructions

> **⚡ Token Efficiency Note**: This is a minimal pointer file (~500 tokens, auto-loaded by Copilot).  
> For complete operational details, reference: `#file:AGENTS.md` (~2,500 tokens, loaded on-demand)  
> For specialized knowledge, use: `#file:SKILLS/<skill-name>/SKILL.md` (loaded on-demand when needed)

## 🎯 Quick Context

**Project**: Rocket-based REST API demonstrating Rust async patterns  
**Stack**: Rust 2024 • Rocket 0.5 • Serde • In-memory storage • Docker  
**Pattern**: Single-file PoC with routes + state management  
**Philosophy**: Learning-focused, start simple before modularizing

## 📐 Core Conventions

- **Naming**: snake_case (functions/variables), PascalCase (types/traits)
- **Ownership**: Minimize clones, use references where possible
- **Error Handling**: `Result<T, E>` with proper error types
- **Async**: Rocket handlers are async by default
- **Safety**: No `unwrap()` in production paths

## 🏗️ Architecture at a Glance

```
Route Handler → Mutex<Vec<Player>> → Response
        ↓
   Validation
```

- **Routes**: Rocket route handlers with guards
- **State**: Thread-safe `Mutex<Vec<Player>>` 
- **Models**: Separate structs for Request/Response/Internal
- **Serialization**: Serde for JSON (de)serialization
- **Future**: SQLite persistence planned (Issue #23)

## ✅ Copilot Should

- Generate idiomatic Rust code with proper lifetimes
- Use Rocket state management correctly (`State<AppState>`)
- Follow ownership rules (minimize unnecessary clones)
- Write tests with Rocket's testing framework
- Apply Serde attributes for JSON mapping
- Use proper HTTP status codes with Rocket's `Status`
- Handle errors with `Result` types

## 🚫 Copilot Should Avoid

- Using `unwrap()` or `expect()` in handlers
- Unnecessary `.clone()` calls
- Blocking operations in async handlers
- Missing error propagation with `?`
- Global mutable state without synchronization
- Ignoring clippy warnings

## ⚡ Quick Commands

```bash
# Run with hot reload (if cargo-watch installed)
cargo watch -x run

# Run normally
cargo run

# Test
cargo test

# Docker
docker compose up

# API: http://localhost:8000/api/players
```

## 📚 Need More Detail?

**For operational procedures**: Load `#file:AGENTS.md`  
**For Docker expertise**: *(Planned)* `#file:SKILLS/docker-containerization/SKILL.md`  
**For SQLite integration**: See GitHub Issue #23 for planned enhancement

---

💡 **Why this structure?** Copilot auto-loads this file on every chat (~500 tokens). Loading `AGENTS.md` or `SKILLS/` explicitly gives you deep context only when needed, saving 80% of your token budget!
