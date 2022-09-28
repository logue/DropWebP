// -----------------------------------------------------------------------
// <copyright file="IPageService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Contracts.Services;

/// <summary>
/// ページサービスのインターフェース
/// </summary>
public interface IPageService
{
    /// <summary>
    /// ページ種別
    /// </summary>
    /// <param name="key">キー</param>
    /// <returns>種別</returns>
    Type GetPageType(string key);
}
