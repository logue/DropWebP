using DropWebP.Interfaces;
using System.Drawing;
using System.IO;
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
            // ファイルに書き出し
            File.WriteAllBytes(outputPath, EncodeWebP(LoadBitmap(path), quality));
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
            // ファイルに書き出し
            return File.WriteAllBytesAsync(outputPath, EncodeWebP(LoadBitmap(path), quality));
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
    }
}
