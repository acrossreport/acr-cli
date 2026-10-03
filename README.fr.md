# ACR CLI

[English](README.md) | [日本語](README.ja.md) | Français

ACR CLI est un outil en ligne de commande qui effectue le rendu des données de rapports ACR (AcrossReport) et les enregistre sous forme de fichiers. Il suffit de lui indiquer une définition et un fichier de données pour obtenir le rendu, sans ouvrir de fenêtre. Il convient au traitement par lots et à l'appel depuis d'autres systèmes.

## Fonctionnalités

- Rendu en ligne de commande, sans interface graphique
- Deux arguments : une définition (JSON) et un fichier de données (JSON)
- Sortie : 【要確認】
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

- Windows x64 : `【要確認】`

## Utilisation

```
acr_cli <fichier de définition> <fichier de données>
```

- 1er argument : fichier de définition (JSON)
- 2e argument : fichier de données (JSON)
- Choix du format et dossier de sortie : 【要確認】

Remarques :

- Seuls les fichiers JSON sont pris en charge. Les fichiers `.acr` ne le sont pas
- Même si `Parameters.TemplateFile` est indiqué dans le fichier de données, la définition indiquée en 1er argument est prioritaire

## Sortie PNG et visionneuse ZIP

En sortie PNG, les pages sont regroupées dans un seul fichier ZIP (format ACR-PNG-PACKAGE).

Pour en vérifier le contenu, ouvrez la visionneuse ZIP incluse (`【要確認】.html`) dans un navigateur et chargez le fichier ZIP. Les boutons Première / Précédente / Suivante / Dernière permettent de parcourir les pages.

## À propos de la sortie

【要確認】

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
