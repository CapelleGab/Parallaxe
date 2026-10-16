# Supported formats

Planned support for **0.1.0**. Video and audio playback depend on the codecs available on each operating system.

| Format | Catalog | Thumbnail | Preview | Rich metadata |
|---|:-:|:-:|:-:|:-:|
| PNG, JPEG, WebP, GIF | ✓ | ✓ | ✓ | ✓ |
| WAV, MP3, FLAC, OGG | ✓ | optional | codec-dependent | ✓ |
| MP4, WebM | ✓ | codec-dependent | codec-dependent | codec-dependent |
| GLB / glTF 2.0 | ✓ | ✓ | ✓ | ✓ |
| OBJ (+MTL) | ✓ | ✓ | ✓ | ✓ |
| STL (ASCII and binary) | ✓ | ✓ | ✓ | geometry |
| FBX, BLEND, USD, STEP | ✓ | — | — | basic |
| Any other file | ✓ | — | — | basic |

**Basic metadata** means name, path, size, extension, filesystem dates and state.

## 3D metadata

- **Triangles** are counted from decoded geometry. For OBJ, the value is labeled *after triangulation*. Lines and points produce no invented triangles.
- **Meshes and instances** are counted separately, so an instanced mesh does not hide how many triangles are actually rendered.
- **Materials, textures and animations** are shown only when the format defines them, `not_applicable` otherwise.
- **Bounds** are shown in file units. Parallaxe never assumes meters.
- **External references** (glTF URIs, MTL textures) are resolved only inside the authorized root. Missing ones are reported as warnings.

## Not in 0.1.0

Editing, conversion, FBX/BLEND/USD/STEP preview, archive browsing and remote URLs. See the [roadmap](../project/roadmap.md).
