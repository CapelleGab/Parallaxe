# Roadmap

The roadmap is ordered by **dependencies and proof of value**, not by calendar. Development is solo for now, with time set aside to learn Rust and test all three platforms. Nothing here is a delivery promise.

Versioning: `0.0.x` are internal prototypes, `0.1.0` is the first usable public MVP, `1.0.0` is a stable release after hardening.

| Phase | Version | Visible result | Status |
|---|---|---|---|
| P0 | — | Research, competitor matrix, test corpus, 3D WebView prototype | **in progress** |
| P1 | `0.0.1` | Tauri app running on 3 OS, database schema, secure IPC, CI | next |
| P2 | `0.0.2` | Scanner, persistent index, file states, image thumbnails | |
| P3 | `0.0.3` | Fluid grid, search, tags, collections, favorites | |
| P4 | `0.0.4` | Image, audio and video previews and inspectors | |
| P5 | `0.0.5` | GLB/glTF, OBJ, STL viewer, 3D statistics, external references | |
| P6 | `0.1.0` | Security hardening, crash recovery, packaging, user tests — **public MVP** | |
| P7 | `0.2.x` | Light, non-destructive image and audio editing, exported derivatives | |
| P8 | `0.3.x` | Dependency graph, projects, 3D optimizations, more formats | |
| P9 | `0.4.x` | Reusable Rust library, CLI with stable JSON output | |
| P10 | `0.5.x` | Optional read-only local MCP server and Agent Skill | |
| P11 | `1.0.0` | Migrations, broad compatibility, accessibility, durable maintenance | |

P7 to P10 may be reordered based on feedback. `1.0.0` does not require shipping P10 if it brings no real value.

## Exit gates

- **G0 — research:** basic 3D preview works on all three OS, a differentiating use case is demonstrable, reuse of components is legally sound.
- **G1 — foundations:** app runs offline, database created, commands authorized, CI green on three OS.
- **G2 — catalog:** 10,000 files cataloged without alteration, stop/resume validated, an unplugged drive purges nothing.
- **G3 — organization:** add, search, tag and reconnect scenarios work without the console; large catalogs scroll smoothly.
- **G4 — media:** switching quickly between media does not leak players.
- **G5 — 3D:** 3D corpus validated, including instanced models, missing textures, triangle-less geometry, large and damaged files.
- **MVP:** every MUST requirement and critical QA scenario passes on all three OS, originals unchanged, limits documented, no network or account required.

## Definition of Done

Implementation, unit/integration tests, error handling, format/OS documentation, code review, non-alteration check, no navigation blocking, green builds. **A demo alone is never proof of done.**

## What does not start a new phase

The urge to add AI, a plugin system, an exotic 3D format or a spectacular screen is not a priority while users cannot reliably import, find and inspect their files.
