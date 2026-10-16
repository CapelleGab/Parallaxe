# Roadmap

La roadmap est ordonnée par **dépendances et preuve de valeur**, pas par calendrier. Le développement est solo pour l'instant, avec du temps prévu pour apprendre Rust et tester les trois plateformes. Rien ici n'est une promesse de livraison.

Versions : `0.0.x` = prototypes internes, `0.1.0` = premier MVP public utilisable, `1.0.0` = version stable après durcissement.

| Phase | Version | Résultat visible | État |
|---|---|---|---|
| P0 | — | Recherche, matrice concurrents, corpus de test, prototype 3D en WebView | **en cours** |
| P1 | `0.0.1` | App Tauri lançable sur 3 OS, schéma de base, IPC sécurisée, CI | prochaine |
| P2 | `0.0.2` | Scanner, index persistant, états des fichiers, miniatures image | |
| P3 | `0.0.3` | Grille fluide, recherche, tags, collections, favoris | |
| P4 | `0.0.4` | Aperçus et inspecteurs image, audio, vidéo | |
| P5 | `0.0.5` | Viewer GLB/glTF, OBJ, STL, statistiques 3D, références externes | |
| P6 | `0.1.0` | Sécurité, reprise après crash, packaging, tests utilisateurs — **MVP public** | |
| P7 | `0.2.x` | Retouches image et audio légères et non destructives, export de dérivés | |
| P8 | `0.3.x` | Graphe de dépendances, projets, optimisations 3D, formats additionnels | |
| P9 | `0.4.x` | Bibliothèque Rust réutilisable, CLI avec sortie JSON stable | |
| P10 | `0.5.x` | Serveur MCP local en lecture seule et Agent Skill optionnels | |
| P11 | `1.0.0` | Migrations, compatibilité élargie, accessibilité, maintenance durable | |

P7 à P10 peuvent être réordonnées selon les retours. La `1.0.0` n'exige pas P10 si elle n'apporte pas de vraie valeur.

## Critères de passage

- **G0 — recherche :** aperçu 3D de base sur les trois OS, cas d'usage différenciant démontrable, réutilisation des composants juridiquement plausible.
- **G1 — fondations :** app lançable hors ligne, base créée, commandes autorisées, CI verte sur trois OS.
- **G2 — catalogue :** 10 000 fichiers catalogués sans altération, arrêt/reprise validés, un disque débranché ne purge rien.
- **G3 — organisation :** ajout, recherche, tags et reconnexion fonctionnent sans console ; gros catalogues fluides.
- **G4 — médias :** changer vite de média ne fait pas fuiter les lecteurs.
- **G5 — 3D :** corpus 3D validé, dont modèles instanciés, textures absentes, géométries sans triangles, gros fichiers et fichiers endommagés.
- **MVP :** toutes les exigences MUST et scénarios QA critiques passent sur les trois OS, originaux inchangés, limites documentées, ni réseau ni compte requis.

## Definition of Done

Implémentation, tests unitaires/intégration, gestion d'erreurs, doc format/OS, revue de code, vérification de non-altération, pas de blocage de navigation, builds verts. **Une démo seule n'est jamais une preuve de Done.**

## Ce qui ne déclenche pas une nouvelle phase

L'envie d'ajouter de l'IA, un système de plugins, un format 3D exotique ou un écran spectaculaire n'est pas une priorité tant que les utilisateurs ne peuvent pas importer, retrouver et inspecter leurs fichiers de manière fiable.
