# Labo de test

Les tests automatiques couvrent le code ; le labo de test permet d'essayer Parallaxe à la main sur une bibliothèque réaliste.

```sh
python3 tests/lab/generate.py ~/ParallaxeLab
```

Il crée, en une dizaine de secondes et sans rien télécharger :

- un dossier de bibliothèque avec tous les formats d'image, de son, de vidéo et de 3D pris en charge, plus 3 000 images en masse ;
- des pièges : fichiers corrompus et déguisés, en-têtes d'image démesurés, noms Unicode, dossiers profonds, fichiers cachés, liens symboliques qui sortent de la bibliothèque, dossier illisible ;
- un dossier externe, un dossier qui ne doit jamais être catalogué et, sur macOS, une image disque à brancher et débrancher ;
- `expected.json`, les valeurs qu'une implémentation correcte doit afficher.

## Vérifications manuelles

| Scénario | Attendu |
|---|---|
| Créer une bibliothèque sur `Castle Kit Library` | 3 073 fichiers ; rien de `Outside Library` ; 3 liens symboliques ignorés ; le dossier verrouillé n'arrête pas le scan |
| Miniatures | Les quatre photos `reference_orientation_*` pointent vers le haut ; les images démesurées ou cassées gardent leur icône |
| Supprimer, modifier, ajouter des fichiers puis rescanner | Les fichiers supprimés passent en manquant, jamais effacés ; les images modifiées ont une nouvelle miniature |
| Ajouter `Godot Project - Dungeon` comme dossier externe | 4 fichiers de plus ; `.godot/` ignoré |
| Brancher le disque, l'ajouter, le débrancher, rescanner | Le dossier passe hors ligne ; ses fichiers restent, marqués hors ligne |
| Annuler un rescan | Rien n'est marqué manquant |

Prérequis et détails : [`tests/lab/README.md`](https://github.com/CapelleGab/Parallaxe/blob/main/tests/lab/README.md).
