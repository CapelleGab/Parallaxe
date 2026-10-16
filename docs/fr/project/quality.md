# Qualité

La 0.1.0 peut sortir quand un créateur peut **ajouter, retrouver, organiser et inspecter** ses assets sur macOS, Windows et Linux **sans endommager ses fichiers**, sans compte, avec des erreurs compréhensibles. Les tests protègent dans l'ordre : les originaux, la durabilité du catalogue, la justesse des propriétés 3D et le fonctionnement multiplateforme.

## Niveaux de test

| Niveau | Exemples | Outils |
|---|---|---|
| Unitaire Rust | Chemins, formats, comptage de triangles, règles de tags, état hors ligne | `cargo test` |
| Unitaire TypeScript / UI | Filtres, états, utilitaires, interactions, accessibilité | Vitest + Testing Library |
| Intégration base | Migrations, transactions, réconciliation | Rust + SQLite temporaire |
| Intégration médias | Décodage des fixtures, aperçus, limites de taille | Tests ciblés en CI |
| Fonctionnel desktop | Ajout de dossier, grille, inspecteur, raccourcis, dialogue natif | Automatisé si possible, checklist manuelle par OS |
| Sécurité | IPC, URI externes, assets malformés, symlinks, path traversal | Unitaire, fuzzing, non-régression |
| Performance | 10k fichiers, rendu 3D répété, mémoire, crash/redémarrage | Benchmarks reproductibles |
| UX exploratoire | Parcours réels de créateurs | Sessions guidées |

## Matrice des plateformes

| Plateforme | Minimum | Variations critiques |
|---|---|---|
| macOS | Apple Silicon | APFS sensible ou non à la casse, permissions, Retina, disques USB |
| Windows | x64 avec WebView2 | Chemins Unicode, disques externes, NTFS, UNC, installateur |
| Linux | x86_64 stable avec WebKitGTK | Wayland/X11, GPU, montages amovibles, FS sensible à la casse |

Au moins une configuration CPU/GPU modeste et une sans accélération 3D fiable sont aussi testées.

## Corpus de test

- **Fichiers normaux :** PNG avec/sans alpha, JPEG avec orientation EXIF, WebP, GIF fixe/animé ; WAV PCM, MP3 VBR/CBR, FLAC, OGG ; MP4 H.264, MP4 au codec non pris en charge, WebM VP9 ; GLB simple et instancié, glTF + `.bin` + PNG, OBJ triangulé et en quads, OBJ+MTL+textures, STL ASCII et binaire, modèles sans triangles ; fichiers inconnus et faux fichiers.
- **Fichiers pathologiques :** vides, tronqués, en-têtes invalides, bombes de décompression, buffers 3D surdimensionnés, noms identiques, chemins longs et non latins, chemins Linux non UTF-8, symlinks cycliques, disques débranchés, racines imbriquées, références glTF vers `../secret.txt`, transactions SQLite interrompues.
- **Oracles :** chaque fixture critique documente sa source, sa licence redistribuable, son SHA-256, les valeurs attendues vérifiées par un outil indépendant et la tolérance. Aucun pack d'assets propriétaire dans le dépôt public.

## Critères de release de la 0.1.0

- [ ] Chaque exigence MUST a un statut explicite `pass`, `fail` ou `blocked`, sans échec critique.
- [ ] Le catalogue se rouvre après un crash et avec un disque absent, tags intacts.
- [ ] GLB/glTF, OBJ, STL validés sur les trois OS.
- [ ] Comptages de triangles vérifiés sur les oracles ; instancing et triangulation corrects ou explicitement indisponibles.
- [ ] Fichiers sources vérifiés inchangés par hash après scans, tags, collections et aperçus.
- [ ] Aucun réseau requis, aucune donnée privée envoyée.
- [ ] Installateurs testés ; limites de codecs publiées.
- [ ] UX testée avec des créateurs ; retours bloquants corrigés ou reportés en toute transparence.
- [ ] Documentation d'installation, d'usage, de contribution et de sécurité disponible.

## Priorités des bugs

| Priorité | Exemples |
|---|---|
| **P0** | Perte ou corruption de fichier source, fuite de données, path traversal, brèche de permissions |
| **P1** | Base corrompue, fonction 3D critique cassée sur un OS supporté, crash fréquent |
| **P2** | Performance dégradée, métadonnée non critique fausse, format partiellement pris en charge |
| **P3** | Cosmétique, confort, idées post-MVP |

Toute régression P0 ou P1 bloque une release.
