// -----------------------------------------------------------------------
// <copyright file="ShellViewModel.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using CommunityToolkit.Mvvm.ComponentModel;

using DropWebP.Contracts.Services;
using DropWebP.Views;

using Microsoft.UI.Xaml.Navigation;

namespace DropWebP.ViewModels;

/// <summary>
/// アプリケーション外枠のビューモデル
/// </summary>
public class ShellViewModel : ObservableRecipient
{
    private bool isBackEnabled;
    private object? selected;

    /// <summary>
    /// ナビゲーションサービス
    /// </summary>
    public INavigationService NavigationService
    {
        get;
    }

    /// <summary>
    /// ナビゲーションの表示内容サービス
    /// </summary>
    public INavigationViewService NavigationViewService
    {
        get;
    }

    /// <summary>
    /// 背面が有効か
    /// </summary>
    public bool IsBackEnabled
    {
        get => isBackEnabled;
        set => SetProperty(ref isBackEnabled, value);
    }

    /// <summary>
    /// 選択
    /// </summary>
    public object? Selected
    {
        get => selected;
        set => SetProperty(ref selected, value);
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="ShellViewModel"/> class.
    /// </summary>
    /// <param name="navigationService">ナビゲーションサービス</param>
    /// <param name="navigationViewService">ナビゲーションで表示される内容のサービス</param>
    public ShellViewModel(INavigationService navigationService, INavigationViewService navigationViewService)
    {
        NavigationService = navigationService;
        NavigationService.Navigated += OnNavigated;
        NavigationViewService = navigationViewService;
    }

    /// <summary>
    /// ナビゲーションによる画面遷移時のイベントハンドラ
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="e">イベント</param>
    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        IsBackEnabled = NavigationService.CanGoBack;

        if (e.SourcePageType == typeof(SettingsPage))
        {
            Selected = NavigationViewService.SettingsItem;
            return;
        }

        var selectedItem = NavigationViewService.GetSelectedItem(e.SourcePageType);
        if (selectedItem != null)
        {
            Selected = selectedItem;
        }
    }
}
