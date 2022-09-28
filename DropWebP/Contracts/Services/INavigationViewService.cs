// -----------------------------------------------------------------------
// <copyright file="INavigationViewService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Contracts.Services;

/// <summary>
/// ナビゲーションビューのサービスのインターフェ０素
/// </summary>
public interface INavigationViewService
{
    /// <summary>
    /// メニュー項目
    /// </summary>
    IList<object>? MenuItems
    {
        get;
    }

    /// <summary>
    /// 設定項目
    /// </summary>
    object? SettingsItem
    {
        get;
    }

    /// <summary>
    /// 初期化
    /// </summary>
    /// <param name="navigationView">対象ナビゲーションビュー</param>
    void Initialize(NavigationView navigationView);

    /// <summary>
    /// イベント解除
    /// </summary>
    void UnregisterEvents();

    /// <summary>
    /// 選択されている項目を取得
    /// </summary>
    /// <param name="pageType">ページの種別</param>
    /// <returns>ナビゲーションビューの項目</returns>
    NavigationViewItem? GetSelectedItem(Type pageType);
}
