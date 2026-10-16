# Formats pris en charge

Prise en charge prévue pour la **0.1.0**. La lecture audio et vidéo dépend des codecs disponibles sur chaque système.

| Format | Catalogue | Miniature | Aperçu | Métadonnées enrichies |
|---|:-:|:-:|:-:|:-:|
| PNG, JPEG, WebP, GIF | ✓ | ✓ | ✓ | ✓ |
| WAV, MP3, FLAC, OGG | ✓ | optionnelle | selon codec | ✓ |
| MP4, WebM | ✓ | selon codec | selon codec | selon codec |
| GLB / glTF 2.0 | ✓ | ✓ | ✓ | ✓ |
| OBJ (+MTL) | ✓ | ✓ | ✓ | ✓ |
| STL (ASCII et binaire) | ✓ | ✓ | ✓ | géométrie |
| FBX, BLEND, USD, STEP | ✓ | — | — | de base |
| Tout autre fichier | ✓ | — | — | de base |

**Métadonnées de base** : nom, chemin, taille, extension, dates du système de fichiers et état.

## Métadonnées 3D

- **Triangles** comptés depuis la géométrie décodée. Pour un OBJ, la valeur est indiquée *après triangulation*. Les lignes et points ne produisent aucun triangle inventé.
- **Maillages et instances** comptés séparément, pour qu'un maillage instancié ne masque pas le nombre de triangles réellement rendus.
- **Matériaux, textures et animations** affichés seulement quand le format les définit, `not_applicable` sinon.
- **Dimensions** en unités du fichier. Parallaxe ne suppose jamais des mètres.
- **Références externes** (URI glTF, textures MTL) résolues uniquement dans la racine autorisée. Les manquantes sont signalées comme avertissements.

## Hors 0.1.0

Édition, conversion, aperçu FBX/BLEND/USD/STEP, navigation dans les archives et URL distantes. Voir la [roadmap](../project/roadmap.md).
