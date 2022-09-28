// -----------------------------------------------------------------------
// <copyright file="INavigationAware.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Contracts.ViewModels;

/// <summary>
/// ナビゲーションインターフェース
/// </summary>
public interface INavigationAware
{
    /// <summary>
    /// Page が読み込まれ、親フレームの現在のソースになった
    /// </summary>
    /// <param name="parameter">ナビゲーション元</param>
    void OnNavigatedTo(object parameter);

    /// <summary>
    /// Page がアンロードされた直後に呼び出され、親フレームの現在のソースではなくなった
    /// </summary>
    void OnNavigatedFrom();
}
