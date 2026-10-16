# Requirements

## MVP 0.1.0 — must have

| ID | Feature | Requirement |
|---|---|---|
| FR-01 | Libraries | Add one or more folders through the OS dialog; sources are never modified. |
| FR-02 | Indexing | Async recursive scan with progress, cancel, resume and per-file errors. |
| FR-03 | Persistence | Durable SQLite for paths, identities, tags, collections, index and state. |
| FR-04 | Browsing | Virtualized grid, sorting, selection, thumbnail sizes, simple search. |
| FR-05 | Organization | Tags, favorites, virtual collections, multi-file assignment. |
| FR-06 | Search and filters | Name, extension, media family, folder, tags, size/dimension ranges, availability. |
| FR-07 | Images | PNG/JPEG/WebP/GIF thumbnails and previews; dimensions, alpha, size, format. |
| FR-08 | Audio | WAV/MP3/FLAC/OGG preview when decodable; duration and verifiable properties. |
| FR-09 | Video | MP4/WebM basic metadata, playback and thumbnail when the codec allows; clear fallback otherwise. |
| FR-10 | Interactive 3D | GLB/glTF, OBJ and STL with orbit, zoom, recenter, background and wireframe. |
| FR-11 | 3D inspection | Triangles, vertices, meshes, materials, textures, animations with explicit methods. |
| FR-12 | External resources | Detect explicit references (glTF URIs, OBJ/MTL) and report missing ones. |
| FR-13 | File lifecycle | Handle deleted, unavailable, modified or moved files without silently losing annotations. |
| FR-14 | OS reliability | Installable builds for macOS, Windows, Linux; keyboard, zoom, no login. |
| FR-15 | Privacy and security | No network needed, no external file executed, access limited to chosen folders. |
| FR-16 | Generic metadata | Any file: name, path, size, extension, filesystem dates, state, open externally. |

## Should have (non-blocking)

Saved and advanced combined searches · on-demand duplicate detection by strong hash, never auto-deleting · codec-independent video thumbnails if legally redistributable · readable JSON catalog export and controlled backup/restore · full animated GIF preview and advanced color metadata.

## After the MVP

- **Image mini-studio:** crop, resize, rotate, brightness/contrast, conversion, non-destructive export.
- **Audio mini-studio:** trim, fade, gain/normalize, conversion, separate export.
- **Simple 3D actions:** orientation, scale, UV/normal inspection, supported conversions, derived export.
- **Asset relations:** dependency graph, broken references, manual project associations.
- **More formats:** FBX, BLEND, USD, STEP, PSD/EXR/HEIF — only after parser, license and stability review.
- **Creative integrations:** "open in", export to Blender/Godot/Unreal through consented connectors.
- **Automation:** CLI, local API, then optional MCP and Skill, read-only first.

## Won't do for now

Cloud sync, user accounts, multi-user collaboration, marketplace, commercial rights management, full video editing, DAW, 3D retopology, generative AI, automatic OCR, editing source files by default, public Internet API.

## Key user stories

**Add a library.** I pick an existing folder and see my assets without moving them. Nothing in the source is created, renamed or modified; permission errors do not block other files; adding the same root twice is detected.

**Find an asset.** I filter my catalog and find a file in a few actions. Search ignores case and handles accents; active filters stay visible and removable; sorting is stable across restarts; "no asset" and "no result" are different empty states.

**Organize without touching originals.** An asset can be in many collections and carry many tags. Deleting a tag or collection never deletes a file.

**Inspect a 3D model.** Orbit, zoom and recenter work and the UI recovers from load failures. Triangles are computed when the topology allows; values are shown only when relevant, "unavailable" otherwise, never an invented zero.

**Survive missing and changed files.** A disconnected folder is "unavailable", not "emptied". Annotations come back when a reliable match is found. An ambiguous move never merges records automatically.

**Work without leaking anything.** Everything works offline; no content or metadata is uploaded; catalog and cache files can be located and deleted.

## Non-functional requirements

| ID | Requirement | Validation |
|---|---|---|
| NFR-01 | One UI codebase for 3 OS | Native builds and tests per target |
| NFR-02 | Offline by default | Run with network blocked; inspect traffic |
| NFR-03 | Responsiveness | UI interactive during scans and thumbnail generation |
| NFR-04 | Resilience | Crash/reopen, corrupt assets, unplugged drives, cancellation |
| NFR-05 | Security | Minimal-privilege IPC, no script execution from assets |
| NFR-06 | Accessibility | Full keyboard paths, contrast, focus, resizable text |
| NFR-07 | Compatibility | Formats, codecs and limits documented per platform |
| NFR-08 | Openness | Public code, contribution docs, documented exports |
| NFR-09 | No fake data | "Unknown" or "not computable" when no reliable method exists |

### Performance targets (to be confirmed by benchmark)

Reference corpus: **10,000 assets** on a local SSD. Search p95 **≤ 300 ms**; no perceived freeze over **200 ms** on usual interactions with indexed data. Indexing time, memory and 3D load thresholds will be set after the `0.0.x` benchmarks.

## Measuring success without telemetry

Supervised tests with at least 8 external testers from three profiles, ≥ 90% success on critical tasks, 100% of reference 3D samples correct or explicitly unavailable, zero altered originals, zero network access required.
