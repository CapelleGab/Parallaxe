# Getting started

## Prerequisites

| Tool | Version | Used for |
|---|---|---|
| [Rust](https://rustup.rs/) | stable | Native core and Tauri shell |
| [Node.js](https://nodejs.org/) | 22+ | Frontend tooling |
| [pnpm](https://pnpm.io/) | 10+ | JavaScript workspace |
| [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) | — | System libraries per OS (WebView2, WebKitGTK…) |
| [uv](https://docs.astral.sh/uv/) | latest | Documentation site |

## Run the app

```sh
git clone git@github.com:CapelleGab/Parallaxe.git
cd Parallaxe
pnpm install
pnpm dev
```

## Run the documentation

```sh
pnpm docs:serve
```

The site is served at <http://127.0.0.1:8000> with live reload.

## Useful commands

| Command | Effect |
|---|---|
| `pnpm dev` | Desktop app in development mode |
| `pnpm build` | Production build of the desktop app |
| `pnpm lint` | ESLint |
| `pnpm format` | Prettier |
| `pnpm typecheck` | TypeScript |
| `pnpm test` | Vitest |
| `cargo test --workspace` | Rust tests |
| `cargo clippy --workspace --all-targets` | Rust lints |
| `pnpm coverage` | Frontend coverage report |
| `cargo llvm-cov --workspace --open` | Rust coverage report (needs `cargo-llvm-cov`) |
| `pnpm docs:build` | Build the documentation in strict mode |

## Repository layout

```text
apps/desktop/            Tauri + React app
crates/parallaxe-core/   Rust business rules
docs/                    This site (en/ and fr/)
mkdocs.yml               Site configuration
```
