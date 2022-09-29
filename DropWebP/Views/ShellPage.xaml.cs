// -----------------------------------------------------------------------
// <copyright file="ShellPage.xaml.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Contracts.Services;
using DropWebP.Helpers;
using DropWebP.ViewModels;

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;

using Windows.System;

namespace DropWebP.Views;

// TODO: Update NavigationViewItem titles and icons in ShellPage.xaml.

/// <summary>
/// シェル
/// </summary>
public sealed partial class ShellPage : Page
{
    /// <summary>
    /// キー入力時
    /// </summary>
    /// <param name="key">キー</param>
    /// <param name="modifiers">モディフィアキー</param>
    /// <returns>入力されたキー</returns>
    private static KeyboardAccelerator BuildKeyboardAccelerator(VirtualKey key, VirtualKeyModifiers? modifiers = null)
    {
        KeyboardAccelerator keyboardAccelerator = new () { Key = key };

        if (modifiers.HasValue)
        {
            keyboardAccelerator.Modifiers = modifiers.Value;
        }

        keyboardAccelerator.Invoked += OnKeyboardAcceleratorInvoked;

        return keyboardAccelerator;
    }

    /// <summary>
    /// キーのイベントハンドラ
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="args">引数</param>
    private static void OnKeyboardAcceleratorInvoked(KeyboardAccelerator sender, KeyboardAcceleratorInvokedEventArgs args)
    {
        INavigationService navigationService = App.GetService<INavigationService>();

        bool result = navigationService.GoBack();

        args.Handled = result;
    }

    /// <summary>
    /// シェルのビューモデル
    /// </summary>
    public ShellViewModel ViewModel
    {
        get;
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="ShellPage"/> class.
    /// </summary>
    /// <param name="viewModel">シェルのビューモデル</param>
    public ShellPage(ShellViewModel viewModel)
    {
        ViewModel = viewModel;
        InitializeComponent();

        ViewModel.NavigationService.Frame = NavigationFrame;
        ViewModel.NavigationViewService.Initialize(NavigationViewControl);

        // TODO: Set the title bar icon by updating /Assets/WindowIcon.ico.
        // A custom title bar is required for full window theme and Mica support.
        // https://docs.microsoft.com/windows/apps/develop/title-bar?tabs=winui3#full-customization
        App.MainWindow.ExtendsContentIntoTitleBar = true;
        App.MainWindow.SetTitleBar(AppTitleBar);
        App.MainWindow.Activated += MainWindow_Activated;
        AppTitleBarText.Text = "AppDisplayName".GetLocalized();
    }

    /// <summary>
    /// シェルが読み込まれたときのイベントハンドラ
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="e">イベント</param>
    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        TitleBarHelper.UpdateTitleBar(RequestedTheme);

        KeyboardAccelerators.Add(BuildKeyboardAccelerator(VirtualKey.Left, VirtualKeyModifiers.Menu));
        KeyboardAccelerators.Add(BuildKeyboardAccelerator(VirtualKey.GoBack));
    }

    /// <summary>
    /// アプリケーションがアクティブになったときのイベントハンドラ
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="args">イベント引数</param>
    private void MainWindow_Activated(object sender, WindowActivatedEventArgs args)
    {
        string resource = args.WindowActivationState == WindowActivationState.Deactivated ? "WindowCaptionForegroundDisabled" : "WindowCaptionForeground";

        AppTitleBarText.Foreground = (SolidColorBrush)App.Current.Resources[resource];
    }

    /// <summary>
    /// リサイズ時
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="args">イベント引数</param>
    private void NavigationViewControl_DisplayModeChanged(NavigationView sender, NavigationViewDisplayModeChangedEventArgs args)
    {
        AppTitleBar.Margin = new Thickness()
        {
            Left = sender.CompactPaneLength * (sender.DisplayMode == NavigationViewDisplayMode.Minimal ? 2 : 1),
            Top = AppTitleBar.Margin.Top,
            Right = AppTitleBar.Margin.Right,
            Bottom = AppTitleBar.Margin.Bottom,
        };
    }
}
