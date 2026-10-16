# Security

Parallaxe reads untrusted files from users' disks: assets downloaded from marketplaces, packs of unknown origin, damaged files. Every format parser is an attack surface. Using Tauri does not make overly powerful commands safe by itself.

## Rules

1. **Originals are read-only.** No code path writes, moves, renames or deletes inside a root.
2. **The WebView manipulates IDs, not paths.** Rust resolves and bounds every path.
3. **Scoped resource access.** glTF buffers and textures are served through a protocol scoped to root + asset ID, never a free `file://` path.
4. **Canonical resolution.** `../` escaping the root, unauthorized absolute paths and symlinks pointing outside are rejected.
5. **No network from assets.** Remote URLs in glTF or metadata are not resolved in the MVP.
6. **Budgets everywhere.** File count, path length, decodable dimensions, triangles, buffer sizes, parser time. Huge values produce a reduced or unavailable preview, never a crash or unbounded allocation.
7. **Data, never code.** Stored metadata contains no executable instruction; assets never run scripts.
8. **Minimal capabilities.** Tauri capabilities are minimal, the CSP is strict, unsolicited external navigation is disabled.
9. **No telemetry, no remote crash reporter.** Logs never contain file contents; shared reports hide personal paths.
10. **Optional updates only.** Any future online update is opt-in, documented and cryptographically verified.

## Release-blocking checks

| ID | Check | Expected |
|---|---|---|
| QA-SEC-01 | glTF URI `../../../private.txt` | Access refused |
| QA-SEC-02 | Symlink in a root pointing to a private folder | Not followed |
| QA-SEC-03 | IPC call with an unknown asset ID or unauthorized library | Refused |
| QA-SEC-04 | Malicious or truncated file | No execution, no unrecoverable crash, budgets respected |
| QA-SEC-05 | JavaScript or remote URL in metadata | No navigation or silent network access |
| QA-SEC-06 | Exported diagnostic log | No asset content or secrets, personal paths minimized |
| QA-PRIV-01 | Run without network | Every MVP workflow works |
| QA-PRIV-02 | Monitor network during import and browsing | No outgoing user data |
| QA-PRIV-03 | Delete a tag or collection | Hash of every original unchanged |
| QA-PRIV-04 | Scan a read-only folder | Correct result, no write attempt |

Any failure here blocks a public release until fixed or until the vulnerable feature is safely disabled.

## Reporting

See the [security policy](https://github.com/CapelleGab/Parallaxe/blob/main/SECURITY.md). Never report vulnerabilities in public issues.
