# Concepts

## Vocabulary

Library
:   A logical set of indexed assets. A library contains one or more roots.

Root
:   A folder you explicitly chose. Parallaxe scans it recursively and never writes inside it.

Asset
:   A catalog entry linked to a physical file — or to a file that is temporarily missing.

Tag
:   A reusable label, stored only in the catalog. Unique per library, case-insensitive.

Collection
:   A virtual group of assets. An asset can belong to any number of collections. Deleting a collection never deletes a file.

Thumbnail
:   A cached derivative that can always be regenerated.

Dependency
:   An explicit reference from one file to another, such as a glTF texture URI or an OBJ material file.

Original
:   The file in your folder. No action in 0.1.0 writes, moves or deletes it.

## Asset states

| State | Meaning |
|---|---|
| `available` | The file is present and readable. |
| `missing` | The file is gone from its path. Tags and collections are kept until you clean up explicitly. |
| `root_offline` | The whole root is unreachable (for example an unplugged external drive). Nothing is purged. |
| `unsupported` | The file is cataloged, but this format or feature is not supported. |
| `error` | Reading failed for this file only. Other files are not affected. |

## Value confidence

Every technical value carries a confidence level:

| Confidence | Meaning |
|---|---|
| `exact` | Read or computed reliably. |
| `estimated` | Derived with a documented method, for example triangles after OBJ triangulation. |
| `unavailable` | Could not be determined. Never displayed as `0`. |
| `not_applicable` | Does not exist for this format, for example animations in an STL file. |

## What happens when files change

- **Same path, new content:** the asset keeps its identity; previews and metadata are refreshed, tags are kept.
- **Renamed or moved, reliably detected:** the path is updated, annotations follow.
- **Ambiguous move:** the asset stays `missing` and you are asked to reassign it. Records are never merged silently.
- **Drive unplugged:** the root becomes `root_offline` and is reconciled when it comes back.
