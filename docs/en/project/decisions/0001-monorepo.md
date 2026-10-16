# ADR-0001 — Single monorepo

- **Status:** accepted
- **Date:** 2026-10-17

## Context

Parallaxe will ship several products: the desktop app first, then a CLI, a local MCP server, an Agent Skill and possibly a browser extension. The CLI and MCP must be thin adapters over the same Rust core (`parallaxe-core`): same business rules, same SQLite schema, same DTOs. Development is solo, while learning Rust.

## Options

1. **Multirepo:** one repository per product.
2. **Monorepo:** one repository with a Cargo workspace (`crates/`) and a pnpm workspace (`apps/`, `packages/`).

## Decision

**Monorepo.** See the [repository layout](../architecture.md#repository-layout).

## Rationale

- A schema or rule change is **one commit** updating core, desktop, CLI and MCP together. With multiple repositories, the core would have to be published and versions kept in lockstep on every change.
- Multi-OS CI, test fixtures and security rules are shared.
- TypeScript DTOs are generated from Rust types, which works cleanly only when both live together.
- The Skill documents the CLI and MCP and must evolve and be tested with them.

## Consequences and reversibility

A crate can still be published to crates.io from the monorepo. A component moves to its own repository only if it has an **independent release cycle and no code dependency on the core** — for example a marketing site or a browser extension talking to a versioned local API.
