// -----------------------------------------------------------------------
// <copyright file="WebPService.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Services
{
    using DropWebP.Interfaces;
    using MahApps.Metro.Controls;
    using MahApps.Metro.Controls.Dialogs;
    using SharpEXR;
    using System;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.Drawing;
    using System.Drawing.Imaging;
    using System.IO;
    using System.Linq;
    using System.Runtime.InteropServices;
    using System.Threading;
    using System.Threading.Tasks;
    using WebP.Net;

    /// <summary>
    /// WebPに変換するサービス.
    /// </summary>
    public class WebPService : IWebPService
    {
        /// <summary>
        /// 多言語化サービス..
        /// </summary>
        private readonly ILocalizerService localizerService;

        /// <summary>
        /// Initializes a new instance of the <see cref="WebPService"/> class.
        /// </summary>
        /// <param name="localizerService">.</param>
        public WebPService(ILocalizerService localizerService)
        {
            // 多言語化サービスをインジェクション
            this.localizerService = localizerService;
        }

        /// <summary>
        /// BitmapをWebPに変換.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル（-1～100）.</param>
        /// <returns>WebPに圧縮したバイト配列.</returns>
        public byte[] EncodeWebP(Bitmap bitmap, long quality = -1)
        {
            // TODO: BGRとABGRの判定と、それに応じた圧縮処理
            if (quality < 0)
            {
                return WebPEncoder.EncodeLossless(bitmap);
            }

            return WebPEncoder.EncodeLossy(bitmap, quality);
        }

        /// <summary>
        /// WebPからビットマップに変換する.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>ビットマップ.</returns>
        public Bitmap DecodeWebP(byte[] bytes)
        {
            // WebPに変換
            return WebPDecoder.Decode(bytes);
        }

        /// <summary>
        /// WebPに変換（非同期版）.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル（-1～100）.</param>
        /// <returns>WebP画像のバイト配列.</returns>
        public Task<byte[]> EncodeWebPAsync(Bitmap bitmap, long quality = -1)
        {
            return Task.Run(() => EncodeWebP(bitmap, quality));
        }

        /// <summary>
        /// WebPからビットマップに変換（非同期版）.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>ビットマップ.</returns>
        public Task<Bitmap> DecodeWebPAsync(byte[] bytes)
        {
            return Task.Run(() => DecodeWebP(bytes));
        }

        /// <summary>
        /// ファイルをWebPに変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        public void ConvertWebP(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            ConvertWebP(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        public void ConvertWebP(string path, string outputPath, long quality = -1)
        {
            Bitmap bitmap = LoadBitmap(path);

            // 読み込めなかったファイルは処理しない
            if (bitmap == null)
            {
                return;
            }

            // ファイルに書き出し
            File.WriteAllBytes(outputPath, EncodeWebP(bitmap, quality));
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public async Task ConvertWebPAsync(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            await ConvertWebPAsync(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>.</returns>
        public async Task ConvertWebPAsync(string path, string outputPath, long quality)
        {
            await ConvertWebPAsync(path, outputPath, quality, default);
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <param name="token">中断トークン.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public async Task ConvertWebPAsync(string path, string outputPath, long quality = -1, CancellationToken token = default)
        {
            Bitmap bitmap = LoadBitmap(path);

            // 読み込めなかったファイルは処理しない
            if (bitmap == null)
            {
                return;
            }

            // ファイルに書き出し
            await File.WriteAllBytesAsync(outputPath, await EncodeWebPAsync(bitmap, quality), token);
        }

        /// <summary>
        /// 指定されたパス文字列から拡張子を削除して返します.
        /// </summary>
        /// <param name="path">ファイルのパス.</param>
        /// <returns>拡張子を抜いたファイルのパス.</returns>
        private static string GetFileName(string path)
        {
            string extension = Path.GetExtension(path);
            if (string.IsNullOrEmpty(extension))
            {
                return path;
            }
            return path.Replace(extension, string.Empty);
        }

        /// <summary>
        /// 画像ファイルをBitmapに変換.
        /// </summary>
        /// <param name="path">画像のパス.</param>
        /// <returns>ビットマップ.</returns>
        private static Bitmap LoadBitmap(string path)
        {
            if (!File.Exists(path))
            {
                throw new FileNotFoundException(path);
            }

            // ビットマップ
            Bitmap bmp = null;

            if (ImageFileExtensions().Contains(Path.GetExtension(path).ToLower()))
            {
                //Open file in read only mode
                using FileStream stream = new(path, FileMode.Open, FileAccess.Read);

                //Get a binary reader for the file stream
                using BinaryReader reader = new(stream);

                //copy the content of the file into a memory stream
                using MemoryStream ms = new(reader.ReadBytes((int)stream.Length));

                //make a new Bitmap object the owner of the MemoryStream
                _ = ms.Seek(0, SeekOrigin.Begin);
                bmp = new(ms);
            }
            else if (Path.GetExtension(path) == ".exr")
            {
                // HDR画像の場合（Cyberpunk2077対策）
                EXRFile exrFile = EXRFile.FromFile(path);
                EXRPart part = exrFile.Parts[0];

                // EXRファイルを開く
                part.OpenParallel(path);

                // 画像サイズを割り当てる
                bmp = new(part.DataWindow.Width, part.DataWindow.Height);
                BitmapData data = bmp.LockBits(new Rectangle(0, 0, bmp.Width, bmp.Height), ImageLockMode.WriteOnly, PixelFormat.Format32bppArgb);
                byte[] destBytes = part.GetBytes(ImageDestFormat.BGRA8, GammaEncoding.sRGB, data.Stride);

                // Bitmapに画像をコピー
                Marshal.Copy(destBytes, 0, data.Scan0, destBytes.Length);
                bmp.UnlockBits(data);

                // EXRファイルを閉じる
                part.Close();
            }
            return bmp;
        }

        /// <summary>
        /// 変換処理.
        /// </summary>
        /// <param name="files">変換対象のファイル.</param>
        /// <param name="Shell">親画面.</param>
        public async void Convert(string[] files, MetroWindow Shell)
        {
            // 全ファイル数
            int count = files.Length;

            // Metroダイアログのデフォルト設定
            MetroDialogSettings metroDialogSettings = new()
            {
                AffirmativeButtonText = localizerService.GetLocalizedString("DialogOk"),
                NegativeButtonText = localizerService.GetLocalizedString("DialogCancel")
            };

            // プログレスコントローラー
            ProgressDialogController controller = await Shell.ShowProgressAsync(
                localizerService.GetLocalizedString("DialogConvertingText"),
                localizerService.GetLocalizedString("DialogInitializingText"), false, metroDialogSettings);

            // キャンセルボタンが押されたときの設定
            controller.Canceled += async (object sender, EventArgs e) =>
            {
                await controller.CloseAsync();
                return;
            };
            controller.SetIndeterminate();

            Debug.Write(files);

            if (count == 1)
            {
                controller.SetMessage(Path.GetFileName(files[0]));
                ConvertWebP(files[0], Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
            }
            else
            {
                // プログレスダイアログの進捗情報の登録
                controller.Minimum = 0;
                controller.Maximum = count;

                CancellationTokenSource tokenSource = new CancellationTokenSource();
                CancellationToken ct = tokenSource.Token;

                Task task = Task.Run(async () =>
                {
                    // キャンセル可能
                    controller.SetCancelable(true);
                    ct.ThrowIfCancellationRequested();
                    for (int i = 0; i <= count; i++)
                    {
                        if (i == count || ct.IsCancellationRequested)
                        {
                            ct.ThrowIfCancellationRequested();
                            controller.SetCancelable(false);
                            controller.SetIndeterminate();
                            break;
                        }
                        controller.SetTitle(
                            localizerService.GetLocalizedString("DialogConvertingText") + string.Format(" ({0}/{1})", i, count)
                        );
                        controller.SetProgress(i);
                        controller.SetMessage(Path.GetFileName(files[i]));

                        // 変換処理
                        await ConvertWebPAsync(files[i], Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);

                        if (!Properties.Settings.Default.KeepOriginal)
                        {
                            // オリジナルを保持しない場合
                            File.Delete(files[i]);
                        }
                    }
                }, tokenSource.Token);

                try
                {
                    await task;
                }
                catch (OperationCanceledException e)
                {
                    Console.WriteLine($"{nameof(OperationCanceledException)} thrown with message: {e.Message}");
                }
                finally
                {
                    tokenSource.Dispose();
                }
            }
            await controller.CloseAsync();

            await Shell.ShowMessageAsync(
                localizerService.GetLocalizedString("DialogFinishText"),
                string.Format(localizerService.GetLocalizedString("DialogFinishText"), count.ToString())
            );

            Utility.Notify.Invoke(
                localizerService.GetLocalizedString("DialogFinishText"),
                string.Format(localizerService.GetLocalizedString("DialogFinishText"), count.ToString())
            );
        }

        /// <summary>
        /// 対応フォーマット.
        /// </summary>
        /// <returns>.</returns>
        private static List<string> ImageFileExtensions()
        {
            List<string> imageFileExtensions = ImageCodecInfo.GetImageEncoders()
                                      .Select(c => c.FilenameExtension)
                                      .SelectMany(e => e.Split(';'))
                                      .Select(e => e.Replace("*", "").ToLower())
                                      .ToList();

            // EXRフォーマットに独自対応
            imageFileExtensions.Add(".exr");

            // なんかエラーが起きるファイル形式
            imageFileExtensions.Remove(".tif");

            if (Properties.Settings.Default.IgnoreJpeg)
            {
                // Jpegを無視する場合
                imageFileExtensions.Remove(".jpg");
                imageFileExtensions.Remove(".jpeg");
            }
            return imageFileExtensions;
        }
    }
}
