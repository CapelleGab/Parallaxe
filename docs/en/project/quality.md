# Quality

0.1.0 can ship when a creator can **add, find, organize and inspect** assets on macOS, Windows and Linux **without damaging files**, without an account, with understandable errors. Tests protect, in order: originals, catalog durability, correctness of 3D properties, and cross-platform behavior.

## Test levels

| Level | Examples | Tooling |
|---|---|---|
| Rust unit | Paths, formats, triangle counts, tag rules, offline state | `cargo test` |
| TypeScript unit / UI | Filters, states, utilities, interactions, accessibility | Vitest + Testing Library |
| Database integration | Migrations, transactions, reconciliation | Rust + temporary SQLite |
| Media integration | Fixture decoding, previews, size limits | Targeted tests in CI |
| Desktop functional | Add folder, grid, inspector, shortcuts, native dialog | Automated where possible, manual checklist per OS |
| Security | IPC, external URIs, malformed assets, symlinks, path traversal | Unit, fuzzing, regression |
| Performance | 10k files, repeated 3D rendering, memory, crash/restart | Reproducible benchmarks |
| Exploratory UX | Real creator workflows | Guided sessions |

## Platform matrix

| Platform | Minimum | Critical variations |
|---|---|---|
| macOS | Apple Silicon | Case-sensitive/insensitive APFS, permissions, Retina, USB drives |
| Windows | x64 with WebView2 | Unicode paths, external drives, NTFS, UNC, installer |
| Linux | Stable x86_64 with WebKitGTK | Wayland/X11, GPU, removable mounts, case-sensitive FS |

At least one low-end CPU/GPU configuration and one without reliable 3D acceleration are tested too.

## Test corpus

- **Normal files:** PNG with/without alpha, EXIF-rotated JPEG, WebP, static/animated GIF; PCM WAV, VBR/CBR MP3, FLAC, OGG; H.264 MP4, unsupported-codec MP4, VP9 WebM; simple and instanced GLB, glTF + `.bin` + PNG, triangulated and quad OBJ, OBJ+MTL+textures, ASCII and binary STL, triangle-less models; unknown and fake files.
- **Pathological files:** empty, truncated, invalid headers, decompression bombs, oversized 3D buffers, identical names, long and non-Latin paths, non-UTF-8 Linux paths, cyclic symlinks, unplugged drives, nested roots, glTF references to `../secret.txt`, interrupted SQLite transactions.
- **Oracles:** each critical fixture documents its source, redistributable license, SHA-256, expected values checked by an independent tool, and tolerance. No proprietary asset packs in the public repository.

## Release gates for 0.1.0

- [ ] Every MUST requirement has an explicit `pass`, `fail` or `blocked` status, with no critical failure.
- [ ] Catalog reopens after a crash and with a missing drive, tags intact.
- [ ] GLB/glTF, OBJ, STL validated on all three OS.
- [ ] Triangle counts verified against oracles; instancing and triangulation correct or explicitly unavailable.
- [ ] Source files verified unchanged by hash after scans, tagging, collections and previews.
- [ ] No network required, no private data sent.
- [ ] Installers tested; codec limits published.
- [ ] UX tested with creators; blocking feedback fixed or transparently deferred.
- [ ] Installation, usage, contribution and security docs available.

## Bug priorities

| Priority | Examples |
|---|---|
| **P0** | Source file loss or corruption, data leak, path traversal, permission breach |
| **P1** | Corrupted database, broken critical 3D feature on a supported OS, frequent crash |
| **P2** | Degraded performance, wrong non-critical metadata, partial format support |
| **P3** | Cosmetic issues, comfort, post-MVP ideas |

Any P0 or P1 regression blocks a release.
