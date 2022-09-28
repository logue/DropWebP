// -----------------------------------------------------------------------
// <copyright file="NavigationHelper.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Helpers;

// Helper class to set the navigation target for a NavigationViewItem.
//
// Usage in XAML:
// <NavigationViewItem x:Uid="Shell_Main" Icon="Document" helpers:NavigationHelper.NavigateTo="AppName.ViewModels.MainViewModel" />
//
// Usage in code:
// NavigationHelper.SetNavigateTo(navigationViewItem, typeof(MainViewModel).FullName);

/// <summary>
/// ナビゲーションヘルパ
/// </summary>
public class NavigationHelper
{
    /// <summary>
    /// ナビゲーション先を取得
    /// </summary>
    /// <param name="item">ナビゲーション項目</param>
    /// <returns>項目の値</returns>
    public static string GetNavigateTo(NavigationViewItem item) => (string)item.GetValue(NavigateToProperty);

    /// <summary>
    /// ナビゲーション元を取得
    /// </summary>
    /// <param name="item">ナビゲーション項目</param>
    /// <param name="value">値</param>
    public static void SetNavigateTo(NavigationViewItem item, string value) => item.SetValue(NavigateToProperty, value);

    /// <summary>
    /// ナビゲーション元のプロパティ
    /// </summary>
    public static readonly DependencyProperty NavigateToProperty =
        DependencyProperty.RegisterAttached("NavigateTo", typeof(string), typeof(NavigationHelper), new PropertyMetadata(null));
}
