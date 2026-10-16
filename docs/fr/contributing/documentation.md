# Documentation

Ce site est construit avec [MkDocs](https://www.mkdocs.org/), [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/) et [mkdocs-static-i18n](https://ultrabug.github.io/mkdocs-static-i18n/).

## Langues

Chaque page existe dans deux dossiers avec le même chemin relatif :

```text
docs/en/project/vision.md   → /project/vision/
docs/fr/project/vision.md   → /fr/project/vision/
```

L'anglais est la langue par défaut. Une page absente en français retombe sur la version anglaise : une contribution dans une seule langue est donc acceptée — précise-le dans la pull request.

## Ajouter une page

1. Crée la page dans `docs/en/` et, si possible, `docs/fr/`.
2. Ajoute-la au `nav` de `mkdocs.yml`. Les titres de sections se traduisent dans `nav_translations`.
3. Vérifie le build : `pnpm docs:build` (mode strict, échoue sur les liens cassés).

## Style

- Phrases courtes, voix active, tutoiement.
- Ne jamais présenter une fonction comme disponible avant sa sortie.
- Suivre les [règles de vocabulaire](../project/design.md#vocabulaire).
