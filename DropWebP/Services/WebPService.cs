using DropWebP.Interfaces;
using MahApps.Metro.Controls;
using MahApps.Metro.Controls.Dialogs;
using SharpEXR;
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using WebP.Net;

namespace DropWebP.Service
{
    /// <summary>
    /// WebPに変換するサービス
    /// </summary>
    public class WebPService : IWebPService
    {
        /// <summary>
        /// BitmapをWebPに変換
        /// </summary>
        /// <param name="bitmap">ビットマップ</param>
        /// <param name="quality">圧縮レベル（-1～100）</param>
        /// <returns>WebPに圧縮したバイト配列</returns>
        public byte[] EncodeWebP(Bitmap bitmap, long quality = -1)
        {
            if (quality < 0)
            {
                return WebPEncoder.EncodeLossless(bitmap);
            }

            return WebPEncoder.EncodeLossy(bitmap, quality);
        }

        /// <summary>
        /// WebPからビットマップに変換する
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列</param>
        /// <returns>ビットマップ</returns>
        public Bitmap DecodeWebP(byte[] bytes)
        {
            // WebPに変換
            return WebPDecoder.Decode(bytes);
        }

        /// <summary>
        /// ファイルをWebPに変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void ConvertWebP(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            ConvertWebP(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="outputPath">ファイルの出力先</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void ConvertWebP(string path, string outputPath, long quality = -1)
        {
            Bitmap bitmap = LoadBitmap(path);
            // ファイルに書き出し
            File.WriteAllBytes(outputPath, EncodeWebP(bitmap, quality));
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public Task ConvertWebPAsync(string path, long quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            return ConvertWebPAsync(path, outputPath, quality);
        }

        /// <summary>
        /// ファイルをWebPに変換する（非同期版）
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="outputPath">ファイルの出力先</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public Task ConvertWebPAsync(string path, string outputPath, long quality = -1)
        {
            Bitmap bitmap = LoadBitmap(path);
            // ファイルに書き出し
            return File.WriteAllBytesAsync(outputPath, EncodeWebP(bitmap, quality));
        }

        /// <summary>
        /// 指定されたパス文字列から拡張子を削除して返します
        /// </summary>
        /// <param name="path">ファイルのパス</param>
        /// <returns>拡張子を抜いたファイルのパス</returns>
        private static string GetFileName(string path)
        {
            var extension = Path.GetExtension(path);
            if (string.IsNullOrEmpty(extension))
            {
                return path;
            }
            return path.Replace(extension, string.Empty);
        }

        /// <summary>
        /// Load Bitmap
        /// </summary>
        /// <see cref="https://stackoverflow.com/questions/2808753/right-way-to-dispose-image-bitmap-and-picturebox"/>
        /// <param name="path"></param>
        /// <returns></returns>
        private static Bitmap LoadBitmap(string path)
        {
            if (!File.Exists(path))
            {
                throw new FileNotFoundException();
            }

            if (Path.GetExtension(path) == ".exr")
            {
                // HDR画像の場合（Cyberpunk2077対策）
                EXRFile exrFile = EXRFile.FromFile(path);
                EXRPart part = exrFile.Parts[0];

                // EXRファイルを開く
                part.OpenParallel(path);
                // 画像サイズを割り当てる
                Bitmap bmp = new Bitmap(part.DataWindow.Width, part.DataWindow.Height);
                BitmapData data = bmp.LockBits(new Rectangle(0, 0, bmp.Width, bmp.Height), ImageLockMode.WriteOnly, PixelFormat.Format32bppArgb);
                byte[] destBytes = part.GetBytes(ImageDestFormat.BGRA8, GammaEncoding.sRGB, data.Stride);
                // Bitmapに画像をコピー
                Marshal.Copy(destBytes, 0, data.Scan0, destBytes.Length);
                bmp.UnlockBits(data);
                // EXRファイルを閉じる
                part.Close();

                return bmp;
            }

            //Open file in read only mode
            using (FileStream stream = new FileStream(path, FileMode.Open, FileAccess.Read))
            //Get a binary reader for the file stream
            using (BinaryReader reader = new BinaryReader(stream))
            {
                //copy the content of the file into a memory stream
                var memoryStream = new MemoryStream(reader.ReadBytes((int)stream.Length));
                //make a new Bitmap object the owner of the MemoryStream
                return new Bitmap(memoryStream);
            }
        }

        /// <summary>
        /// 変換処理
        /// </summary>
        /// <param name="files">変換対象のファイル</param>
        /// <param name="Shell">親画面</param>
        public async void Convert(string[] files, MetroWindow Shell)
        {
            // 全ファイル数
            int count = files.Length;

            // Metroダイアログのデフォルト設定
            MetroDialogSettings metroDialogSettings = new MetroDialogSettings()
            {
                AffirmativeButtonText = "OK",
                NegativeButtonText = "Cancel"
            };

            // プログレスコントローラー
            ProgressDialogController controller = await Shell.ShowProgressAsync("Now converting", "Initializing....", false, metroDialogSettings);
            controller.SetIndeterminate();
            // キャンセルボタンが押されたときの設定
            controller.Canceled += async (object sender, EventArgs e) =>
            {
                controller.Minimum = 0;
                controller.Maximum = 0;
                await controller.CloseAsync();
                return;
            };
            // プログレスダイアログの進捗情報の登録
            controller.Minimum = 0;
            controller.Maximum = count;

            await Task.Run(() =>
            {
                // キャンセル可能
                controller.SetCancelable(true);
                for (int i = 0; i <= count; i++)
                {
                    if (i == count)
                    {
                        controller.SetCancelable(false);
                        break;
                    }
                    controller.SetProgress(i);
                    controller.SetMessage(files[i]);

                    // 変換処理
                    ConvertWebPAsync(files[i], Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
                }
            });
            await controller.CloseAsync();

            await Shell.ShowMessageAsync("Finish.", count.ToString() + " files are converted.");
        }
    }
}
