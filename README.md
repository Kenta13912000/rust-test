# sysctl-parser

## 概要

設定ファイルを読み込み、スキーマに沿って型チェックするRustライブラリ。

## 使用方法

1. tests/ 内の config.txt と schema.txt を編集して自分の設定に合わせる
2. `cargo test` で単体テストを実行

## テストケース

- valid_config.txt : 正しい設定
- invalid_bool.txt : bool 型エラー
- invalid_integer.txt : integer 型エラー
- missing_key.txt : 必須キー不足エラー
