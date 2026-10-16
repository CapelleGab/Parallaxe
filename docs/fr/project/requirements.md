# Exigences

## MVP 0.1.0 — obligatoire

| ID | Fonction | Exigence |
|---|---|---|
| FR-01 | Bibliothèques | Ajouter un ou plusieurs dossiers via le dialogue de l'OS ; les sources ne sont jamais modifiées. |
| FR-02 | Indexation | Scan récursif asynchrone avec progression, annulation, reprise et erreurs par fichier. |
| FR-03 | Persistance | SQLite durable pour chemins, identités, tags, collections, index et état. |
| FR-04 | Navigation | Grille virtualisée, tri, sélection, tailles de miniatures, recherche simple. |
| FR-05 | Organisation | Tags, favoris, collections virtuelles, affectation multi-fichiers. |
| FR-06 | Recherche et filtres | Nom, extension, famille de média, dossier, tags, plages de taille/dimensions, disponibilité. |
| FR-07 | Images | Miniatures et aperçus PNG/JPEG/WebP/GIF ; dimensions, alpha, taille, format. |
| FR-08 | Audio | Aperçu WAV/MP3/FLAC/OGG si décodable ; durée et propriétés vérifiables. |
| FR-09 | Vidéo | MP4/WebM : métadonnées de base, lecture et miniature si le codec le permet ; repli clair sinon. |
| FR-10 | 3D interactive | GLB/glTF, OBJ et STL avec orbite, zoom, recentrage, fond et wireframe. |
| FR-11 | Inspection 3D | Triangles, sommets, maillages, matériaux, textures, animations avec méthodes explicites. |
| FR-12 | Ressources externes | Détecter les références explicites (URI glTF, OBJ/MTL) et signaler les manquantes. |
| FR-13 | Cycle de vie | Gérer fichiers supprimés, indisponibles, modifiés ou déplacés sans perte silencieuse d'annotations. |
| FR-14 | Fiabilité OS | Builds installables macOS, Windows, Linux ; clavier, zoom, pas de login. |
| FR-15 | Confidentialité et sécurité | Aucun réseau nécessaire, aucun fichier externe exécuté, accès limité aux dossiers choisis. |
| FR-16 | Métadonnées génériques | Tout fichier : nom, chemin, taille, extension, dates, état, ouverture externe. |

## Souhaitable (non bloquant)

Recherches combinées avancées et sauvegardées · détection de doublons à la demande par empreinte forte, sans suppression automatique · miniatures vidéo indépendantes des codecs si redistribuables légalement · export JSON lisible du catalogue et sauvegarde/restauration contrôlée · aperçu complet des GIF animés et métadonnées couleur avancées.

## Après le MVP

- **Mini-studio image :** recadrer, redimensionner, rotation, luminosité/contraste, conversion, export non destructif.
- **Mini-studio audio :** découpe, fondu, gain/normalisation, conversion, export séparé.
- **Actions 3D simples :** orientation, échelle, inspection UV/normales, conversions prises en charge, export dérivé.
- **Relations d'assets :** graphe de dépendances, références cassées, associations manuelles à un projet.
- **Formats supplémentaires :** FBX, BLEND, USD, STEP, PSD/EXR/HEIF — seulement après validation des parseurs, licences et stabilité.
- **Intégrations créatives :** « ouvrir dans », export vers Blender/Godot/Unreal via des connecteurs consentis.
- **Automatisation :** CLI, API locale, puis MCP et Skill optionnels, en lecture seule d'abord.

## Hors périmètre pour l'instant

Synchronisation cloud, comptes utilisateur, collaboration multi-utilisateurs, marketplace, gestion commerciale de droits, montage vidéo complet, DAW, retopologie 3D, IA générative, OCR automatique, édition des sources par défaut, API Internet publique.

## User stories clés

**Ajouter une bibliothèque.** Je choisis un dossier existant et vois mes assets sans les déplacer. Rien n'est créé, renommé ou modifié dans la source ; une erreur de permission ne bloque pas les autres fichiers ; ajouter deux fois la même racine est détecté.

**Retrouver un asset.** Je filtre mon catalogue et retrouve un fichier en quelques actions. La recherche ignore la casse et gère les accents ; les filtres actifs restent visibles et annulables ; le tri est stable après redémarrage ; « aucun asset » et « aucun résultat » sont deux états vides distincts.

**Organiser sans toucher aux originaux.** Un asset peut être dans plusieurs collections et porter plusieurs tags. Supprimer un tag ou une collection ne supprime jamais de fichier.

**Inspecter un modèle 3D.** Orbite, zoom et recentrage fonctionnent et l'interface reprend la main après un échec de chargement. Les triangles sont calculés quand la topologie le permet ; les valeurs ne s'affichent que si elles sont pertinentes, « indisponible » sinon, jamais un zéro inventé.

**Survivre aux fichiers absents ou modifiés.** Un dossier déconnecté est « indisponible », pas « vidé ». Les annotations reviennent quand une correspondance fiable est trouvée. Un déplacement ambigu ne fusionne jamais automatiquement les fiches.

**Travailler sans fuite.** Tout marche hors ligne ; aucun contenu ni métadonnée n'est envoyé ; catalogue et cache peuvent être localisés et supprimés.

## Exigences non fonctionnelles

| ID | Exigence | Validation |
|---|---|---|
| NFR-01 | Une seule base de code UI pour 3 OS | Builds et tests natifs par cible |
| NFR-02 | Hors ligne par défaut | Exécution réseau bloqué ; inspection du trafic |
| NFR-03 | Réactivité | Interface interactive pendant scans et miniatures |
| NFR-04 | Résilience | Crash/réouverture, assets corrompus, disques débranchés, annulation |
| NFR-05 | Sécurité | IPC à privilèges minimaux, aucun script exécuté depuis un asset |
| NFR-06 | Accessibilité | Parcours clavier complets, contraste, focus, texte redimensionnable |
| NFR-07 | Compatibilité | Formats, codecs et limites documentés par plateforme |
| NFR-08 | Ouverture | Code public, doc de contribution, exports documentés |
| NFR-09 | Pas de fausses données | « Inconnu » ou « non calculable » faute de méthode fiable |

### Objectifs de performance (à confirmer par benchmark)

Corpus de référence : **10 000 assets** sur SSD local. Recherche p95 **≤ 300 ms** ; aucun blocage ressenti au-delà de **200 ms** pour les interactions usuelles sur données indexées. Temps d'indexation, mémoire et seuils 3D fixés après les benchmarks `0.0.x`.

## Mesurer le succès sans télémétrie

Tests supervisés avec au moins 8 testeurs externes de trois profils, ≥ 90 % de réussite sur les tâches critiques, 100 % des modèles 3D de référence corrects ou explicitement indisponibles, zéro original altéré, zéro accès réseau requis.
