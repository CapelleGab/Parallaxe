# Security Policy

Parallaxe reads untrusted files (images, audio, video, 3D models) from users' disks. Bugs that let a file escape its library folder, execute content, exhaust resources or leak data are taken seriously.

## Supported versions

No stable release exists yet. Until `1.0.0`, only the latest release receives security fixes.

## Reporting a vulnerability

**Do not open a public issue.** Use GitHub's [private vulnerability reporting](https://github.com/CapelleGab/Parallaxe/security/advisories/new).

Please include:

- the affected version and OS;
- a description of the impact;
- minimal steps or a minimal file to reproduce — only files you are allowed to share.

You should receive an acknowledgement within 7 days. We will agree on a disclosure date with you once a fix is available and credit you unless you prefer otherwise.

## In scope

- Reading files outside an authorized library root (path traversal, symlinks, glTF/MTL references).
- Code execution or network access triggered by an asset.
- Crashes or unbounded memory/CPU use from a crafted file.
- Exposure of IPC commands or Tauri capabilities beyond what the UI needs.
- Personal data leaking into logs or diagnostic exports.
