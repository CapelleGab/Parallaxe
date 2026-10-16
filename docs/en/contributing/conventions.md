# Conventions

## Commits

[Conventional Commits](https://www.conventionalcommits.org/) with a scope — `type(scope): description` — short, lowercase and in English:

```text
feat(viewer): add stl loader
fix(indexer): keep tags when a root goes offline
docs(site): explain triangle counting
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`.
Common scopes: `desktop`, `core`, `indexer`, `catalog`, `viewer`, `ipc`, `ui`, `site`, `ci`, `deps`, `repo`. One logical change per commit, one topic per pull request.

## Code

- Formatting is automatic: `rustfmt`, Prettier.
- Small, tested functions over clever abstractions.
- Comments are rare and short, and explain *why*: `// WAL is unsafe on network volumes`.
- Code and identifiers are in English.

## Frontend

- **Tailwind CSS** for styling, using the design tokens defined in `src/styles/tokens.css`. Never hard-code colors in components.
- **Plain CSS** when it is clearer than utilities (3D viewer canvas, complex layouts, keyframes).
- **[Motion](https://motion.dev/)** (formerly Framer Motion) only when an animation is justified, always respecting `prefers-reduced-motion`.
- Components live by feature in `src/features/`, shared UI in `src/components/`.

## Rust

- `parallaxe-core` has no Tauri dependency.
- Paths are `PathBuf`/`OsString`, never assumed UTF-8.
- Errors are typed and mapped to the shared IPC error envelope.
- `cargo clippy -- -D warnings` must pass.
