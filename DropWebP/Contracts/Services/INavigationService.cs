// -----------------------------------------------------------------------
// <copyright file="INavigationService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;

namespace DropWebP.Contracts.Services;

/// <summary>
/// ナビゲーションサービスのインターフェース
/// </summary>
public interface INavigationService
{
    /// <summary>
    /// ナビゲーション発生後イベント
    /// </summary>
    event NavigatedEventHandler Navigated;

    /// <summary>
    /// 戻るボタンが有効か
    /// </summary>
    bool CanGoBack
    {
        get;
    }

    /// <summary>
    /// フレーム
    /// </summary>
    Frame? Frame
    {
        get; set;
    }

    /// <summary>
    /// ナビゲーション先
    /// </summary>
    /// <param name="pageKey">ページのキー</param>
    /// <param name="parameter">パラメータ</param>
    /// <param name="clearNavigation">ナビゲーションをクリア</param>
    /// <returns>成否</returns>
    bool NavigateTo(string pageKey, object? parameter = null, bool clearNavigation = false);

    /// <summary>
    /// 戻る
    /// </summary>
    /// <returns>成否</returns>
    bool GoBack();
}
