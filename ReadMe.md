# DropWebP

[![Build status](https://ci.appveyor.com/api/projects/status/e0l46jgjh5lv2evh/branch/master?svg=true)](https://ci.appveyor.com/project/logue/dropwebp/branch/master)

このソフトウェアは、ドラッグアンドドップで画像ファイルを WebP に圧縮するアプリケーションです。

動作には.net7.0 ランタイムが必要です。下記サイトからダウンロードしてください。
<https://dotnet.microsoft.com/download/dotnet>

## 使用方法

WebP 画像に圧縮したい画像ファイルをドラッグアンドロップしてください。
同じディレクトリに保存されます。

また、クリップボードに保存されている画像をこのアプリの画面上でペーストすることにより、直接 WebP 画像を出力することができます。

## 今後の予定

- ディレクトリ内監視（ファイルが追加されると自動圧縮）
- Discord に直接投稿

## WebP とは？

WebP は、Google が開発した次世代の画像形式で、2021 年現在の現行のブラウザすべてが対応しています。
WebP の開発元である Google は、ほぼ同等の画質で PNG よりも約 26％、JPG より約 25〜34％軽くできると発表しています。

自分が試したところ、もともと 12.9M あった無圧縮の 4K 画質の PNG ファイルを無劣化 WebP に変換したところ、7.85MB まで落とすことができました。
推奨値は 95%の不可逆圧縮ですが、その場合 1.32M 程度になります。

使用できる Web サービスは、Discord、LINE、Twitter、Facebook のみです。Mixi および、Lobi は対応していません。

なお、[WebpCodecSetup.exe](https://storage.googleapis.com/downloads.webmproject.org/releases/webp/WebpCodecSetup.exe)を用いることでエクスプローラーからサムネイルを表示することができます。

### 圧縮サンプル

| 圧縮形式                                                                               | 容量  |
| -------------------------------------------------------------------------------------- | ----- |
| [オリジナル画像](https://logue.github.io/DropWebP/assets/original.png)                 | 10.7M |
| [WebP 無劣化圧縮](https://logue.github.io/DropWebP/assets/lossless.webp)               | 6.33M |
| [WebP 不可逆圧縮（推奨設定）](https://logue.github.io/DropWebP/assets/compressed.webp) | 1.15M |
| [JPEG 最高品質（参考）](https://logue.github.io/DropWebP/assets/compressed.jpg)        | 6.24M |

※[PSO2NGS](https://pso2.jp/)の 4K スクリーンショットです。

## ライセンス

Licensed under the [MIT](LICENSE) License.

Copyright © 2021-2023 [Logue](https://logue.dev/)
