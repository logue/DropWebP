// -----------------------------------------------------------------------
// <copyright file="IWebPService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System.Drawing;
using System.Threading;
using System.Threading.Tasks;
using MahApps.Metro.Controls;

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
        /// WebPに変換（非同期版）.
        /// </summary>
        /// <param name="bitmap">ビットマップ.</param>
        /// <param name="quality">圧縮レベル（-1～100）.</param>
        /// <returns>WebP画像のバイト配列.</returns>
        public Task<byte[]> EncodeWebPAsync(Bitmap bitmap, long quality = -1);

        /// <summary>
        /// WebPからビットマップに変換（非同期版）.
        /// </summary>
        /// <param name="bytes">WebP画像のバイト配列.</param>
        /// <returns>ビットマップ.</returns>
        public Task<Bitmap> DecodeWebPAsync(byte[] bytes);

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
        /// ファイルをWebPに変換する（非同期版）.
        /// </summary>
        /// <param name="path">ファイルの入力パス.</param>
        /// <param name="outputPath">ファイルの出力先.</param>
        /// <param name="quality">品質。負数で無劣化圧縮.</param>
        /// <param name="token">中断トークン.</param>
        /// <returns>The <see cref="Task"/>.</returns>
        public Task ConvertWebPAsync(string path, string outputPath, long quality = -1, CancellationToken token = default);

        /// <summary>
        /// 変換処理.
        /// </summary>
        /// <param name="files">変換対象のファイル.</param>
        /// <param name="shell">親画面.</param>
        public void Convert(string[] files, MetroWindow shell);
    }
}