<div align="center">

<img src="docs/assets/favicon.svg" width="96" height="96" alt="Parallaxe logo" />

# Parallaxe

**See beyond your files.**

A free, open source, local-first library for creative assets — images, sounds, videos and 3D models.
Organize, inspect and preview your files without moving them, without an account, without the cloud.

[![CI](https://img.shields.io/github/actions/workflow/status/CapelleGab/Parallaxe/ci.yml?branch=main&style=for-the-badge&logo=githubactions&logoColor=white&label=CI&labelColor=111418)](https://github.com/CapelleGab/Parallaxe/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/CapelleGab/Parallaxe?style=for-the-badge&logo=codecov&logoColor=white&labelColor=111418)](https://codecov.io/gh/CapelleGab/Parallaxe)
[![Docs](https://img.shields.io/github/actions/workflow/status/CapelleGab/Parallaxe/docs.yml?branch=main&style=for-the-badge&logo=materialformkdocs&logoColor=white&label=docs&labelColor=111418)](https://capellegab.github.io/Parallaxe/)
[![License](https://img.shields.io/badge/license-Apache--2.0-8DB8FF?style=for-the-badge&labelColor=111418)](LICENSE)
[![Status](https://img.shields.io/badge/status-early%20design-F6C66A?style=for-the-badge&labelColor=111418)](https://capellegab.github.io/Parallaxe/project/roadmap/)

[![Tauri](https://img.shields.io/badge/Tauri_2-24C8D8?style=flat-square&logo=tauri&logoColor=white)](https://v2.tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-149ECA?style=flat-square&logo=react&logoColor=white)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-0EA5E9?style=flat-square&logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)
[![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white)](https://sqlite.org/)
[![Three.js](https://img.shields.io/badge/Three.js-000000?style=flat-square&logo=threedotjs&logoColor=white)](https://threejs.org/)

[Documentation](https://capellegab.github.io/Parallaxe/) · [Français](https://capellegab.github.io/Parallaxe/fr/) · [Roadmap](https://capellegab.github.io/Parallaxe/project/roadmap/) · [Contributing](CONTRIBUTING.md)

</div>

> **⚠️ Early design phase** — Parallaxe is not usable yet. There is no release to download.

## Why

Creators pile up references, textures, sound effects, clips and 3D models across scattered folders. Finding the right one is slow, understanding its technical properties often means opening another program, and reorganizing folders breaks dependencies.

Parallaxe indexes the folders you already have and adds a virtual layer on top: tags, collections, favorites, previews and reliable technical metadata — triangle counts, materials, codecs, durations, missing textures.

## Principles

- **Your files stay untouched.** Parallaxe references your folders; it never moves, renames or rewrites an original.
- **Local-first, no account.** Works offline. Nothing is uploaded. No telemetry.
- **Truthful.** An unknown value is shown as unknown, never as `0`.
- **Cross-platform.** macOS, Windows and Linux from a single codebase.
- **No forced AI.** The app ships no model and needs none.

## Planned for 0.1.0

| Family | Formats                                                                 |
| ------ | ----------------------------------------------------------------------- |
| Images | PNG, JPEG, WebP, GIF                                                    |
| Audio  | WAV, MP3, FLAC, OGG                                                     |
| Video  | MP4, WebM (playback depends on system codecs)                           |
| 3D     | GLB / glTF 2.0, OBJ (+MTL), STL — interactive viewer and geometry stats |

Any other file can still be cataloged with its basic metadata. See the [roadmap](https://capellegab.github.io/Parallaxe/project/roadmap/) for what comes next (light editing, CLI, local read-only MCP server).

## Stack

[Tauri 2](https://v2.tauri.app/) · Rust · React · TypeScript · Tailwind CSS · SQLite · Three.js

## Development

```sh
pnpm install
pnpm dev          # run the desktop app
pnpm test         # frontend tests
cargo test        # rust tests
pnpm docs:serve   # documentation site
```

Prerequisites and details: [Getting started](https://capellegab.github.io/Parallaxe/contributing/setup/).

## License

[Apache License 2.0](LICENSE).
