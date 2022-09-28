// -----------------------------------------------------------------------
// <copyright file="LocalSettingsOptions.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Models;

/// <summary>
/// ローカル設定モデル
/// </summary>
public class LocalSettingsOptions
{
    /// <summary>
    /// アプリケーションのデータフォルダ
    /// </summary>
    public string? ApplicationDataFolder
    {
        get; set;
    }

    /// <summary>
    /// ローカルの設定ファイル
    /// </summary>
    public string? LocalSettingsFile
    {
        get; set;
    }
}
