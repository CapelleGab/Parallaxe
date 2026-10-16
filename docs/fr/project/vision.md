# Vision

> **Parallaxe** — *See beyond your files.*

## Le problème

Un créateur accumule images de référence, textures, icônes, sons, vidéos, modèles 3D et fichiers de travail dans des dossiers épars. Retrouver la bonne ressource est lent. Comprendre son format ou sa compatibilité impose souvent d'ouvrir un autre logiciel. Déplacer les fichiers peut casser leurs dépendances. Les outils existants privilégient souvent un type de média, une plateforme ou une façon d'organiser.

Parallaxe ne revendique ni l'invention de la galerie d'assets ni celle du viewer 3D. Son pari est de les assembler en un produit cohérent et extensible : **multiplateforme + 3D + métadonnées fiables + local-first**.

## Pour qui

1. **Artistes 3D et développeurs de jeux indépendants** — textures, modèles, références, packs d'assets, sons d'interface, caractéristiques techniques.
2. **Designers, illustrateurs et artistes visuels** — références, variantes, collections, tags, recherche rapide.
3. **Créateurs vidéo et audio** — effets sonores, ambiances, clips et leurs métadonnées.
4. **Équipes créatives et makers** — plus tard : échange de catalogues, workflows, intégrations moteurs. La collaboration réseau ne fait pas partie du MVP.

## La promesse

**Une seule application pour voir ce que contient un asset, ce qu'il vaut techniquement et où le retrouver — sans déplacer l'original ni rien envoyer dans le cloud.**

Après avoir installé Parallaxe sur macOS, Windows ou Linux et choisi un dossier existant, tu peux :

1. parcourir des milliers de ressources dans une galerie fluide ;
2. retrouver un fichier par nom, type, tag ou propriété ;
3. ouvrir une image, écouter un son, lire une vidéo prise en charge ou manipuler un modèle 3D ;
4. lire des métadonnées fiables et contextualisées, dont le nombre de triangles quand il est calculable ;
5. créer tags et collections sans modifier les originaux ;
6. retrouver ton catalogue après redémarrage et gérer les fichiers absents ou modifiés.

## Principes non négociables

| Principe | Conséquence |
|---|---|
| **Gratuit et open source** | Pas d'abonnement, de watermark, de verrou premium ni de télémétrie imposée. Apache-2.0. |
| **Local-first, confidentialité by design** | Ni compte ni Internet nécessaires. Les données ne sont jamais envoyées. |
| **Vraiment multiplateforme** | Une interface React/TypeScript avec Tauri 2, de vrais builds et tests par OS. |
| **Non destructif** | Ajouter une bibliothèque ne déplace, ne renomme et ne réécrit aucun asset. Les retouches futures exportent une copie. |
| **Pas d'IA imposée** | Aucune IA dans le produit. Un agent pourra plus tard consulter les données via une interface locale optionnelle. |
| **Rapide et résilient** | Indexation asynchrone annulable, caches bornés, erreurs isolées par fichier. |
| **Accessible** | Clavier, libellés, focus visible, contraste, zoom et préférences système. |

## Ce que Parallaxe ne sera pas

- Pas un clone de Photoshop, Audacity ou Blender — seulement des retouches ciblées après le MVP.
- Pas un service cloud, un DAM SaaS, un gestionnaire Git ni un moteur d'IA générative.
- Pas une promesse de décoder tous les formats propriétaires — les limites sont affichées.
- Pas un fork d'Allusion : une nouvelle base Tauri 2, inspirée des besoins et non de code recopié.
