# ACR CLI

[English](README.md) | 日本語 | [Français](README.fr.md)

ACR CLI は、ACR(AcrossReport)の帳票データを描画し、ファイルとして出力するコマンドラインツールです。デザイン定義とデータファイルを渡すだけで、画面を開かずに帳票を描画します。バッチ処理や、他のシステムからの呼び出しに向いています。

## 特長

- 画面を使わず、コマンドラインだけで描画
- 引数は2つ:デザイン定義(JSON)とデータファイル(JSON)
- 出力:【要確認】
- 複数ページの PNG 出力は、1つの ZIP ファイルにまとめて保存
- ZIP の中の PNG をページ送りで確認できる ZIP ビューア(単体 HTML ファイル)を同梱

## 動作環境

| OS | 状況 |
|---|---|
| Windows x64 | 対応(本リリース) |
| macOS(Apple Silicon) | 対応予定 |
| macOS(Intel) | 対応予定 |
| Linux x64 | 対応予定 |

- 対応する Windows のバージョン:Windows 11 以上

## ダウンロード

[Releases](https://github.com/acrossreport/acr-cli/releases) から、お使いの OS 用のファイルをダウンロードしてください。

- Windows x64:`【要確認】`

## 使い方

```
acr_cli <デザイン定義ファイル> <データファイル>
```

- 1番目の引数:デザイン定義ファイル(JSON)
- 2番目の引数:データファイル(JSON)
- 出力形式の指定・出力先:【要確認】

注意:

- 対象は JSON ファイルのみです。`.acr` ファイルには対応していません
- データファイルに `Parameters.TemplateFile` が書かれていても、1番目の引数で指定したデザイン定義が優先されます

## PNG 出力と ZIP ビューア

PNG で出力すると、各ページを1つの ZIP ファイル(ACR-PNG-PACKAGE 形式)にまとめて保存します。

中身を確認するには、同梱の ZIP ビューア(`【要確認】.html`)をブラウザで開き、ZIP ファイルを読み込みます。先頭 / 前 / 次 / 最終 のボタンでページ送りできます。

## 出力について

【要確認】

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
