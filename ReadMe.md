# DropWebP

このソフトウェアは、ドラッグアンドドップで画像ファイルをWebPに圧縮するアプリケーションです。

## 使用方法

WebP画像に圧縮したい画像ファイルをドラッグアンドロップしてください。
同じディレクトリに保存されます。

## 今後の予定

* ディレクトリ選択で一括圧縮
* ディレクトリ内監視（ファイルが追加されると自動圧縮）
* Discordに直接投稿

## WebPとは？

WebPは、Googleが開発した次世代の画像形式で、2021年現在の現行のブラウザすべてが対応しています。
WebPの開発元であるGoogleは、ほぼ同等の画質でPNGよりも約26％、JPGより約25〜34％軽くできると発表しています。

自分が試したところ、もともと12.9Mあった無圧縮の4K画質のPNGファイルを無劣化WebPに変換したところ、7.85MBまで落とすことができました。
推奨値は95%の不可逆圧縮ですが、その場合1.32M程度になります。

使用できるWebサービスは、Discord、LINEのみです。Twitter、Facebookは自動的にWebPに圧縮されます。
Mixiおよび、Lobiは対応していません。

### 圧縮サンプル

|圧縮形式|容量
|--|--
|[オリジナル画像](docs/image/original.png)|10.7M
|[WebP無劣化圧縮](docs/image/lossless.webp)|6.33M
|[WebP不可逆圧縮（推奨設定）](docs/image/compressed.webp)|1.15M
|[JPEG最高品質（参考）](docs/image/compressed.jpg)|6.24M

※[PSO2NGS](https://pso2.jp/)の4Kスクリーンショットです。

## ライセンス

Licensed under the [MIT](LICENSE) License.

Copyright © 2021 [Logue](https://logue.dev/)
