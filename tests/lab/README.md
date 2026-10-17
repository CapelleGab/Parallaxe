# Test lab

`generate.py` builds a complete, realistic library to try Parallaxe by hand: every supported format, the edge cases from the [quality plan](https://capellegab.github.io/Parallaxe/project/quality/) and a few traps. Everything is generated locally, so the files are free to use and the lab can be rebuilt at any time.

```sh
python3 tests/lab/generate.py ~/ParallaxeLab          # create
python3 tests/lab/generate.py ~/ParallaxeLab --force  # rebuild the library folders
```

Requirements: Python 3.10+, [Pillow](https://python-pillow.org/) and [ffmpeg](https://ffmpeg.org/) with libx264, libvpx, libmp3lame, libopus and flac. The external disk image is created on macOS only.

## Contents

| Folder                      | Purpose                                                                                                    |
| --------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `Castle Kit Library/`       | The library folder to pick on first run: images, audio, video, 3D, misc files, traps and 3,000 bulk images |
| `Godot Project - Dungeon/`  | An external folder, with a hidden `.godot/` cache that must be ignored                                     |
| `Outside Library/`          | Must **never** appear in Parallaxe, even through symlinks or glTF references                               |
| `External Disk.sparseimage` | A disk to plug (`hdiutil attach`) and unplug (`hdiutil detach`) to test offline folders                    |
| `expected.json`             | Values a correct implementation must report: scan totals, triangle counts, refused references…             |

The traps include empty, truncated and disguised files, headers claiming 40,000 × 40,000 pixels, Unicode, emoji and 200-character names, a 12-level deep folder, hidden files, symlinks that loop or leave the library, and a folder without read permission.

`expected.json` is the oracle for future automated tests. Values for 3D files were checked with the [Khronos glTF validator](https://github.com/KhronosGroup/glTF-Validator).

To delete a lab on macOS or Linux, first run `chmod 755 "Castle Kit Library/Traps/Locked folder"`.
