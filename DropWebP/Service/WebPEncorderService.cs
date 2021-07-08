using DropWebP.Interfaces;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using WebP.Net;

namespace DropWebP.Service
{
    /// <summary>
    /// WebPに変換するサービス
    /// </summary>
    public class WebPEncorderService : IWebPEncorderService
    {
        /// <summary>
        /// WebPで変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void EncordeWebP(string path, float quality = -1)
        {
            // 出力ファイル名
            string outputPath = GetFileName(path) + ".webp";

            EncordeWebP(path, outputPath, quality);
        }
        /// <summary>
        /// WebPで変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="outputPath">ファイルの出力先</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void EncordeWebP(string path, string outputPath, float quality = -1)
        {
            if (!File.Exists(path))
            {
                throw new FileNotFoundException();
            }
            // 画像ファイル読み込み
            Bitmap bitmap = new Bitmap(path);
            // 出力バッファ
            byte[] bytes;
            // WebPに変換
            Debug.WriteLine("Start Encorde.");
            if (quality < 0)
            {
                bytes = WebPEncoder.EncodeLossless(bitmap);
            }
            else
            {
                bytes = WebPEncoder.EncodeLossy(bitmap, 95);
            }
            Debug.WriteLine("Finish Encorde.");

            // 書き出し
            File.WriteAllBytes(outputPath, bytes);
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
    }
}
