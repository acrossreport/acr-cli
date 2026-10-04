# ACR CLI

[English](README.md) | 日本語 | [Français](README.fr.md)

ACR CLI は、ACR(AcrossReport)の帳票データを描画し、ファイルとして出力するコマンドラインツールです。デザイン定義とデータファイルを渡すだけで、画面を開かずに帳票を描画します。バッチ処理や、他のシステムからの呼び出しに向いています。

## 特長

- 画面を使わず、コマンドラインだけで描画
- 引数は2つ:デザイン定義(JSON)とデータファイル(JSON)
- 出力:PDF と PNG(ページごとの PNG を1つの ZIP にまとめたもの)を毎回両方出力
- 複数ページの PNG 出力は、1つの ZIP ファイルにまとめて保存
- ZIP の中の PNG をページ送りで確認できる ZIP ビューア(単体 HTML ファイル)を同梱

## 動作環境

| OS | 状況 |
|---|---|
| Windows x64 | 対応 |
| Windows ARM64 | 対応 |
| macOS(Apple Silicon) | 対応 |
| macOS(Intel) | 対応 |
| Linux x64 | 対応 |
| Linux ARM64 | 対応 |

- 対応する Windows のバージョン:Windows 11 以上
- Linux:x86_64 または ARM64(aarch64)、glibc 2.34 以上(Ubuntu 22.04 以上が目安)、OpenSSL 3、fontconfig と FreeType が必要です(Ubuntu/Debian では `sudo apt install libfontconfig1 libfreetype6`)
- macOS:Apple Silicon または Intel、macOS 11 以上

## ダウンロード

[Releases](https://github.com/acrossreport/acr-cli/releases) から、お使いの OS 用のファイルをダウンロードしてください。

- Windows x64:`acr-cli-v0.1.0-win32-x64.zip`
- Windows ARM64:`acr-cli-v0.1.0-win32-arm64.zip`
- macOS(Apple Silicon):`acr-cli-v0.1.0-darwin-arm64.zip`
- macOS(Intel):`acr-cli-v0.1.0-darwin-x64.zip`
- Linux x64:`acr-cli-v0.1.0-linux-x64.zip`
- Linux ARM64:`acr-cli-v0.1.0-linux-arm64.zip`

macOS では、ブラウザでダウンロードした実行ファイルはセキュリティ機能(Gatekeeper)によって実行がブロックされます。ZIP を展開したフォルダで、次のコマンドを一度だけ実行してください。

```
xattr -d com.apple.quarantine acr_cli
```

## ライセンス認証(初回のみ)

初めて使う前に、次のコマンドでライセンス認証を行ってください。画面の案内に従って、メールアドレスと使用する PC を登録します。

```
acr_cli activate
```

認証をしていない状態で実行すると、`License not activated` と表示して終了します。

## 使い方

```
acr_cli <デザイン定義ファイル> <データファイル>
```

- 1番目の引数:デザイン定義ファイル(JSON)
- 2番目の引数:データファイル(JSON)
- 出力形式の指定・出力先:指定は不要です。PDF と ZIP を両方、実行したフォルダの `Output` フォルダに出力します(無ければ作成)

注意:

- 対象は JSON ファイルのみです。`.acr` ファイルには対応していません
- データファイルに `Parameters.TemplateFile` が書かれていても、1番目の引数で指定したデザイン定義が優先されます

## PNG 出力と ZIP ビューア

PNG は、各ページを1つの ZIP ファイル(ACR-PNG-PACKAGE 形式)にまとめて保存します。

中身を確認するには、同梱の ZIP ビューア(`acr-zip-viewer.html`)をブラウザで開き、ZIP ファイルを読み込みます。先頭 / 前 / 次 / 最終 のボタンでページ送りできます。

## 出力について

1回の実行で、次の2つのファイルを作成します。

- `Output/<データファイル名>_<日時>.pdf`
- `Output/<データファイル名>_<日時>.zip`

`<データファイル名>` は2番目の引数のファイル名(拡張子なし)、`<日時>` は実行した日時(`YYYYMMDDHHmm` 形式)です。

ZIP には `manifest.json` と、ページごとの PNG(`pages/001.png`、`pages/002.png` …、96dpi)が入っています。

`--D` を付けると、用紙サイズ・キャンバスサイズ(twips)とページ数を表示します。`--version` でバージョンを表示します。

## 関連リンク

- ACR Designer:https://github.com/acrossreport/acr-designer
- ACR Viewer:https://github.com/acrossreport/acr-viewer
- ACR 仕様(JSON テンプレート):https://github.com/acrossreport/acr-spec
- 公式サイト:https://acrossreport.com

## ライセンス

本ソフトウェアのソースコードは公開していません。利用条件は [LICENSE](LICENSE) をご確認ください。

## お問い合わせ

across.support@gmail.com

---

© Across Systems Corporation
ACR の中間描画命令アーキテクチャは特許出願中です。
