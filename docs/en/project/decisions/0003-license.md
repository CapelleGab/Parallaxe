# ADR-0003 — Apache-2.0 license

- **Status:** accepted
- **Date:** 2026-10-17

## Context

Parallaxe is free and open source. The license must allow wide reuse of the code (including the future core library and CLI), stay compatible with the Rust and JavaScript ecosystems, and protect contributors and users.

## Options

| License | Pros | Cons |
|---|---|---|
| MIT | Simplest, universal | No explicit patent grant |
| **Apache-2.0** | Permissive, explicit patent grant and patent retaliation, NOTICE mechanism | Slightly longer, incompatible with GPLv2-only code |
| MPL-2.0 | File-level copyleft | Less common in the Tauri/Rust ecosystem, more friction for integrators |
| GPL-3.0 | Strong copyleft, would allow reusing Allusion/TagStudio code | Prevents permissive reuse of the core, deters integrations |

## Decision

**Apache-2.0** for all original code in this repository.

## Consequences

- Code from GPL projects (Allusion, TagStudio) is never copied into the codebase.
- Every dependency must be compatible with Apache-2.0 distribution; licenses are audited in CI before any release.
- Bundled third-party components (codecs, decoders) are listed with their licenses in the distributed builds.
- Contributions are accepted under Apache-2.0 (inbound = outbound), no CLA.
