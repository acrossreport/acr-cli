# ACR CLI

English | [日本語](README.ja.md) | [Français](README.fr.md)

ACR CLI is a command-line tool that renders ACR (AcrossReport) report data and outputs it as files. Pass a design definition and a data file, and it renders the report without opening any window. It is suited for batch processing and integration with other systems.

## Features

- Command-line rendering with no GUI
- Takes two arguments: a design definition (JSON) and a data file (JSON)
- Output: both PDF and PNG (all pages bundled into one ZIP) on every run
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

- Windows x64: `acr-cli-v0.1.0-win32-x64.zip`

## Usage

```
acr_cli <design definition file> <data file>
```

- 1st argument: design definition file (JSON)
- 2nd argument: data file (JSON)
- Output options and output location: no options needed. Both the PDF and the ZIP are written to the `Output` folder under the current directory (created if missing)

Notes:

- Only JSON files are supported. `.acr` files are not supported
- Even if `Parameters.TemplateFile` is written in the data file, the design definition given as the 1st argument takes priority

## PNG output and the ZIP viewer

When outputting PNG, the pages are bundled into a single ZIP file (ACR-PNG-PACKAGE format).

To check the contents, open the included ZIP viewer (`acr-zip-viewer.html`) in a browser and load the ZIP file. You can page through the PNGs with First / Previous / Next / Last.

## About Output

Each run creates two files:

- `Output/<data file name>_<timestamp>.pdf`
- `Output/<data file name>_<timestamp>.zip`

`<data file name>` is the name of the second argument without its extension, and `<timestamp>` is the run time in `YYYYMMDDHHmm` format.

The ZIP contains `manifest.json` and one PNG per page (`pages/001.png`, `pages/002.png`, ..., 96 dpi).

With `--D`, the page size and canvas size (in twips) and the page count are printed. `--version` prints the version.

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
