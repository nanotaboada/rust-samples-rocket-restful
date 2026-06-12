# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for the
`rust-samples-rocket-restful` project. An ADR captures a significant
architectural choice, the context that drove it, and the trade-offs accepted.

ADRs are immutable once accepted. If a decision changes, the original ADR is
marked `Deprecated` or `Superseded by ADR-XXXX` and a new ADR is written.

## Index

| # | Title | Status |
|---|-------|--------|
| [0001](0001-adopt-rocket-as-rest-api-framework.md) | Adopt Rocket as REST API Framework | Accepted |
| [0002](0002-four-layer-architecture.md) | Four-Layer Architecture | Accepted |
| [0003](0003-diesel-r2d2-bundled-sqlite.md) | Diesel ORM + r2d2 + Bundled SQLite | Accepted |
| [0004](0004-uuid-surrogate-squad-number-natural-key.md) | UUID Surrogate Key + Squad Number Natural Key | Accepted |
| [0005](0005-full-replace-put-no-patch.md) | Full-Replace PUT, No PATCH | Accepted |
| [0006](0006-embed-migrations-startup-schema.md) | Embedded Migrations at Startup | Accepted |
| [0007](0007-integration-only-test-strategy.md) | Integration-Only Test Strategy | Accepted |
| [0008](0008-docker-compose-strategy.md) | Docker and Compose Strategy | Accepted |
| [0009](0009-ballon-dor-themed-versioning.md) | Use Ballon d'Or-Themed Semantic Versioning | Accepted |
| [0010](0010-ai-assisted-development-workflow.md) | Adopt AI-Assisted Development Workflow | Accepted |
| [0011](0011-spec-driven-development.md) | Adopt Spec-Driven Development (SDD) | Accepted |

## Creating a New ADR

1. Copy `template.md` to `NNNN-short-title-kebab-case.md` (next sequential number).
2. Fill in all sections. Status starts as `Proposed`; update to `Accepted` once the decision is confirmed.
3. Add a row to the index table above.
4. Update `CLAUDE.md` → "Architecture Decision Records" section if the decision affects agent guidance.

## References

- [Documenting Architecture Decisions — Michael Nygard (2011)](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [ADR GitHub Organization](https://adr.github.io/)
