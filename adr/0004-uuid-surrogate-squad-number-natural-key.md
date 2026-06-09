# ADR-0004: UUID Surrogate Key + Squad Number Natural Key

Date: 2026-06-09

## Status

Accepted

## Context

The project needed a stable identifier strategy for player resources. Options
considered:

- **Sequential integer primary key** — simple, compact; but auto-increment IDs
  are predictable (enumerable), which is a security concern for public-facing
  APIs.
- **UUID as the sole key for all operations** — globally unique, non-guessable;
  but consumers must look up or store the UUID to perform mutations, and UUIDs
  are meaningless in the football domain.
- **Composite key (`squad_number` + `team`)** — natural, domain-meaningful, but
  players change teams and the composite makes route paths awkward.
- **Dual key: UUID surrogate + squad number natural key** — UUID provides global
  uniqueness and non-guessability; squad number is the domain identifier that
  consumers actually know and use for mutations.

## Decision

We will use a dual-key approach:

- `id` (UUID v4, stored as `TEXT` in SQLite): primary key; used only for
  `GET /players/{uuid}`.
- `squad_number` (unique integer): natural key; used for all mutation routes
  (`PUT /players/squadnumber/{n}`, `DELETE /players/squadnumber/{n}`).

Both keys are **immutable once set**. On `PUT`, the UUID and squad number from
the stored record are always preserved; values in the request body for these
fields are silently ignored.

## Consequences

- **Positive**: UUID prevents ID enumeration attacks on the admin lookup
  endpoint. Squad number is the identifier consumers already know, making
  mutation routes ergonomic. Immutability removes the need for cascade update
  logic.
- **Negative**: SQLite has no native UUID type; the UUID is stored as a 36-char
  `TEXT` column, using slightly more space than a 16-byte binary representation.
  Two indices (UUID primary key + squad_number unique constraint) add a small
  write overhead.
- **Neutral**: `GET /players/{uuid}` is documented as an admin/internal lookup;
  consumer-facing queries can use `GET /players` (list all) and
  `PUT/DELETE /players/squadnumber/{n}`.
