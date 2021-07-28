using MahApps.Metro.Controls;
using System.Drawing;
using System.Threading.Tasks;

namespace DropWebP.Interfaces
{
    /// <summary>
    /// WebPサービスのインターフェース.
    /// </summary>
    public interface IWebPService
    {
        /// <summary>
        /// ビットマップをWebPにする.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル.</param>
        /// <returns>バイト.</returns>
        public byte[] EncodeWebP(Bitmap bitmap, long quality);

        /// <summary>
        /// WebPからビットマップに変換する.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>ビットマップ.</returns>
        public Bitmap DecodeWebP(byte[] bytes);

        /// <summary>
        /// WebPで変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        public void ConvertWebP(string path, long quality);

        /// <summary>
        /// WebPで変換する.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        public void ConvertWebP(string path, string outputPath, long quality);

        /// <summary>
        /// WebPで変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public Task ConvertWebPAsync(string path, long quality);

        /// <summary>
        /// WebPで変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public Task ConvertWebPAsync(string path, string outputPath, long quality);

        /// <summary>
        /// 変換処理.
        /// </summary>
        /// <param name="files">変換対象のファイル.</param>
        /// <param name="Shell">親画面.</param>
        public void Convert(string[] files, MetroWindow Shell);
    }
}
