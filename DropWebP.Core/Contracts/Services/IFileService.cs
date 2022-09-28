// -----------------------------------------------------------------------
// <copyright file="IFileService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Core.Contracts.Services;

/// <summary>
/// ファイルサービスのインターフェース
/// </summary>
public interface IFileService
{
    /// <summary>
    /// 読み込み
    /// </summary>
    /// <typeparam name="T">データ型</typeparam>
    /// <param name="folderPath">フォルダのパス</param>
    /// <param name="fileName">ファイル名</param>
    /// <returns>ファイルの内容</returns>
    T Read<T>(string folderPath, string fileName);

    /// <summary>
    /// 保存
    /// </summary>
    /// <typeparam name="T">データ型</typeparam>
    /// <param name="folderPath">フォルダのパス</param>
    /// <param name="fileName">ファイル名</param>
    /// <param name="content">保存するデータ内容</param>
    void Save<T>(string folderPath, string fileName, T content);

    /// <summary>
    /// ファイル削除
    /// </summary>
    /// <param name="folderPath">フォルダのパス</param>
    /// <param name="fileName">ファイル名</param>
    void Delete(string folderPath, string fileName);
}
