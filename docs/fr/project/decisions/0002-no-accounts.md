# ADR-0002 — Aucun compte utilisateur

- **Statut :** acceptée
- **Date :** 2026-10-17

## Décision

Parallaxe n'a **aucun compte utilisateur**, ni obligatoire ni optionnel : pas d'inscription, pas de login, pas de serveur d'identité opéré par le projet.

## Raisons

Le local-first et la confidentialité by design sont des principes fondateurs. Un système de comptes ajouterait un serveur à opérer, des données personnelles à protéger et une dépendance au réseau — sans aucun bénéfice pour le MVP.

## Conséquences

Les fonctions futures comme le partage de fichiers, le stockage réseau partagé ou une extension navigateur devront fonctionner **sans compte**. Pistes à explorer, chacune dans sa propre ADR :

- **Partage :** liens « capacité » — fichiers chiffrés de bout en bout avec la clé dans le fragment de l'URL, jamais envoyée au serveur, avec expiration et limite de téléchargements ; ou transfert direct pair-à-pair. La licence déclarée accompagne l'asset.
- **Identité sans compte :** une paire de clés générée localement par appareil quand un groupe doit reconnaître ses membres.
- **Stockage réseau :** un dossier réseau est une racine comme une autre. Le catalogue SQLite reste local à chaque machine ; partager les annotations passerait par un fichier d'export, pas par un serveur.
- **Extension navigateur :** parle à l'app desktop locale (native messaging), pas à un service en ligne.
