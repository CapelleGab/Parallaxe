# Sécurité

Parallaxe lit des fichiers non fiables sur le disque des utilisateurs : assets de marketplaces, packs d'origine inconnue, fichiers endommagés. Chaque parseur de format est une surface d'attaque. Utiliser Tauri ne rend pas sûres des commandes trop puissantes.

## Règles

1. **Les originaux sont en lecture seule.** Aucun chemin de code n'écrit, ne déplace, ne renomme ni ne supprime dans une racine.
2. **La WebView manipule des IDs, pas des chemins.** Rust résout et borne chaque chemin.
3. **Accès aux ressources scopé.** Buffers et textures glTF sont servis par un protocole limité à racine + ID d'asset, jamais un chemin `file://` libre.
4. **Résolution canonique.** Les `../` qui sortent de la racine, les chemins absolus non autorisés et les symlinks vers l'extérieur sont refusés.
5. **Aucun réseau depuis un asset.** Les URL distantes d'un glTF ou de métadonnées ne sont pas résolues dans le MVP.
6. **Des budgets partout.** Nombre de fichiers, longueur de chemin, dimensions décodables, triangles, taille de buffers, temps de parsing. Une valeur démesurée donne un aperçu réduit ou indisponible, jamais un crash ou une allocation illimitée.
7. **Des données, jamais du code.** Les métadonnées stockées ne contiennent aucune instruction exécutable ; les assets n'exécutent jamais de script.
8. **Capabilities minimales.** Capabilities Tauri minimales, CSP stricte, navigation externe non sollicitée désactivée.
9. **Pas de télémétrie ni de crash reporter distant.** Les journaux ne contiennent jamais le contenu des fichiers ; les rapports partagés masquent les chemins personnels.
10. **Mises à jour optionnelles.** Toute future mise à jour en ligne est opt-in, documentée et vérifiée cryptographiquement.

## Vérifications bloquantes pour une release

| ID | Vérification | Attendu |
|---|---|---|
| QA-SEC-01 | URI glTF `../../../private.txt` | Accès refusé |
| QA-SEC-02 | Symlink dans une racine vers un dossier privé | Non suivi |
| QA-SEC-03 | Appel IPC avec un ID d'asset inconnu ou une bibliothèque non autorisée | Refusé |
| QA-SEC-04 | Fichier malveillant ou tronqué | Pas d'exécution, pas de crash irrécupérable, budgets respectés |
| QA-SEC-05 | JavaScript ou URL distante dans des métadonnées | Aucune navigation ni accès réseau silencieux |
| QA-SEC-06 | Journal de diagnostic exporté | Ni contenu d'asset ni secret, chemins personnels minimisés |
| QA-PRIV-01 | Exécution sans réseau | Tous les parcours MVP fonctionnent |
| QA-PRIV-02 | Surveillance réseau pendant import et consultation | Aucune donnée utilisateur sortante |
| QA-PRIV-03 | Supprimer un tag ou une collection | Hash de tous les originaux inchangé |
| QA-PRIV-04 | Scanner un dossier en lecture seule | Résultat correct, aucune tentative d'écriture |

Tout échec ici bloque une release publique tant qu'il n'est pas corrigé ou que la fonction vulnérable n'est pas désactivée de façon sûre.

## Signalement

Voir la [politique de sécurité](https://github.com/CapelleGab/Parallaxe/blob/main/SECURITY.md). Ne jamais signaler une vulnérabilité dans une issue publique.
