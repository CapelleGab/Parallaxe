# ADR-0004 — Une bibliothèque vit dans un dossier

- **Statut :** acceptée
- **Date :** 2026-10-17

## Contexte

La 0.1.0 était pensée pour *référencer* autant de dossiers existants que voulu. En pratique, la plupart des créateurs préfèrent garder leurs assets à un seul endroit qu'ils organisent en sous-dossiers, comme un coffre Obsidian. Un dossier unique est aussi ce qui rendra plus tard une bibliothèque transportable sur un disque externe ou un NAS, sans aucun compte.

## Options

1. **Références uniquement :** une bibliothèque est un ensemble de dossiers éparpillés sur le disque.
2. **Dossier de bibliothèque uniquement :** chaque asset doit être copié dans un seul dossier, comme Eagle.
3. **Dossier de bibliothèque d'abord, dossiers externes autorisés.**

## Décision

**Option 3.**

- Créer une bibliothèque demande un **nom** et un **dossier de bibliothèque**, choisi ou créé via le sélecteur de dossier natif. L'utilisateur l'organise avec ses propres sous-dossiers ; tags et collections ajoutent une couche virtuelle par-dessus.
- On peut toujours ajouter des **dossiers externes** à une bibliothèque. Ils sont référencés sur place, ce qui préserve les projets de jeu, Blender ou moteur.
- Dans le catalogue, le dossier de la bibliothèque est sa **racine principale** ; les dossiers externes sont des racines ordinaires.

## Portabilité — direction pour plus tard

Quand le stockage réseau et le partage arriveront, tags et collections seront stockés dans le dossier de la bibliothèque, dans un répertoire caché `.parallaxe/`, sous forme de fichiers simples faciles à synchroniser. La base SQLite locale devient alors un index reconstructible, comme le prévoit déjà l'architecture.

Ce changement modifiera la règle de lecture seule en : **Parallaxe ne modifie jamais les assets ; il écrit uniquement dans `.parallaxe/`.** Il aura sa propre ADR avant que le moindre code n'y écrive. D'ici là, rien n'est écrit dans les dossiers de bibliothèque.

SQLite ne vit jamais sur un volume réseau : des écritures concurrentes via SMB/NFS peuvent le corrompre.

## Conséquences

- L'option 2 est rejetée : imposer des copies dupliquerait les fichiers, casserait les chemins des projets et contredirait la promesse de ne jamais déplacer les originaux.
- Importer un fichier dans le dossier de la bibliothèque (glisser-déposer, extension navigateur) devient une fonction future naturelle. Elle **copie** toujours, sur action explicite.
- La migration `0002` ajoute `roots.is_primary`, avec au plus une racine principale par bibliothèque.
