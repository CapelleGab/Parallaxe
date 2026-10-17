# ADR-0004 — A library lives in a folder

- **Status:** accepted
- **Date:** 2026-10-17

## Context

Version 0.1.0 was designed around *referencing* any number of existing folders. In practice, most creators would rather keep their assets in one place they organize with sub-folders, the way an Obsidian vault works. A single folder is also what makes a library portable to an external drive or a NAS later, without any account.

## Options

1. **References only:** a library is a set of folders scattered on the disk.
2. **Library folder only:** every asset must be copied into one folder, like Eagle.
3. **Library folder first, external folders allowed.**

## Decision

**Option 3.**

- Creating a library asks for a **name** and a **library folder**, chosen or created through the native folder picker. Users organize it with their own sub-folders; tags and collections add a virtual layer on top.
- **External folders** can still be added to a library. They are referenced in place, which keeps game, Blender or engine projects working.
- In the catalog, the library folder is the library's **primary root**; external folders are ordinary roots.

## Portability — direction for later

When network storage and sharing arrive, tags and collections will be stored inside the library folder, in a hidden `.parallaxe/` directory, as plain sync-friendly files. The local SQLite database then becomes a rebuildable index, as already planned in the architecture.

That change will amend the read-only rule to: **Parallaxe never modifies assets; it only writes inside `.parallaxe/`.** It will get its own ADR before any code writes there. Until then, nothing is written inside library folders.

SQLite itself never lives on a network volume: concurrent writes over SMB/NFS can corrupt it.

## Consequences

- Option 2 is rejected: forcing copies would duplicate files, break project paths and contradict the promise of never moving originals.
- Importing a file into the library folder (drag and drop, browser extension) becomes a natural future feature. It always **copies**, on explicit user action.
- Migration `0002` adds `roots.is_primary`, with at most one primary root per library.
