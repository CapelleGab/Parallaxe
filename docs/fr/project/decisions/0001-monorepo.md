# ADR-0001 — Monorepo unique

- **Statut :** acceptée
- **Date :** 2026-10-17

## Contexte

Parallaxe livrera plusieurs produits : d'abord l'app desktop, puis une CLI, un serveur MCP local, un Agent Skill et peut-être une extension navigateur. La CLI et le MCP doivent être de simples adaptateurs du même cœur Rust (`parallaxe-core`) : mêmes règles métier, même schéma SQLite, mêmes DTO. Le développement est solo, avec un apprentissage de Rust en parallèle.

## Options

1. **Multirepo :** un dépôt par produit.
2. **Monorepo :** un seul dépôt avec un workspace Cargo (`crates/`) et un workspace pnpm (`apps/`, `packages/`).

## Décision

**Monorepo.** Voir l'[organisation du dépôt](../architecture.md#organisation-du-depot).

## Raisons

- Un changement de schéma ou de règle tient en **un seul commit** qui met à jour cœur, desktop, CLI et MCP ensemble. Avec plusieurs dépôts, il faudrait publier le cœur et synchroniser les versions à chaque modification.
- CI multi-OS, fixtures de test et règles de sécurité sont partagées.
- Les DTO TypeScript sont générés depuis les types Rust, ce qui ne marche proprement que s'ils vivent ensemble.
- Le Skill documente la CLI et le MCP et doit évoluer et être testé avec eux.

## Conséquences et réversibilité

Un crate peut toujours être publié sur crates.io depuis le monorepo. Un composant ne part dans son propre dépôt que s'il a un **cycle de release indépendant et aucune dépendance de code au cœur** — par exemple un site vitrine ou une extension navigateur parlant à une API locale versionnée.
