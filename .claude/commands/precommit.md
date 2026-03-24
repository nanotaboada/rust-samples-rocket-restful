Run the pre-commit checklist for this project:

1. Remind me to update `CHANGELOG.md` `[Unreleased]` section (Added / Changed / Fixed / Removed) — I must do this manually.
2. Run `cargo fmt` — formats code in place.
3. Run `cargo clippy --all-targets --all-features -- -D warnings` — must pass clean.
4. Run `cargo test` — all tests must pass.
5. Run `cargo build` — must succeed.

Run steps 2–5, report the results clearly, then propose a branch name and commit message for my approval using the format `type(scope): description (#issue)` (max 80 chars; types: `feat` `fix` `chore` `docs` `test` `refactor` `ci` `perf`). Do not create the branch or commit until I explicitly confirm.
