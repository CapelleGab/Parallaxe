# ADR-0003 — Licence Apache-2.0

- **Statut :** acceptée
- **Date :** 2026-10-17

## Contexte

Parallaxe est gratuit et open source. La licence doit permettre une large réutilisation du code (dont la future bibliothèque cœur et la CLI), rester compatible avec les écosystèmes Rust et JavaScript, et protéger contributeurs et utilisateurs.

## Options

| Licence | Avantages | Inconvénients |
|---|---|---|
| MIT | La plus simple, universelle | Pas de licence de brevets explicite |
| **Apache-2.0** | Permissive, licence de brevets explicite avec clause de riposte, mécanisme NOTICE | Un peu plus longue, incompatible avec du code GPLv2-only |
| MPL-2.0 | Copyleft au niveau fichier | Moins courante dans l'écosystème Tauri/Rust, plus de friction pour les intégrateurs |
| GPL-3.0 | Copyleft fort, permettrait de réutiliser le code d'Allusion/TagStudio | Empêche la réutilisation permissive du cœur, freine les intégrations |

## Décision

**Apache-2.0** pour tout le code original de ce dépôt.

## Conséquences

- Aucun code de projets GPL (Allusion, TagStudio) n'est copié dans la base.
- Chaque dépendance doit être compatible avec une distribution Apache-2.0 ; les licences sont auditées en CI avant toute release.
- Les composants tiers embarqués (codecs, décodeurs) sont listés avec leur licence dans les builds distribués.
- Les contributions sont acceptées sous Apache-2.0 (inbound = outbound), sans CLA.
