// -----------------------------------------------------------------------
// <copyright file="IThemeSelectorService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml;

namespace DropWebP.Contracts.Services;

/// <summary>
/// テーマセレクタサービスのインターフェース
/// </summary>
public interface IThemeSelectorService
{
    /// <summary>
    /// テーマ
    /// </summary>
    ElementTheme Theme
    {
        get;
    }

    /// <summary>
    /// 初期化
    /// </summary>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task InitializeAsync();

    /// <summary>
    /// テーマ設定
    /// </summary>
    /// <param name="theme">テーマ</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task SetThemeAsync(ElementTheme theme);

    /// <summary>
    /// 要求されたテーマを設定
    /// </summary>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task SetRequestedThemeAsync();
}
