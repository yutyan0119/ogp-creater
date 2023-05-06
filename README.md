# OGP-CREATER
![](hello,%20world.png)
## icon license
icon画像はフリーではありません。**無断使用禁止**です。

icon image is not free. **DO NOT USE WITHOUT PERMISSION**.

## 概要 (Overview)
OGP画像を生成するRustアプリです。

This is a Rust app that generates OGP images.

## 画像仕様
- 1200x630
- 文字列は中心630x630の範囲に収まるようにしています
- font scaleが70より小さい場合は、真ん中で改行し、スケールを計算し直した後に、分けて描画します
- 右下にアイコンが描画されます

## image specification
- 1200x630
- The string is adjusted to fit within the range of 630x630 in the center.
- If the font scale is less than 70, it will be divided and drawn after calculating the scale again.
- An icon is drawn in the lower right corner.

## 使い方 (Usage)
```sh
$ ogp-creater <title>
```
これで、タイトルがtitleのOGP画像が生成されます。

## font license
- [Noto Sans JP](https://fonts.google.com/specimen/Noto+Sans+JP)
  - open font license