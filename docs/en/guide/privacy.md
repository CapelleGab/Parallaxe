# Privacy

Parallaxe is built so that you never have to trust a server.

- **No account.** There is no sign-up and no login, now or later.
- **Offline by default.** Every feature of 0.1.0 works with the network disabled.
- **Nothing is uploaded.** Neither your files nor their metadata leave your computer.
- **No telemetry.** No analytics, no remote crash reporter.
- **Read-only on your folders.** The catalog and caches live in the app data directory, outside your roots.
- **You stay in control.** You can locate and delete the catalog and caches at any time.

Logs never contain file contents. Diagnostic exports hide personal paths unless you choose otherwise.

## Future integrations

A local CLI and an optional read-only MCP server are planned after the MVP. The MCP server will be off by default and limited to the libraries you allow.

!!! note
    A *local* MCP server does not mean a *local* model. If you connect a cloud AI client, that client may send the results to its provider.
