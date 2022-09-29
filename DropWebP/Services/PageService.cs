// -----------------------------------------------------------------------
// <copyright file="PageService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using CommunityToolkit.Mvvm.ComponentModel;

using DropWebP.Contracts.Services;
using DropWebP.ViewModels;
using DropWebP.Views;

using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Services;

/// <summary>
/// ページサービス
/// </summary>
public class PageService : IPageService
{
    /// <summary>
    /// ページ一覧
    /// </summary>
    private readonly Dictionary<string, Type> pages = new ();

    /// <summary>
    /// Initializes a new instance of the <see cref="PageService"/> class.
    /// </summary>
    public PageService()
    {
        Configure<MainViewModel, MainPage>();
        Configure<SettingsViewModel, SettingsPage>();
    }

    /// <inheritdoc/>
    public Type GetPageType(string key)
    {
        Type? pageType;
        lock (pages)
        {
            if (!pages.TryGetValue(key, out pageType))
            {
                throw new ArgumentException($"Page not found: {key}. Did you forget to call PageService.Configure?");
            }
        }

        return pageType;
    }

    /// <summary>
    /// 設定
    /// </summary>
    /// <typeparam name="VM">ビューモデル</typeparam>
    /// <typeparam name="V">ビュー</typeparam>
    /// <exception cref="ArgumentException">例外</exception>
    private void Configure<VM, V>()
        where VM : ObservableObject
        where V : Page
    {
        lock (pages)
        {
            string key = typeof(VM).FullName!;
            if (pages.ContainsKey(key))
            {
                throw new ArgumentException($"The key {key} is already configured in PageService");
            }

            Type type = typeof(V);
            if (pages.Any(p => p.Value == type))
            {
                throw new ArgumentException($"This type is already configured with key {pages.First(p => p.Value == type).Key}");
            }

            pages.Add(key, type);
        }
    }
}
