# ADR-0009: Use Ballon d'Or-Themed Semantic Versioning

Date: 2026-06-10

## Status

Accepted

## Context

The project uses Semantic Versioning (MAJOR.MINOR.PATCH) for release numbers.
Release codenames — appended as a suffix to the version tag — need a convention
that is memorable, consistent within this repository, and thematically
appropriate for a football-domain project.

Options considered: standard semver only (no names), animal names (arbitrary,
no thematic link), city names (too generic), and names drawn from the football
domain (players, clubs, tournaments).

Each of the six sibling repositories in the cross-language comparison series
adopts its own football-themed naming category. This repository uses Ballon
d'Or award nominees — one of the sport's most prestigious individual honours.

## Decision

Append an alphabetically ordered Ballon d'Or nominee surname to every release
tag. Format: `v{MAJOR}.{MINOR}.{PATCH}-{surname}` (e.g. `v1.0.0-aguero`).

The full A–Z list is documented in `CHANGELOG.md`. Surnames are assigned
sequentially; the current position is tracked by the last released tag.
The CD pipeline validates the codename against the sequence before publishing.

## Consequences

### Positive

- Alphabetical progression makes release ordering unambiguous at a glance,
  independent of dates.
- The Ballon d'Or theme is instantly recognisable in a football-domain project
  and carries cultural weight: nominees include multiple World Cup winners and
  the sport's most celebrated players.
- The convention is deterministic — the next codename is always known in advance.

### Negative

- Non-standard: tooling that expects pure semver tags must be configured to
  ignore the suffix.
- The A–Z sequence has 26 slots before a wrap-around or convention change is
  needed.
- The CD pipeline must validate codenames, adding a step not present in pure
  semver workflows.

### Neutral

- The full A–Z list and current position are maintained in `CHANGELOG.md`.
- The codename is purely cosmetic — semver carries the semantic meaning.
