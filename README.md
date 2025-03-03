# wake-on-lan

wake-on-lan(`wol`) は、指定した MAC アドレス宛に Wake-on-LAN の Magic Packet を送信するためのシンプルな Rust アプリケーションです。

## 機能

- コマンドライン引数で指定したMACアドレスに基づいてMagic Packetを生成
- UDPブロードキャスト（ポート9）を利用してMagic Packetを送信

## インストール

Rust と Cargo がインストールされていることを前提としています。  
リポジトリをクローン後、以下のコマンドでビルドしてください

```sh
cargo build --release
```

## 使用方法

実行ファイルにMACアドレスを渡すだけで、Magic Packetが送信されます。

```sh
./target/release/wol <MAC ADDRESS> 
```
