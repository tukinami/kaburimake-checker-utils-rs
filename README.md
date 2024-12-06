# 被り負けチェッカーTSユーティリティー on Rust

[GitHub Repository](https://github.com/tukinami/kaburimake-checker-utils-rs)

[被り負けチェッカーTS](https://github.com/tukinami/kaburimake-checker-ts)用のユーティリティ・ツールです。

## 使い方

ダウンロードは[Releasesの最新版](https://github.com/tukinami/kaburimake-checker-utils-rs/releases/latest)から使用環境にあった実行ファイルをダウンロード・展開してください。

(※現在、`x86_64-pc-windows-msvc`と`i686-pc-windows-msvc`用のビルドしかありません)

実行ファイルのある場所を、仮に`path/to/kaburimake-checker-utils-rs`とします。

コマンドプロンプト、PowerShellなどのシェルで、`path/to/kaburimake-checker-utils-rs`に移動します。

その後、Windowsの場合、`kaburimake-checker-utils-rs.exe --help`入力し、決定します。(他のOSの場合は実行ファイルの名前を適宜読み替えてください)

使い方が出てくるので、それに従って使用してください。

例:

``` PowerShell
PS path\to\kaburimake-checker-utils-rs> kaburimake-checker-utils-rs.exe --version
0.1.0
```

## コマンドとオプション

例:

``` PowerShell
PS path\to\kaburimake-checker-utils-rs> kaburimake-checker-utils-rs.exe build --input C:/SSP/ghost
```

### `build`

ゴーストのフォルダがあるディレクトリ(例: `C:/SSP/ghost`)からjsonファイルを作成します。

- --output <path> : 出力するファイルを指定します。既定値: `./ghost_list.json`
- --input [dirs...] : 入力するディレクトリを指定します。複数指定可能。

### `append`

ゴーストのフォルダがあるディレクトリ(例: `C:/SSP/ghost`)からの情報をjsonファイルに追記します。

- --output <path> : 追記するファイルを指定します。既定値: `./ghost_list.json`
- --input [dirs...] : 入力するディレクトリを指定します。複数指定可能。

### `merge`

同じ形式のjsonファイルの情報をまとめ、1つのjsonファイルに出力します。

- --output <path> : 出力するファイルを指定します。既定値: `./ghost_list.json`
- --input [paths...] : 入力するファイルのパスを指定します。複数指定可能。

### `erase`

jsonファイルから指定した値を持つデータを削除します。

- --output <path> : 削除したいデータを持つjsonファイルを指定します。 既定値: `./ghost_list.json`
- --directory <directory> : 削除したいディレクトリ名を指定します。
- --sakuraname <name> : 削除したいsakuraNameを指定します。
- --keroname <name> : 削除したいkeroNameを指定します。

注意として、各々指定した値を持つ全てのデータが削除されます。

### `help`

ヘルプを表示します。

## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [clap](https://github.com/clap-rs/clap) / Kevin B. Knapp and Clap Contributors
+ [serde\_json](https://github.com/serde-rs/json) / Erick Tryzelaar, David Tolnay
+ [serde](https://github.com/serde-rs/serde) / Erick Tryzelaar,David Tolnay
+ [Chrono](https://github.com/chronotope/chrono) / Kang Seonghoon and contributors
+ [encoding\_rs](https://github.com/hsivonen/encoding_rs) / Henri Sivonen
+ (テスト実行時) [tempfile](https://github.com/Stebalien/tempfile) / Steven Allen, The Rust Project Developers, Ashley Mannix, Jason White

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
