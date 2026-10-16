# Confidentialité

Parallaxe est conçu pour que tu n'aies jamais à faire confiance à un serveur.

- **Aucun compte.** Pas d'inscription ni de login, ni maintenant ni plus tard.
- **Hors ligne par défaut.** Toutes les fonctions de la 0.1.0 marchent réseau coupé.
- **Rien n'est envoyé.** Ni tes fichiers ni leurs métadonnées ne quittent ton ordinateur.
- **Aucune télémétrie.** Pas d'analytics, pas de rapport de crash à distance.
- **Lecture seule sur tes dossiers.** Le catalogue et les caches vivent dans le dossier de données de l'application, hors de tes racines.
- **Tu gardes la main.** Tu peux localiser et supprimer catalogue et caches à tout moment.

Les journaux ne contiennent jamais le contenu des fichiers. Les exports de diagnostic masquent les chemins personnels sauf choix contraire.

## Intégrations futures

Une CLI locale et un serveur MCP optionnel en lecture seule sont prévus après le MVP. Le serveur MCP sera désactivé par défaut et limité aux bibliothèques que tu autorises.

!!! note
    Un serveur MCP *local* ne signifie pas un modèle *local*. Si tu connectes un client IA dans le cloud, il peut transmettre les résultats à son fournisseur.
