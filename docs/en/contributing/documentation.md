# Documentation

This site is built with [MkDocs](https://www.mkdocs.org/), [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/) and [mkdocs-static-i18n](https://ultrabug.github.io/mkdocs-static-i18n/).

## Languages

Each page exists in two folders with the same relative path:

```text
docs/en/project/vision.md   → /project/vision/
docs/fr/project/vision.md   → /fr/project/vision/
```

English is the default language. A page missing in French falls back to the English version, so a contribution in one language only is fine — mention it in the pull request.

## Adding a page

1. Create the page in `docs/en/` and, if possible, `docs/fr/`.
2. Add it to `nav` in `mkdocs.yml`. Section titles are translated in `nav_translations`.
3. Check the build: `pnpm docs:build` (strict mode, fails on broken links).

## Style

- Short sentences, active voice, second person.
- Do not describe a feature as available before it ships.
- Use the [wording guidelines](../project/design.md#wording).
