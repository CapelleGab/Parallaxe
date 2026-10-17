# Test lab

Automated tests cover the code; the test lab lets you try Parallaxe by hand on a realistic library.

```sh
python3 tests/lab/generate.py ~/ParallaxeLab
```

It builds, in about ten seconds and without downloading anything:

- a library folder with every supported image, audio, video and 3D format, plus 3,000 bulk images;
- traps: corrupted and disguised files, oversized image headers, Unicode names, deep folders, hidden files, symlinks leaving the library, an unreadable folder;
- an external folder, a folder that must never be cataloged and, on macOS, a disk image to plug and unplug;
- `expected.json`, the values a correct implementation must report.

## Manual checks

| Scenario | Expected |
|---|---|
| Create a library on `Castle Kit Library` | 3,073 files; nothing from `Outside Library`; 3 symlinks skipped; the locked folder does not stop the scan |
| Thumbnails | The four `reference_orientation_*` photos point up; oversized and broken images keep their icon |
| Delete, change and add files, then rescan | Deleted files are flagged missing, never removed; changed images get a new thumbnail |
| Add `Godot Project - Dungeon` as external folder | 4 more files; `.godot/` ignored |
| Plug the disk, add it, unplug it, rescan | The folder goes offline; its files stay, marked offline |
| Cancel a rescan | Nothing is flagged missing |

Requirements and details: [`tests/lab/README.md`](https://github.com/CapelleGab/Parallaxe/blob/main/tests/lab/README.md).
