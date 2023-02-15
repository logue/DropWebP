// -----------------------------------------------------------------------
// <copyright file="ShellWindow.xaml.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using ControlzEx.Theming;
using MahApps.Metro.Controls;
using System;
using System.Runtime.InteropServices;
using System.Windows.Interop;

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

        IntPtr hWnd = new WindowInteropHelper(GetWindow(this)).EnsureHandle();
        DWMWINDOWATTRIBUTE attribute =
            DWMWINDOWATTRIBUTE.DWMWA_WINDOW_CORNER_PREFERENCE & DWMWINDOWATTRIBUTE.DWMWA_MICA_EFFECT;
        DWM_WINDOW_CORNER_PREFERENCE preference = DWM_WINDOW_CORNER_PREFERENCE.DWMWCP_ROUND;
        DwmSetWindowAttribute(hWnd, attribute, ref preference, sizeof(uint));
    }

    /// <summary>
    ///     デスクトップ ウィンドウ マネージャーのP/in Voke
    /// </summary>
    /// <param name="hwnd">ハンドラ</param>
    /// <param name="dwAttribute">DWMWINDOWATTRIBUTE 列挙型の値として指定された、設定する値を示すフラグ。</param>
    /// <param name="pvAttribute">設定する属性値を含むオブジェクトへのポインター。</param>
    /// <param name="cbAttribute">pvAttribute パラメーターを使用して設定される属性値のサイズ</param>
    /// <returns>HRESULT値</returns>
    [DllImport("dwmapi.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern long DwmSetWindowAttribute(
        IntPtr hwnd,
        DWMWINDOWATTRIBUTE dwAttribute,
        ref DWM_WINDOW_CORNER_PREFERENCE pvAttribute,
        uint cbAttribute);

#pragma warning disable SA1602 // Enumeration items should be documented
    /// <summary>
    ///     DWMWINDOWATTRIBUTE 列挙型
    /// </summary>
    [Flags]
    public enum DWMWINDOWATTRIBUTE
    {
        DWMWA_USE_IMMERSIVE_DARK_MODE = 20,
        DWMWA_WINDOW_CORNER_PREFERENCE = 33,
        DWMWA_MICA_EFFECT = 1029
    }

    /// <summary>
    ///     DWM_WINDOW_CORNER_PREFERENCE 列挙型
    /// </summary>
    public enum DWM_WINDOW_CORNER_PREFERENCE
    {
        DWMWCP_DEFAULT = 0,
        DWMWCP_DONOTROUND = 1,
        DWMWCP_ROUND = 2,
        DWMWCP_ROUNDSMALL = 3
    }
#pragma warning restore SA1602 // Enumeration items should be documented
}
