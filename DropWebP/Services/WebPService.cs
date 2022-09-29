// -----------------------------------------------------------------------
// <copyright file="WebPService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using DropWebP.Contracts.Services;
using SharpEXR;
using WebP.Net;

namespace DropWebP.Services
{
    /// <summary>
    /// WebPに変換するサービス.
    /// </summary>
    public class WebPService : IWebPService
    {
        /// <summary>
        /// 設定サービス
        /// </summary>
        private readonly LocalSettingsService settingsService;

        /// <summary>
        /// 指定されたパス文字列から拡張子を削除して返します.
        /// </summary>
        /// <param name="path">ファイルのパス.</param>
        /// <returns>拡張子を抜いたファイルのパス.</returns>
        private static string GetFileName(string path)
        {
            string extension = Path.GetExtension(path);
            return string.IsNullOrEmpty(extension) ? path : path.Replace(extension, string.Empty);
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="WebPService"/> class.
        /// </summary>
        /// <param name="settingsService">設定サービス</param>
        public WebPService(LocalSettingsService settingsService)
        {
            this.settingsService = settingsService;
        }

        /// <summary>
        /// BitmapをWebPに変換.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル（-1～100）.</param>
        /// <returns>WebPに圧縮したバイト配列.</returns>
        public byte[] EncodeWebP(Bitmap bitmap, long quality = -1)
        {
            using WebPObject webP = new (bitmap);

            // TODO: BGRとABGRの判定と、それに応じた圧縮処理
            return quality < 0 ? webP.GetWebPLossless() : webP.GetWebPLossy(quality);
        }

        /// <summary>
        /// WebPからImageに変換する.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>ビットマップ.</returns>
        public Bitmap DecodeWebP(byte[] bytes)
        {
            using WebPObject webP = new (bytes);

            // WebPに変換
            return (Bitmap)webP.GetImage();
        }

        /// <summary>
        /// WebPに変換（非同期版）.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル（-1～100）.</param>
        /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
        public Task<byte[]> EncodeWebPAsync(Bitmap bitmap, long quality = -1)
        {
            return Task.Run(() => EncodeWebP(bitmap, quality));
        }

        /// <summary>
        /// WebPからビットマップに変換（非同期版）.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
        public Task<Bitmap> DecodeWebPAsync(byte[] bytes)
        {
            return Task.Run(() => DecodeWebP(bytes));
        }

        /// <summary>
        /// ファイルをWebPに変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>成否.</returns>
        public bool ConvertWebP(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            return ConvertWebP(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>成否.</returns>
        public bool ConvertWebP(string path, string outputPath, long quality = -1)
        {
            // 拡張子（小文字に統一）
            string extension = Path.GetExtension(path).ToLower();

            if (extension == ".webp" || !ImageFileExtensions().Contains(extension))
            {
                // すでにWebPのファイルと処理しない拡張子を除外
                return true;
            }

            Bitmap bitmap = LoadBitmap(path);

            // 読み込めなかったファイルは処理しない
            if (bitmap == null)
            {
                return false;
            }

            if (File.Exists(outputPath))
            {
                // ファイルが存在する場合は上書き
                File.Delete(outputPath);
            }

            // ファイルに書き出し
            try
            {
                File.WriteAllBytes(outputPath, EncodeWebP(bitmap, quality));
            }
            catch (Exception)
            {
                return false;
            }

            if (!settingsService.ReadSettingAsync<bool>("KeepOriginal").Result)
            {
                // オリジナルを保持しない場合
                File.Delete(path);
            }

            return true;
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public async Task<bool> ConvertWebPAsync(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            return await ConvertWebPAsync(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>.</returns>
        public async Task<bool> ConvertWebPAsync(string path, string outputPath, long quality)
        {
            return await ConvertWebPAsync(path, outputPath, quality, default);
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <param name="token">中断トークン.</param>
        /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
        public async Task<bool> ConvertWebPAsync(string path, string outputPath, long quality = -1, CancellationToken token = default)
        {
            // 拡張子（小文字に統一）
            string extension = Path.GetExtension(path).ToLower();

            if (extension == ".webp" || !ImageFileExtensions().Contains(extension))
            {
                // すでにWebPのファイルと処理しない拡張子を除外
                return true;
            }

            Bitmap? bitmap = LoadBitmap(path);

            // 読み込めなかったファイルは処理しない
            if (bitmap == null)
            {
                return false;
            }

            if (File.Exists(outputPath))
            {
                // ファイルが存在する場合は上書き
                File.Delete(outputPath);
            }

            // ファイルに書き出し
            await File.WriteAllBytesAsync(outputPath, await EncodeWebPAsync(bitmap, quality), token);

            if (!settingsService.ReadSettingAsync<bool>("KeepOriginal").Result)
            {
                // オリジナルを保持しない場合
                File.Delete(path);
            }

            return true;
        }

        /*
        /// <summary>
        /// 変換処理.
        /// </summary>
        /// <param name="files">変換対象のファイル.</param>
        /// <param name="shell">親画面.</param>
        public async void Convert(string[] files, MetroWindow shell)
        {
            // 全ファイル数
            int count = files.Length;

            // 失敗したファイル
            List<string> failures = new();

            // Metroダイアログのデフォルト設定
            MetroDialogSettings metroDialogSettings = new()
            {
                // 優先ボタン
                AffirmativeButtonText = ResourceExtensions.GetLocalized("DialogOk"),

                // キャンセルボタン
                NegativeButtonText = ResourceExtensions.GetLocalized("DialogCancel"),
            };

            // プログレスモーダルのコントローラー
            ProgressDialogController controller = await shell.ShowProgressAsync(
                ResourceExtensions.GetLocalized("DialogConvertingText"),
                ResourceExtensions.GetLocalized("DialogInitializingText"),
                false,
                metroDialogSettings);

            // キャンセルボタンが押されたときの設定
            controller.Canceled += async (object sender, EventArgs e) =>
            {
                await controller.CloseAsync();
                return;
            };

            // ダイアログの進捗を中間状態にする
            controller.SetIndeterminate();

            if (count == 1)
            {
                // ファイルが一つしかない場合
                controller.SetMessage(string.Format(ResourceExtensions.GetLocalized("ConvertingMessage"), Path.GetFileName(files[0])));
                bool result = ConvertWebP(
                    files[0],
                    settingsService.ReadSettingAsync<bool>("Lossless").Result ? -1 : settingsService.ReadSettingAsync<int>("Quality").Result
                );
                if (!result)
                {
                    failures.Add(files[0]);
                }
            }
            else
            {
                // プログレスダイアログの進捗情報の登録
                controller.Minimum = 0;
                controller.Maximum = count;

                CancellationTokenSource tokenSource = new();
                CancellationToken ct = tokenSource.Token;

                // 変換処理
                Task task = Task.Run(
                    async () =>
                {
                    // キャンセル可能
                    controller.SetCancelable(true);
                    ct.ThrowIfCancellationRequested();

                    // 逐次処理
                    foreach (var item in files.Select((file, index) => new { file, index }))
                    {
                        if (item.index == count || ct.IsCancellationRequested)
                        {
                            // キャンセルボタンが押されたか、最後のファイルだったとき
                            ct.ThrowIfCancellationRequested();
                            controller.SetCancelable(false);
                            controller.SetIndeterminate();
                            break;
                        }

                        // モーダルのタイトル
                        controller.SetTitle(
                            ResourceExtensions.GetLocalized("DialogConvertingText") + string.Format(" ({0}/{1})", item.index, count));

                        // モーダルの進捗
                        controller.SetProgress(item.index);

                        // モーダルのメッセージ
                        controller.SetMessage(string.Format(ResourceExtensions.GetLocalized("ConvertingMessage"), Path.GetFileName(item.file)));

                        // 変換処理
                        bool result = await ConvertWebPAsync(item.file,
                            settingsService.ReadSettingAsync<bool>("Lossless").Result ? -1 : settingsService.ReadSettingAsync<int>("Quality").Result);

                        if (!result)
                        {
                            failures.Add(item.file);
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

            string message = string.Format(ResourceExtensions.GetLocalized("CompleteMessage"), count.ToString());

            if (failures.Count != 0)
            {
                // 処理できなかったファイルがある場合
                message = string.Format(ResourceExtensions.GetLocalized("FailureMessage"), count.ToString(), failures.Count);
            }

            if (Properties.Settings.Default.NotifyComplete)
            {
                // トースト通知
                new ToastContentBuilder()
                    .AddText(localizerService.GetLocalizedString("DialogCompleteText"))
                    .AddText(message)
                    .Show();
            }
            else
            {
                // ダイアログによる通知
                _ = await shell.ShowMessageAsync(
                    localizerService.GetLocalizedString("DialogCompleteText"),
                    message);
            }
        }
        */

        /// <summary>
        /// 対応フォーマット.
        /// </summary>
        /// <returns>拡張子一覧.</returns>
        private List<string> ImageFileExtensions()
        {
            List<string> imageFileExtensions = ImageCodecInfo.GetImageEncoders()
                                      .Select(c => c.FilenameExtension)
                                      .SelectMany(e => e.Split(';'))
                                      .Select(e => e.Replace("*", string.Empty).ToLower())
                                      .ToList();

            // EXRフォーマットに独自対応
            imageFileExtensions.Add(".exr");

            // なんかエラーが起きるファイル形式
            _ = imageFileExtensions.Remove(".tif");
            _ = imageFileExtensions.Remove(".tiff");

            if (settingsService.ReadSettingAsync<bool>("IgnoreJpeg").Result)
            {
                // Jpegを無視する場合
                _ = imageFileExtensions.Remove(".jpg");
                _ = imageFileExtensions.Remove(".jpeg");
            }

            return imageFileExtensions;
        }

        /// <summary>
        /// 画像ファイルをBitmapに変換.
        /// </summary>
        /// <param name="path">画像のパス.</param>
        /// <returns>ビットマップ.</returns>
        private Bitmap? LoadBitmap(string path)
        {
            if (!File.Exists(path))
            {
                throw new FileNotFoundException(path);
            }

            // ビットマップ
            Bitmap? bmp = null;

            if (ImageFileExtensions().Contains(Path.GetExtension(path).ToLower()))
            {
                // Open file in read only mode
                using FileStream stream = new (path, FileMode.Open, FileAccess.Read);

                // Get a binary reader for the file stream
                using BinaryReader reader = new (stream);

                // copy the content of the file into a memory stream
                using MemoryStream ms = new (reader.ReadBytes((int)stream.Length));

                // make a new Bitmap object the owner of the MemoryStream
                _ = ms.Seek(0, SeekOrigin.Begin);
                bmp = new (ms);
            }
            else if (Path.GetExtension(path) == ".exr")
            {
                // HDR画像の場合（Cyberpunk2077対策）
                EXRFile exrFile = EXRFile.FromFile(path);
                EXRPart part = exrFile.Parts[0];

                // EXRファイルを開く
                part.OpenParallel(path);

                // 画像サイズを割り当てる
                bmp = new (part.DataWindow.Width, part.DataWindow.Height);
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
    }
}