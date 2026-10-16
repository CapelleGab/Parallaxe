# Parallaxe — guide for Claude

Free, open source, local-first library for creative assets (macOS, Windows, Linux).
Tauri 2 · React + TypeScript · Tailwind CSS · Rust · SQLite · Three.js.

## Sources of truth

- `docs/en/project/` (mirrored in `docs/fr/`): vision, roadmap, requirements, architecture, security, quality, design.
- `docs/en/project/decisions/`: ADRs. Any new structural decision gets an ADR, in both languages.
- `private/` (git-ignored, may be absent): the maintainer's full planning documents and bonus ideas. Bonus ideas are **out of scope** unless the maintainer says otherwise.

## Current phase

P0 — research and benchmark. The skeleton exists (`apps/desktop`, `crates/parallaxe-core`); no product feature is implemented yet. Next step: P1 (`0.0.1`) — database schema, secure IPC, CI green on three OS.

## Non-negotiable rules

1. **Originals are read-only.** Never write, move, rename or delete inside a library root. Caches and DB live elsewhere.
2. **No account, no required network, no telemetry** (ADR-0002).
3. **No invented values:** `unavailable` ≠ `0` ≠ `not_applicable`. Metrics carry `confidence` and `method` (`TechnicalValue` in core).
4. **The WebView never receives free paths:** it uses asset/root IDs; Rust resolves and bounds paths.
5. **Assets are untrusted:** size/time/memory budgets, never executed.
6. **No AI in the product.** CLI/MCP/Skill come after the MVP, as adapters of `parallaxe-core`; MCP is read-only and off by default.
7. **No GPL code** copied in (Apache-2.0 project, ADR-0003).

## Layout

`apps/desktop` (Tauri shell in `src-tauri/`, React in `src/`) · `crates/parallaxe-core` (no Tauri dependency) · `docs/en`, `docs/fr` + `mkdocs.yml`. Create a folder only when it holds real code.

## Conventions

- Commits: `type(scope): description`, short, lowercase, English. One logical change per commit. **No `Co-Authored-By` trailer.**
- Comments: rare and short (`// why it matters`).
- Root files of the repository are in English. The docs site is bilingual (EN default, FR in `docs/fr`).
- Styling: Tailwind with tokens from `src/styles/tokens.css`; plain CSS when clearer; Motion only when an animation is justified.
- The maintainer is learning Rust: keep functions small and tested, briefly explain non-obvious Rust concepts in conversation (not in code comments).

## Checks

```sh
pnpm format:check && pnpm lint && pnpm typecheck && pnpm test
cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
pnpm docs:build
```
