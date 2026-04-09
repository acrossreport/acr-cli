# ACR — Across Report Renderer CLI

> **Patent pending** (Japan, 2026)
> Copyright © 2002-2026 Across Systems Corporation
> Licensed under the [Apache License 2.0](LICENSE)

---

A cross-platform, pixel-perfect report rendering engine.
Generate PDF and PNG from JSON templates — on Windows, Linux, and macOS.

**日本語版はこちら → [README.ja.md](README.ja.md)**

---

## Features

- **Cross-platform** — Windows / Linux / macOS (including Apple Silicon)
- **Pixel-perfect output** — 1-dot accuracy guaranteed. What you design is exactly what you get (WYSIWYG)
- **JSON templates** — Human-readable, Git-friendly report definitions
- **No printer driver required** — Direct PDF and PNG output via Google Skia
- **ActiveReports conver** — Migrate existing RPX assets with [rpx2json](https://github.com/acrossreport/rpx2json)
- **Lightweight binary** — No .NET or runtime dependencies

---

## Download

Download the binary for your platform from [Releases](https://github.com/acrossreport/acr-cli/releases):

| Platform | File |
|---|---|
| Windows (x64) | `acr-windows-x86_64.zip` |
| Linux (x64) | `acr-linux-x86_64.tar.gz` |
| macOS (Apple Silicon) | `acr-macos-aarch64.tar.gz` |

---

## Quick Start

```bash
# Render a report to PDF and PNG (ZIP containing all pages)
acr_cli template.json data.json
# Check version
acr_cli --version
# Activate license
acr_cli activate
```
---
## Japanese Font Configuration

To render Japanese characters correctly, font settings must be configured
in `config.json` located in the ACR working directory.

By default, **BIZ UD Gothic** (Google) is set as the font family.

If BIZ UD Gothic is not installed on the OS, ACR will fall back to the
default OS font. For consistent output across multi-vendor environments,
install BIZ UD Gothic on each OS.

When running ACR CLI as a standalone tool, set `font_dir` to the full path
of the font directory in `config.json`, and ensure appropriate file access
permissions are configured.

```json
"render": {
    "dpi": 96,
    "font": {
        "default_family": "BIZUDGothic",
        "fallback_family": "BIZUDGothic",
        "font_dir": "/usr/share/fonts/truetype/bizud-gothic",
        "force_embed_pdf": false
    }
}
```

> **Note:** On Windows, set `font_dir` to the full path such as
> `C:\\Users\\<user>\\AppData\\Local\\Microsoft\\Windows\\Fonts`
---

## JSON Template Structure

```json
{
  "ACR": {
    "Meta": {
      "Version": "1.0",
      "Author": "Across Systems"
    },
    "Report": {
      "PageSettings": {
        "PaperSize": "A4",
        "Orientation": "Portrait"
      },
      "Sections": []
    }
  }
}
```

For full template specification, see [docs/template-spec.md](docs/template-spec.md).

---

## Migrating from ActiveReports (RPX)

Use [rpx2json](https://github.com/acrossreport/rpx2json) to convert existing RPX files:

```bash
rpx2json your-report.rpx
# → your-report.acr (ACR JSON format)
```

---

## Roadmap

| Phase | Status | Description |
|---|---|---|
| CLI (PDF / PNG) | ✅ Now | Current release |
| ACR Designer | 🔜 Next | Free GUI designer for Windows / Linux / macOS |
| acr_renderer_escpos | 🔜 Planned | Receipt printer support |
| acr_renderer_zpl | 🔜 Planned | Label printer (Zebra) |
| acr_renderer_sbpl | 🔜 Planned | Label printer (SATO) |
| acr_renderer_tpcl | 🔜 Planned | Label printer (Toshiba) |

---

## Architecture

ACR uses a device-independent intermediate drawing model:

```
JSON Template
     ↓
Template Parser
     ↓
Intermediate Render Commands  ← device-independent
     ↓
Backend Converter
     ↓
PDF / PNG / ESC/POS / ZPL / SBPL ...
```

This architecture ensures that the same template produces identical output
across all platforms and output formats.

---

## Build from Source

### Requirements

- Rust 1.75 or later
- Cargo

### Build

```bash
git clone https://github.com/acrossreport/acr-cli.git
cd acr-cli
cargo build --release
```

---

## License

Licensed under the [Apache License 2.0](LICENSE).

> Patent pending: ARC-001 (Japan, 2026)
> The intermediate drawing model and device-independent rendering architecture
> described in this software are subject to patent protection.

---

## About

**Across Systems Corporation**
Tokyo, Japan
[https://github.com/acrossreport](https://github.com/acrossreport)
