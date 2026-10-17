# Concepts

## Vocabulaire

Bibliothèque
:   Un ensemble nommé d'assets indexés. Chaque bibliothèque a un **dossier de bibliothèque** où tu ranges et organises tes assets, et peut référencer des dossiers externes.

Dossier de bibliothèque
:   Le dossier choisi ou créé à la création de la bibliothèque. Organise-le avec tes propres sous-dossiers. Voir l'[ADR-0004](../project/decisions/0004-library-folder.md).

Dossier externe
:   Un autre dossier référencé sur place, par exemple un projet de jeu ou Blender que tu ne veux pas déplacer.

Racine
:   Le nom technique du dossier de bibliothèque et des dossiers externes. Parallaxe parcourt les racines récursivement et n'écrit jamais dedans.

Asset
:   Une fiche du catalogue liée à un fichier physique — ou à un fichier temporairement absent.

Tag
:   Une étiquette réutilisable, stockée uniquement dans le catalogue. Unique par bibliothèque, insensible à la casse.

Collection
:   Un regroupement virtuel d'assets. Un asset peut appartenir à autant de collections que voulu. Supprimer une collection ne supprime jamais un fichier.

Miniature
:   Un dérivé en cache, toujours régénérable.

Dépendance
:   Une référence explicite d'un fichier vers un autre, comme l'URI d'une texture glTF ou le fichier matériau d'un OBJ.

Original
:   Le fichier dans ton dossier. Aucune action de la 0.1.0 ne l'écrit, ne le déplace ni ne le supprime.

## États d'un asset

| État | Signification |
|---|---|
| `available` | Le fichier est présent et lisible. |
| `missing` | Le fichier n'est plus à son emplacement. Tags et collections sont conservés jusqu'à un nettoyage explicite. |
| `root_offline` | Toute la racine est inaccessible (par exemple un disque externe débranché). Rien n'est purgé. |
| `unsupported` | Le fichier est catalogué, mais ce format ou cette fonction n'est pas pris en charge. |
| `error` | La lecture a échoué pour ce fichier uniquement. Les autres ne sont pas affectés. |

## Fiabilité des valeurs

Chaque valeur technique porte un niveau de confiance :

| Confiance | Signification |
|---|---|
| `exact` | Lue ou calculée de façon fiable. |
| `estimated` | Dérivée par une méthode documentée, par exemple les triangles après triangulation d'un OBJ. |
| `unavailable` | Impossible à déterminer. Jamais affichée comme `0`. |
| `not_applicable` | N'existe pas pour ce format, par exemple les animations d'un STL. |

## Quand les fichiers changent

- **Même chemin, nouveau contenu :** l'asset garde son identité ; aperçus et métadonnées sont recalculés, les tags conservés.
- **Renommé ou déplacé, détecté de façon fiable :** le chemin est mis à jour, les annotations suivent.
- **Déplacement ambigu :** l'asset reste `missing` et on te demande de le réassocier. Aucune fusion silencieuse.
- **Disque débranché :** la racine passe `root_offline` et est réconciliée à son retour.
