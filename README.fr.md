# ACR CLI

[English](README.md) | [日本語](README.ja.md) | Français

ACR CLI est un outil en ligne de commande qui effectue le rendu des données de rapports ACR (AcrossReport) et les enregistre sous forme de fichiers. Il suffit de lui indiquer une définition et un fichier de données pour obtenir le rendu, sans ouvrir de fenêtre. Il convient au traitement par lots et à l'appel depuis d'autres systèmes.

## Fonctionnalités

- Rendu en ligne de commande, sans interface graphique
- Deux arguments : une définition (JSON) et un fichier de données (JSON)
- Sortie : PDF et PNG (toutes les pages réunies dans un seul ZIP), produits à chaque exécution
- La sortie PNG multipage est regroupée dans un seul fichier ZIP
- Visionneuse ZIP incluse (fichier HTML unique) pour parcourir les PNG du ZIP page par page

## Configuration requise

| OS | Statut |
|---|---|
| Windows x64 | Pris en charge (cette version) |
| macOS (Apple Silicon) | Prévu |
| macOS (Intel) | Prévu |
| Linux x64 | Prévu |

- Versions de Windows prises en charge : Windows 11 ou version ultérieure

## Téléchargement

Téléchargez le fichier correspondant à votre OS depuis les [Releases](https://github.com/acrossreport/acr-cli/releases).

- Windows x64 : `acr-cli-v0.1.0-win32-x64.zip`

## Utilisation

```
acr_cli <fichier de définition> <fichier de données>
```

- 1er argument : fichier de définition (JSON)
- 2e argument : fichier de données (JSON)
- Choix du format et dossier de sortie : aucune option nécessaire. Le PDF et le ZIP sont tous deux écrits dans le dossier `Output` du répertoire courant (créé s'il n'existe pas)

Remarques :

- Seuls les fichiers JSON sont pris en charge. Les fichiers `.acr` ne le sont pas
- Même si `Parameters.TemplateFile` est indiqué dans le fichier de données, la définition indiquée en 1er argument est prioritaire

## Sortie PNG et visionneuse ZIP

En sortie PNG, les pages sont regroupées dans un seul fichier ZIP (format ACR-PNG-PACKAGE).

Pour en vérifier le contenu, ouvrez la visionneuse ZIP incluse (`acr-zip-viewer.html`) dans un navigateur et chargez le fichier ZIP. Les boutons Première / Précédente / Suivante / Dernière permettent de parcourir les pages.

## À propos de la sortie

Chaque exécution crée deux fichiers :

- `Output/<nom du fichier de données>_<horodatage>.pdf`
- `Output/<nom du fichier de données>_<horodatage>.zip`

`<nom du fichier de données>` est le nom du second argument sans son extension, et `<horodatage>` la date et l'heure d'exécution au format `YYYYMMDDHHmm`.

Le ZIP contient `manifest.json` et un PNG par page (`pages/001.png`, `pages/002.png`, ..., 96 dpi).

Avec `--D`, le format de page et la taille du canevas (en twips) ainsi que le nombre de pages sont affichés. `--version` affiche la version.

## Liens

- ACR Designer : https://github.com/acrossreport/acr-designer
- ACR Viewer : https://github.com/acrossreport/acr-viewer
- Spécification ACR (modèle JSON) : https://github.com/acrossreport/acr-spec
- Site officiel : https://acrossreport.com

## Licence

Le code source de ce logiciel n'est pas public. Veuillez consulter le fichier [LICENSE](LICENSE) pour les conditions d'utilisation.

## Contact

across.support@gmail.com

---

© Across Systems Corporation
L'architecture d'instructions de dessin intermédiaires d'ACR fait l'objet d'une demande de brevet.
