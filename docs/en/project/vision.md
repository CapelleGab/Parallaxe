# Vision

> **Parallaxe** — *See beyond your files.*

## The problem

Creators pile up reference images, textures, icons, sounds, videos, 3D models and work files in scattered folders. Finding the right resource is slow. Understanding its format or compatibility often requires opening another program. Moving files can break their dependencies. Existing tools tend to favor one media type, one platform or one way of organizing.

Parallaxe claims to invent neither the asset gallery nor the 3D viewer. Its bet is to combine them into a coherent, extensible product: **cross-platform + 3D + reliable metadata + local-first**.

## Who it is for

1. **3D artists and indie game developers** — textures, models, references, asset packs, UI sounds, technical properties.
2. **Designers, illustrators and visual artists** — references, variants, collections, tags, fast search.
3. **Video and audio creators** — sound effects, ambiences, clips and their metadata.
4. **Creative teams and makers** — later: catalog exchange, workflows, engine integrations. Network collaboration is not part of the MVP.

## The promise

**One app to see what an asset contains, what it is worth technically and where to find it — without moving the original or sending anything to the cloud.**

After installing Parallaxe on macOS, Windows or Linux and choosing an existing folder, you can:

1. browse thousands of resources in a fluid gallery;
2. find a file by name, type, tag or property;
3. open an image, play a sound, play a supported video or manipulate a 3D model;
4. read reliable, contextualized metadata, including triangle counts when computable;
5. create tags and collections without modifying originals;
6. get your catalog back after a restart and handle missing or changed files.

## Non-negotiable principles

| Principle | Consequence |
|---|---|
| **Free and open source** | No subscription, watermark, premium lock or forced telemetry. Apache-2.0. |
| **Local-first, private by design** | No account and no Internet needed. User data is never uploaded. |
| **Truly cross-platform** | One React/TypeScript UI with Tauri 2, real builds and tests per OS. |
| **Non-destructive** | Adding a library never moves, renames or rewrites an asset. Future edits export a new copy. |
| **No forced AI** | No AI in the product. An agent may later query data through an optional local interface. |
| **Fast and resilient** | Cancellable async indexing, bounded caches, per-file error isolation. |
| **Accessible** | Keyboard, labels, visible focus, contrast, zoom and system preferences. |

## What Parallaxe will not be

- Not a Photoshop, Audacity or Blender clone — only targeted edits after the MVP.
- Not a cloud service, a SaaS DAM, a Git manager or a generative AI engine.
- Not a promise to decode every proprietary format — limits are shown.
- Not a fork of Allusion: a new Tauri 2 codebase, inspired by needs, not by copied code.
