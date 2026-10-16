# Démarrer

## Prérequis

| Outil | Version | Usage |
|---|---|---|
| [Rust](https://rustup.rs/) | stable | Cœur natif et shell Tauri |
| [Node.js](https://nodejs.org/) | 22+ | Outillage frontend |
| [pnpm](https://pnpm.io/) | 10+ | Workspace JavaScript |
| [Prérequis Tauri](https://v2.tauri.app/start/prerequisites/) | — | Bibliothèques système par OS (WebView2, WebKitGTK…) |
| [uv](https://docs.astral.sh/uv/) | récente | Site de documentation |

## Lancer l'application

```sh
git clone git@github.com:CapelleGab/Parallaxe.git
cd Parallaxe
pnpm install
pnpm dev
```

## Lancer la documentation

```sh
pnpm docs:serve
```

Le site est servi sur <http://127.0.0.1:8000> avec rechargement automatique.

## Commandes utiles

| Commande | Effet |
|---|---|
| `pnpm dev` | App desktop en mode développement |
| `pnpm build` | Build de production de l'app |
| `pnpm lint` | ESLint |
| `pnpm format` | Prettier |
| `pnpm typecheck` | TypeScript |
| `pnpm test` | Vitest |
| `cargo test --workspace` | Tests Rust |
| `cargo clippy --workspace --all-targets` | Lints Rust |
| `pnpm coverage` | Rapport de couverture frontend |
| `cargo llvm-cov --workspace --open` | Rapport de couverture Rust (nécessite `cargo-llvm-cov`) |
| `pnpm docs:build` | Build de la doc en mode strict |

## Organisation du dépôt

```text
apps/desktop/            App Tauri + React
crates/parallaxe-core/   Règles métier Rust
docs/                    Ce site (en/ et fr/)
mkdocs.yml               Configuration du site
```
