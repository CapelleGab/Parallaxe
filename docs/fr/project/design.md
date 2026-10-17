# Design

Parallaxe doit évoquer un **atelier créatif haut de gamme** tout en s'effaçant devant le contenu. Images et rendus 3D apportent leurs propres couleurs ; le chrome de l'application reste sobre. Pas de néons généralisés, de faux futurisme, d'animations décoratives coûteuses ni d'effets « IA » gratuits.

## Principes d'expérience

1. **Le contenu d'abord** — montrer les ressources avant les réglages.
2. **Chemin court** — peu d'actions pour inspecter un asset, surtout au clavier.
3. **Divulgation progressive** — propriétés avancées à la demande, l'essentiel toujours visible.
4. **Interface honnête** — montrer les limites des parseurs et codecs, les estimations et les valeurs indisponibles.
5. **Sûr par défaut** — jamais de déplacement, suppression ou écrasement inattendu.
6. **Sans mode bloquant** — les tâches longues tournent en arrière-plan sans bloquer la navigation.
7. **Source unique de vérité** — tags et collections cohérents partout.
8. **Codes natifs** — même grammaire d'interface, raccourcis et dialogues propres à chaque OS.

Trois verbes structurent le produit : **Découvrir → Inspecter → Réutiliser**. « Éditer » rejoint le cycle après le MVP.

## Tokens de couleur

Les gris sont strictement neutres : les créateurs jugent les couleurs d'un asset sur eux, l'interface ne doit donc pas les teinter. Les composants n'utilisent que des tokens sémantiques, jamais de valeurs hexadécimales brutes. Les paires sont vérifiées selon WCAG 2.2 (≥ 4,5:1 pour le texte, ≥ 3:1 pour les éléments graphiques essentiels). La couleur ne remplace jamais un libellé d'état.

| Token | Sombre | Clair | Usage |
|---|---|---|---|
| `bg-canvas` | `#1B1B1B` | `#ECECEC` | Fenêtre, barre latérale |
| `bg-surface` | `#232323` | `#F7F7F7` | Zone de travail principale |
| `bg-elevated` | `#2E2E2E` | `#E2E2E2` | Panneaux, menus, survol |
| `fg-primary` | `#EDEDED` | `#1A1A1A` | Contenu principal |
| `fg-secondary` | `#9E9E9E` | `#5E5E5E` | Informations secondaires |
| `border-default` | `#3A3A3A` | `#C9C9C9` | Séparateurs |
| `accent` | `#8A9DFF` | `#3452D6` | Sélection, action principale |
| `fg-on-accent` | `#0E1330` | `#FFFFFF` | Texte sur l'accent |
| `success` | `#6FD39B` | `#116B3E` | Succès |
| `warning` | `#F2C46B` | `#8A5A00` | Avertissement réparable |
| `danger` | `#FF9A94` | `#A83232` | Risque, destruction |
| `focus-ring` | `#AEBBFF` | `#2142C9` | Focus clavier |

## Typographie et espacements

- **Polices système** (`-apple-system`, Segoe UI, sans-serif Linux) pour l'interface : rendu natif aux petites tailles.
- **Bricolage Grotesque** pour les grands titres uniquement, embarquée dans l'app (aucun réseau). Ses axes de taille optique et de largeur donnent leur caractère aux titres.
- Échelle : 12, 13, **14 (base)**, 16, 20, 24, 30 px. Interligne ≥ 1,4. Graisses 400–700.
- **Chiffres tabulaires** avec groupement localisé et unités explicites (`42 680 triangles`, `29,4 Mio`).
- Les noms de fichiers longs sont tronqués visuellement seulement ; le nom complet reste dans l'infobulle, la copie et les technologies d'assistance.
- Espacements : 4, 8, 12, 16, 24, 32, 48 px. Rayons : 4, 6, 10, 14 px. Zones cliquables ≥ 36 px, idéalement 44 px.
- Mouvement : 100–180 ms, jamais nécessaire pour comprendre une action ; `prefers-reduced-motion` respecté.
- Thèmes sombre et clair dès le départ, qui suivent l'OS par défaut.

## Composants

`AppButton`, `IconButton`, `TextInput`, `SearchField`, `FilterChip`, `TagChip`, `AssetTile`, `AssetListRow`, `MediaPreview`, `ModelViewer`, `MetadataField`, `ProgressIndicator`, `Toast`, `Dialog`, `EmptyState`, `ContextMenu`, `StatusPill`, `ErrorPanel`.

Chaque composant a un nom accessible et un rôle, des états asynchrones observables, une gestion clavier et des tests. Une valeur `not_applicable` s'affiche différemment d'un `0` valide. `ErrorPanel` n'affiche jamais de stack trace ni de chemin sensible par défaut.

## Vocabulaire

Utiliser « Ajouter un dossier », « Indexer », « Retirer de la bibliothèque », « Ouvrir le fichier », « Ouvrir le dossier », « Reconnecter la bibliothèque », « Aperçu indisponible », « Calcul non disponible ». Éviter « Télécharger votre bibliothèque », « Effacer » pour un retrait virtuel, ou « Optimiser » quand rien n'est mesuré.

## Accessibilité

Chaque action critique est accessible au clavier avec un focus visible. Les contrôles 3D ont des alternatives clavier et n'exigent jamais une souris à trois boutons. Au zoom 200 %, toutes les actions essentielles restent accessibles. Lecteurs d'écran : VoiceOver, Narrator, Orca.
