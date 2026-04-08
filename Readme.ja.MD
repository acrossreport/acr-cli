# ACR — Across Report Renderer CLI

> **特許出願中** (日本, 2026)
> Copyright © 2002-2026 Across Systems Corporation
> [Apache License 2.0](LICENSE) にて配布

---

クロスプラットフォーム対応の高精度帳票レンダリングエンジンです。
JSONテンプレートからPDF・PNGを生成します。Windows / Linux / macOS に対応しています。

**English → [README.md](README.md)**

---

## 特徴

- **クロスプラットフォーム** — Windows / Linux / macOS（Apple Silicon含む）
- **1ドット精度のWYSIWYG** — デザインした通りに出力されます
- **JSONテンプレート** — 人が読めるテキスト形式。Git管理・API連携が容易
- **プリンタドライバ不要** — Google Skia による直接PDF・PNG出力
- **ActiveReportsコンバート** — 既存RPX資産を [rpx2json](https://github.com/acrossreport/rpx2json) で移行可能
- **軽量バイナリ** — .NETやランタイム不要

---

## ダウンロード

[Releases](https://github.com/acrossreport/acr-cli/releases) からお使いのプラットフォーム向けバイナリをダウンロードしてください。

| プラットフォーム | ファイル |
|---|---|
| Windows (x64) | `acr-windows-x86_64.zip` |
| Linux (x64) | `acr-linux-x86_64.tar.gz` |
| macOS (Intel) | `acr-macos-x86_64.tar.gz` |
| macOS (Apple Silicon) | `acr-macos-aarch64.tar.gz` |

---

## クイックスタート

```bash
# PDF と PNG を同時出力
acr render --template invoice.json --data data.json

# PDF のみ出力
acr render --template invoice.json --data data.json --format pdf

# PNG のみ出力
acr render --template invoice.json --data data.json --format png
```

---

## JSONテンプレート構造

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

テンプレートの完全な仕様については [docs/template-spec.md](docs/template-spec.md) を参照してください。

---

## ActiveReports（RPX）からの移行

既存のRPXファイルは [rpx2json](https://github.com/acrossreport/rpx2json) で変換できます。

```bash
rpx2json your-report.rpx
# → your-report.acr（ACR JSON形式）
```

---

## ロードマップ

| フェーズ | 状態 | 内容 |
|---|---|---|
| CLI（PDF / PNG） | ✅ 公開中 | 現在のリリース |
| ACR Designer | 🔜 次期 | 無償GUIデザイナー（Windows / Linux / macOS） |
| acr_renderer_escpos | 🔜 予定 | レシートプリンタ対応 |
| acr_renderer_zpl | 🔜 予定 | ラベルプリンタ（Zebra）対応 |
| acr_renderer_sbpl | 🔜 予定 | ラベルプリンタ（SATO）対応 |
| acr_renderer_tpcl | 🔜 予定 | ラベルプリンタ（東芝）対応 |

---

## アーキテクチャ

ACRはデバイス非依存の中間描画命令モデルを採用しています。

```
JSONテンプレート
     ↓
テンプレート解析
     ↓
中間描画命令（デバイス非依存）  ← 特許出願中の核心部分
     ↓
バックエンド変換
     ↓
PDF / PNG / ESC/POS / ZPL / SBPL ...
```

同一テンプレートから、全プラットフォーム・全出力形式で同一の帳票が生成されます。

---

## ソースからビルド

### 必要環境

- Rust 1.75 以上
- Cargo

### ビルド手順

```bash
git clone https://github.com/acrossreport/acr-cli.git
cd acr-cli
cargo build --release
```

ビルド後のバイナリは `target/release/acr` に生成されます。

---

## ライセンス

[Apache License 2.0](LICENSE) にて配布しています。

> 特許出願中: ARC-001（日本, 2026年）
> 本ソフトウェアに記載されている中間描画命令および
> デバイス非依存レンダリングアーキテクチャは特許保護の対象です。

---

## About

**Across Systems Corporation**
東京都
[https://github.com/acrossreport](https://github.com/acrossreport)
