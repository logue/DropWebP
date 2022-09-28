// -----------------------------------------------------------------------
// <copyright file="ResourceExtensions.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.Windows.ApplicationModel.Resources;

namespace DropWebP.Helpers;

/// <summary>
/// リソース機能拡張
/// </summary>
public static class ResourceExtensions
{
    /// <summary>
    /// リソースローダー
    /// </summary>
    private static readonly ResourceLoader ResourceLoader = new();

    /// <summary>
    /// 翻訳文を取得
    /// </summary>
    /// <param name="resourceKey">翻訳キー</param>
    /// <returns>翻訳文</returns>
    public static string GetLocalized(this string resourceKey) => ResourceLoader.GetString(resourceKey);
}
