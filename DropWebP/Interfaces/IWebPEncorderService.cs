namespace DropWebP.Interfaces
{
    public interface IWebPEncorderService
    {
        /// <summary>
        /// WebPで変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void EncordeWebP(string path, float quality = -1);
        /// <summary>
        /// WebPで変換する
        /// </summary>
        /// <param name="path">ファイルの入力パス</param>
        /// <param name="outputPath">ファイルの出力先</param>
        /// <param name="quality">品質。負数で無劣化圧縮</param>
        public void EncordeWebP(string path, string outputPath, float quality = -1);
    }
}
