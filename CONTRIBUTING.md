# Contributing to Parallaxe

Thanks for your interest! Parallaxe is an early-stage project, so the most valuable contributions right now are **feedback, bug reports and small, focused pull requests**.

Contributions are welcome in English or French. _Les contributions en français sont les bienvenues._

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Ground rules

These come from the project's core principles and are checked in every review:

1. **Originals are read-only.** No code path may write, move, rename or delete a file inside a user's library folder. Caches and the database live outside them.
2. **No account, no required network, no telemetry.** Do not add a dependency that phones home by default.
3. **No invented values.** `unavailable`, `not_applicable` and `0` are different things. Technical metadata carries a confidence and a method.
4. **The WebView never receives free-form paths.** It works with asset and root IDs; paths are resolved and bounded in Rust.
5. **Assets are untrusted data.** Parsers have size, time and memory budgets and never execute content.
6. **No copied GPL code.** Parallaxe is Apache-2.0; do not paste code from GPL projects such as Allusion or TagStudio.

## Before you start

- **Bugs:** open an issue with steps to reproduce, OS and version. Do not attach files you are not allowed to redistribute.
- **Features:** open an issue or a discussion first. Anything outside the [roadmap](https://capellegab.github.io/Parallaxe/project/roadmap/) needs agreement before code.
- **Security issues:** never in public issues — see [SECURITY.md](SECURITY.md).

## Development setup

Prerequisites: [Rust](https://rustup.rs/) (the version pinned in `rust-toolchain.toml` is installed automatically), [Node.js](https://nodejs.org/) 22+, [pnpm](https://pnpm.io/) 10+, the [Tauri 2 system dependencies](https://v2.tauri.app/start/prerequisites/) for your OS, and [uv](https://docs.astral.sh/uv/) for the documentation site.

```sh
pnpm install
pnpm dev          # desktop app
pnpm docs:serve   # documentation at http://127.0.0.1:8000
```

## Checks

Run these before opening a pull request — CI runs them on macOS, Windows and Linux:

```sh
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Coverage

Coverage is collected in CI and published on [Codecov](https://codecov.io/gh/CapelleGab/Parallaxe). To check it locally:

```sh
pnpm coverage                                  # frontend, report in apps/desktop/coverage/
cargo install cargo-llvm-cov                   # once
cargo llvm-cov --workspace --open              # rust, HTML report
```

New code should come with tests; business rules in `parallaxe-core` are expected to be fully covered.

## Code style

- Formatting is automatic: `rustfmt` for Rust, Prettier for TypeScript, CSS, JSON and Markdown.
- Prefer small, tested functions over clever abstractions.
- Comments are rare and short. Explain _why_, not _what_: `// WAL is unsafe on network volumes`.
- Code, identifiers and commit messages are in English.

## Commits and pull requests

We use [Conventional Commits](https://www.conventionalcommits.org/) with a scope: `type(scope): description`, short and lowercase.

```text
feat(viewer): add stl loader
fix(indexer): keep tags when a root goes offline
docs(site): explain triangle counting
chore(deps): bump tauri
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`.
Common scopes: `desktop`, `core`, `indexer`, `catalog`, `viewer`, `ipc`, `ui`, `site`, `ci`, `deps`, `repo`.

- One topic per pull request, one logical change per commit.
- Link the related issue and describe how you tested on which OS.
- Update the documentation (English **and** French when you can) and `CHANGELOG.md` for user-facing changes.

## Architecture decisions

Structural choices are recorded as ADRs in the [documentation](https://capellegab.github.io/Parallaxe/project/decisions/). Propose a new ADR in a pull request when your change introduces one.

## License

By contributing, you agree that your contributions are licensed under the [Apache License 2.0](LICENSE).
