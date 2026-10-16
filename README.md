<div align="center">

# Parallaxe

**See beyond your files.**

A free, open source, local-first library for creative assets — images, sounds, videos and 3D models.
Organize, inspect and preview your files without moving them, without an account, without the cloud.

[Documentation](https://capellegab.github.io/Parallaxe/) · [Français](https://capellegab.github.io/Parallaxe/fr/) · [Contributing](CONTRIBUTING.md)

</div>

> [!WARNING]
> Parallaxe is in its early design phase. There is no usable release yet.

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

| Family | Formats |
|---|---|
| Images | PNG, JPEG, WebP, GIF |
| Audio | WAV, MP3, FLAC, OGG |
| Video | MP4, WebM (playback depends on system codecs) |
| 3D | GLB / glTF 2.0, OBJ (+MTL), STL — interactive viewer and geometry stats |

Any other file can still be cataloged with its basic metadata. See the [roadmap](https://capellegab.github.io/Parallaxe/project/roadmap/) for what comes next (light editing, CLI, local read-only MCP server).

## Stack

[Tauri 2](https://v2.tauri.app/) · Rust · React · TypeScript · Tailwind CSS · SQLite · Three.js

## Development

```sh
pnpm install
pnpm dev            # run the desktop app
pnpm docs:serve     # run the documentation site
```

Prerequisites and details: [Getting started](https://capellegab.github.io/Parallaxe/contributing/setup/).

## License

[Apache License 2.0](LICENSE).
