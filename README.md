ACR — Across Report Renderer CLI

Patent pending (Japan, 2026)
Copyright © 2002-2026 Across Systems Corporation
Licensed under the Apache License 2.0


A cross-platform, pixel-perfect report rendering engine.
Generate PDF and PNG from JSON templates — on Windows, Linux, and macOS.
日本語版はこちら → README.ja.md
Features

Cross-platform — Windows / Linux / macOS (including Apple Silicon)
Pixel-perfect output — 1-dot accuracy guaranteed. What you design is exactly what you get (WYSIWYG)
JSON templates — Human-readable, Git-friendly report definitions
No printer driver required — Direct PDF and PNG output via Google Skia
ActiveReports conver — Migrate existing RPX assets with rpx2json
Lightweight binary — No .NET or runtime dependencies


Download
Download the binary for your platform from Releases:
PlatformFileWindows (x64)acr-windows-x86_64.zipLinux (x64)acr-linux-x86_64.tar.gzmacOS (Apple Silicon)acr-macos-aarch64.tar.gz

Quick Start
bash# Render a report to PDF and PNG (ZIP containing all pages)
acr_cli template.json data.json
# Check version
acr_cli --version
# Activate license
acr_cli activate

JSON Template Structure
json{
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
For full template specification, see docs/template-spec.md.
Migrating from ActiveReports (RPX)
Use rpx2json to convert existing RPX files:
bashrpx2json your-report.rpx
# → your-report.acr (ACR JSON format)

Roadmap
PhaseStatusDescriptionCLI (PDF / PNG)✅ NowCurrent releaseACR Designer🔜 NextFree GUI designer for Windows / Linux / macOSacr_renderer_escpos🔜 PlannedReceipt printer supportacr_renderer_zpl🔜 PlannedLabel printer (Zebra)acr_renderer_sbpl🔜 PlannedLabel printer (SATO)acr_renderer_tpcl🔜 PlannedLabel printer (Toshiba)

Architecture
ACR uses a device-independent intermediate drawing model:
JSON Template
     ↓
Template Parser
     ↓
Intermediate Render Commands  ← device-independent
     ↓
Backend Converter
     ↓
PDF / PNG / ESC/POS / ZPL / SBPL ...
This architecture ensures that the same template produces identical output across all platforms and output formats.
Build from Source
Requirements

Rust 1.75 or later
Cargo

Build
bashgit clone https://github.com/acrossreport/acr-cli.git
cd acr-cli
cargo build --release

License
Licensed under the Apache License 2.0.

Patent pending: ARC-001 (Japan, 2026)
The intermediate drawing model and device-independent rendering architecture
described in this software are subject to patent protection.


About
Across Systems Corporation
Tokyo, Japan
https://github.com/acrossreport
