# Conventions

## Commits

[Conventional Commits](https://www.conventionalcommits.org/) avec une portée — `type(scope): description` — courts, en minuscules et en anglais :

```text
feat(viewer): add stl loader
fix(indexer): keep tags when a root goes offline
docs(site): explain triangle counting
```

Types : `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`.
Portées courantes : `desktop`, `core`, `indexer`, `catalog`, `viewer`, `ipc`, `ui`, `site`, `ci`, `deps`, `repo`. Un changement logique par commit, un sujet par pull request.

## Code

- Formatage automatique : `rustfmt`, Prettier.
- Des petites fonctions testées plutôt que des abstractions astucieuses.
- Commentaires rares et courts, qui expliquent le *pourquoi* : `// WAL is unsafe on network volumes`.
- Code et identifiants en anglais.

## Frontend

- **Tailwind CSS** pour le style, avec les tokens du design system définis dans `src/styles/tokens.css`. Jamais de couleur en dur dans un composant.
- **CSS classique** quand c'est plus clair que les utilitaires (canvas du viewer 3D, mises en page complexes, keyframes).
- **[Motion](https://motion.dev/)** (ex-Framer Motion) uniquement quand une animation est justifiée, toujours en respectant `prefers-reduced-motion`.
- Les composants vivent par fonctionnalité dans `src/features/`, l'UI partagée dans `src/components/`.

## Rust

- `parallaxe-core` ne dépend pas de Tauri.
- Les chemins sont des `PathBuf`/`OsString`, jamais supposés UTF-8.
- Les erreurs sont typées et converties vers l'enveloppe d'erreur IPC commune.
- `cargo clippy -- -D warnings` doit passer.
