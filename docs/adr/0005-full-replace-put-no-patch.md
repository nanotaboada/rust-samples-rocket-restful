# ADR-0005: Full-Replace PUT, No PATCH

Date: 2026-06-09

## Status

Accepted

## Context

The project needed an update semantic for player resources. Options considered:

- **PATCH only** — partial update; the client sends only changed fields.
  Requires the server to define a merge strategy: JSON Merge Patch (RFC 7396)
  treats absent fields as "keep existing" and explicit `null` as "clear field";
  JSON Patch (RFC 6902) uses an operation list. Both add complexity and require
  careful null/absent disambiguation.
- **Both PUT (full replace) and PATCH (partial update)** — maximum
  flexibility; doubles the implementation, test, and documentation surface area
  for a single-resource CRUD service.
- **PUT only (full replace)** — the client sends all fields; the server
  replaces the stored record entirely (except for immutable keys). No merge
  logic required.

## Decision

We will implement `PUT /players/squadnumber/{squad_number}` as a full resource
replacement. No `PATCH` endpoint will be provided.

## Consequences

- **Positive**: Implementation is straightforward — no merge logic, no need to
  choose between RFC 7396 and RFC 6902, no ambiguity about whether an absent
  field means "unchanged" or "set to null". Easier to test: a PUT test always
  sends a complete payload.
- **Negative**: Clients must fetch the current representation before updating a
  single field to avoid accidentally clearing other fields. This is a minor
  burden for a flat domain model with roughly ten fields.
- **Neutral**: PUT semantics are well-defined in RFC 7231: the payload
  represents the desired final state of the resource. The immutable `id` and
  `squad_number` fields are always taken from the stored record, so
  "full replace" means all *mutable* fields are replaced.
