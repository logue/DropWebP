// -----------------------------------------------------------------------
// <copyright file="NavigationViewService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System.Diagnostics.CodeAnalysis;
using DropWebP.Contracts.Services;
using DropWebP.Helpers;
using DropWebP.ViewModels;
using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Services;

/// <summary>
/// ナビゲーションビューのサービス
/// </summary>
public class NavigationViewService : INavigationViewService
{
    /// <summary>
    /// ナビゲーションサービス
    /// </summary>
    private readonly INavigationService navigationService;

    /// <summary>
    /// ページサービス
    /// </summary>
    private readonly IPageService pageService;

    /// <summary>
    /// ナビゲーションビュー
    /// </summary>
    private NavigationView? navigationView;

    /// <inheritdoc/>
    public IList<object>? MenuItems => navigationView?.MenuItems;

    /// <inheritdoc/>
    public object? SettingsItem => navigationView?.SettingsItem;

    /// <summary>
    /// Initializes a new instance of the <see cref="NavigationViewService"/> class.
    /// </summary>
    /// <param name="navigationService">ナビゲーションサービス</param>
    /// <param name="pageService">ページサービス</param>
    public NavigationViewService(INavigationService navigationService, IPageService pageService)
    {
        this.navigationService = navigationService;
        this.pageService = pageService;
    }

    /// <inheritdoc/>
    [MemberNotNull(nameof(navigationView))]
    public void Initialize(NavigationView navigationView)
    {
        this.navigationView = navigationView;
        this.navigationView.BackRequested += OnBackRequested;
        this.navigationView.ItemInvoked += OnItemInvoked;
    }

    /// <inheritdoc/>
    public void UnregisterEvents()
    {
        if (navigationView != null)
        {
            navigationView.BackRequested -= OnBackRequested;
            navigationView.ItemInvoked -= OnItemInvoked;
        }
    }

    /// <inheritdoc/>
    public NavigationViewItem? GetSelectedItem(Type pageType)
    {
        return navigationView != null
            ? GetSelectedItem(navigationView.MenuItems, pageType) ?? GetSelectedItem(navigationView.FooterMenuItems, pageType)
            : null;
    }

    /// <summary>
    /// [戻る] ボタンがクリックやタップなどの操作を受け取った
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="args">引数</param>
    private void OnBackRequested(NavigationView sender, NavigationViewBackRequestedEventArgs args)
    {
        _ = navigationService.GoBack();
    }

    /// <summary>
    /// メニュー内の項目がクリックやタップなどの操作を受け取った
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="args">引数</param>
    private void OnItemInvoked(NavigationView sender, NavigationViewItemInvokedEventArgs args)
    {
        if (args.IsSettingsInvoked)
        {
            _ = navigationService.NavigateTo(typeof(SettingsViewModel).FullName!);
        }
        else
        {
            NavigationViewItem? selectedItem = args.InvokedItemContainer as NavigationViewItem;

            if (selectedItem?.GetValue(NavigationHelper.NavigateToProperty) is string pageKey)
            {
                _ = navigationService.NavigateTo(pageKey);
            }
        }
    }

    /// <summary>
    /// ナビゲーションビューの項目を選択
    /// </summary>
    /// <param name="menuItems">項目</param>
    /// <param name="pageType">ページ種別</param>
    /// <returns>選択されたナビゲーションビューの項目</returns>
    private NavigationViewItem? GetSelectedItem(IEnumerable<object> menuItems, Type pageType)
    {
        foreach (NavigationViewItem item in menuItems.OfType<NavigationViewItem>())
        {
            if (IsMenuItemForPageType(item, pageType))
            {
                return item;
            }

            NavigationViewItem? selectedChild = GetSelectedItem(item.MenuItems, pageType);
            if (selectedChild != null)
            {
                return selectedChild;
            }
        }

        return null;
    }

    /// <summary>
    /// ナビゲーションビューのページか
    /// </summary>
    /// <param name="menuItem">項目</param>
    /// <param name="sourcePageType">ページ種別</param>
    /// <returns>真偽</returns>
    private bool IsMenuItemForPageType(NavigationViewItem menuItem, Type sourcePageType)
    {
        return menuItem.GetValue(NavigationHelper.NavigateToProperty) is string pageKey
&& pageService.GetPageType(pageKey) == sourcePageType;
    }
}
