// -----------------------------------------------------------------------
// <copyright file="ShellWindow.xaml.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using ControlzEx.Theming;
using MahApps.Metro.Controls;
using System;
using System.Windows;
using System.Windows.Interop;
using System.Windows.Media;
using static DropWebP.Helpers.DwmApi;

namespace DropWebP.Views;

/// <summary>
///     Interaction logic for Shell.xaml.
/// </summary>
public partial class ShellWindow : MetroWindow
{
    /// <summary>
    ///     Initializes a new instance of the <see cref="ShellWindow" /> class.
    /// </summary>
    public ShellWindow()
    {
        ThemeManager.Current.ThemeSyncMode = ThemeSyncMode.SyncWithAppMode;
        ThemeManager.Current.SyncTheme();

        InitializeComponent();
    }

    /// <summary>
    ///     スタイルを適用
    /// </summary>
    /// <param name="hWnd">ウィンドウハンドル</param>
    public static void UpdateStyleAttributes(HwndSource hWnd)
    {
        int trueValue = 0x01;
        int falseValue = 0x00;

        // ダークモードの切り替え
        if (ThemeManager.Current.DetectTheme().BaseColorScheme == ThemeManager.BaseColorDark)
        {
            DwmSetWindowAttribute(
                hWnd.Handle,
                DWMWINDOWATTRIBUTE.DWMWA_USE_IMMERSIVE_DARK_MODE,
                ref trueValue,
                sizeof(uint));
        }
        else
        {
            DwmSetWindowAttribute(
                hWnd.Handle,
                DWMWINDOWATTRIBUTE.DWMWA_USE_IMMERSIVE_DARK_MODE,
                ref falseValue,
                sizeof(uint));
        }

        // ウィンドウの角を丸くする
        int rounded = (int)DWM_WINDOW_CORNER_PREFERENCE.DWMWCP_ROUND;

        DwmSetWindowAttribute(
            hWnd.Handle,
            DWMWINDOWATTRIBUTE.DWMWA_WINDOW_CORNER_PREFERENCE,
            ref rounded,
            sizeof(uint));

        // ウィンドウの背景を半透過にする
        int bg = (int)DWM_SYSTEMBACKDROP_TYPE.DWMSBT_TABBEDWINDOW;

        DwmSetWindowAttribute(
            hWnd.Handle,
            DWMWINDOWATTRIBUTE.DWMWA_SYSTEMBACKDROP_TYPE,
            ref bg,
            sizeof(uint));
    }

    private void Window_ContentRendered(object sender, EventArgs e)
    {
        // Get current hWnd
        HwndSource source = HwndSource.FromHwnd(new WindowInteropHelper(this).Handle);

        // Apply Mica brush and ImmersiveDarkMode if needed
        UpdateStyleAttributes(source);

        // Hook to Windows theme change to reapply the brushes when needed
        ThemeManager.Current.ThemeChanged += (s, ev) => UpdateStyleAttributes(source);
    }

    private void Window_Loaded(object sender, RoutedEventArgs e)
    {
        // Get PresentationSource
        PresentationSource presentationSource = PresentationSource.FromVisual((Visual)sender);

        // Subscribe to PresentationSource's ContentRendered event
        presentationSource.ContentRendered += Window_ContentRendered;
    }
}
