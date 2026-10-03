# ACR CLI

English | [日本語](README.ja.md) | [Français](README.fr.md)

ACR CLI is a command-line tool that renders ACR (AcrossReport) report data and outputs it as files. Pass a design definition and a data file, and it renders the report without opening any window. It is suited for batch processing and integration with other systems.

## Features

- Command-line rendering with no GUI
- Takes two arguments: a design definition (JSON) and a data file (JSON)
- Output: 【要確認】
- Multi-page PNG output is bundled into a single ZIP file
- Includes a ZIP viewer (single HTML file) to page through the PNGs in the ZIP

## System Requirements

| OS | Status |
|---|---|
| Windows x64 | Supported (this release) |
| macOS (Apple Silicon) | Planned |
| macOS (Intel) | Planned |
| Linux x64 | Planned |

- Supported Windows versions: Windows 11 or later

## Download

Download the file for your OS from [Releases](https://github.com/acrossreport/acr-cli/releases).

- Windows x64: `【要確認】`

## Usage

```
acr_cli <design definition file> <data file>
```

- 1st argument: design definition file (JSON)
- 2nd argument: data file (JSON)
- Output options and output location: 【要確認】

Notes:

- Only JSON files are supported. `.acr` files are not supported
- Even if `Parameters.TemplateFile` is written in the data file, the design definition given as the 1st argument takes priority

## PNG output and the ZIP viewer

When outputting PNG, the pages are bundled into a single ZIP file (ACR-PNG-PACKAGE format).

To check the contents, open the included ZIP viewer (`【要確認】.html`) in a browser and load the ZIP file. You can page through the PNGs with First / Previous / Next / Last.

## About Output

【要確認】

## Links

- ACR Designer: https://github.com/acrossreport/acr-designer
- ACR Viewer: https://github.com/acrossreport/acr-viewer
- ACR specification (JSON template): https://github.com/acrossreport/acr-spec
- Official website: https://acrossreport.com

## License

The source code of this software is not publicly available. Please see [LICENSE](LICENSE) for the terms of use.

## Contact

across.support@gmail.com

---

© Across Systems Corporation
The intermediate drawing instruction architecture of ACR is patent pending.
