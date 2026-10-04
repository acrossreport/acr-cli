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
| Windows x64 | Pris en charge |
| Windows ARM64 | Pris en charge |
| macOS (Apple Silicon) | Pris en charge |
| macOS (Intel) | Pris en charge |
| Linux x64 | Pris en charge |
| Linux ARM64 | Pris en charge |

- Versions de Windows prises en charge : Windows 11 ou version ultérieure
- Linux : x86_64 ou ARM64 (aarch64), glibc 2.34 ou ultérieure (par ex. Ubuntu 22.04 ou ultérieure), OpenSSL 3, fontconfig et FreeType sont nécessaires (sous Ubuntu/Debian : `sudo apt install libfontconfig1 libfreetype6`)
- macOS : Apple Silicon ou Intel, macOS 11 ou version ultérieure

## Téléchargement

Téléchargez le fichier correspondant à votre OS depuis les [Releases](https://github.com/acrossreport/acr-cli/releases).

- Windows x64 : `acr-cli-v0.1.0-win32-x64.zip`
- Windows ARM64 : `acr-cli-v0.1.0-win32-arm64.zip`
- macOS (Apple Silicon) : `acr-cli-v0.1.0-darwin-arm64.zip`
- macOS (Intel) : `acr-cli-v0.1.0-darwin-x64.zip`
- Linux x64 : `acr-cli-v0.1.0-linux-x64.zip`
- Linux ARM64 : `acr-cli-v0.1.0-linux-arm64.zip`

Sous macOS, les exécutables téléchargés avec un navigateur sont bloqués par la fonction de sécurité (Gatekeeper). Exécutez une seule fois la commande suivante dans le dossier où vous avez extrait le ZIP :

```
xattr -d com.apple.quarantine acr_cli
```

## Activation de la licence (première utilisation uniquement)

Avant la première utilisation, activez votre licence avec la commande suivante. Suivez les instructions à l'écran pour enregistrer votre adresse e-mail et le PC utilisé.

```
acr_cli activate
```

Sans activation, l'outil affiche `License not activated` et s'arrête.

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

Les pages PNG sont regroupées dans un seul fichier ZIP (format ACR-PNG-PACKAGE).

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
