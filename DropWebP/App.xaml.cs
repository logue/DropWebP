// -----------------------------------------------------------------------
// <copyright file="App.xaml.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Helpers;
using DropWebP.Interfaces;
using DropWebP.Services;
using DropWebP.ViewModels;
using DropWebP.Views;
using MahApps.Metro.Controls;
using Prism.DryIoc;
using Prism.Ioc;
using Prism.Regions;
using System;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows;

namespace DropWebP;

/// <summary>
///     Interaction logic for App.xaml.
/// </summary>
public partial class App : PrismApplication
{
    /// <summary>
    ///     ShowWindowAsync関数のパラメータに渡す定義値(画面を元の大きさに戻す)..
    /// </summary>
#pragma warning disable SA1310 // Field names should not contain underscore
    private const int SW_RESTORE = 9;
#pragma warning restore SA1310 // Field names should not contain underscore
    /// <summary>
    ///     外部プロセスのメイン・ウィンドウを起動するためのWin32 API.
    /// </summary>
    /// <param name="hwnd">The hWnd<see cref="IntPtr" />.</param>
    /// <returns>The <see cref="bool" />.</returns>
    [DllImport("user32.dll")]
    private static extern bool SetForegroundWindow(IntPtr hwnd);

    /// <summary>
    ///     The ShowWindowAsync.
    /// </summary>
    /// <param name="hWnd">The hWnd<see cref="IntPtr" />.</param>
    /// <param name="nCmdShow">The nCmdShow<see cref="int" />.</param>
    /// <returns>The <see cref="bool" />.</returns>
    [DllImport("user32.dll")]
    private static extern bool ShowWindowAsync(IntPtr hWnd, int nCmdShow);

    /// <summary>
    ///     最小化状態か.
    /// </summary>
    /// <param name="hwnd">The hWnd<see cref="IntPtr" />.</param>
    /// <returns>The <see cref="bool" />.</returns>
    [DllImport("user32.dll")]
    private static extern bool IsIconic(IntPtr hwnd);

    /// <summary>
    ///     The CreateShell.
    /// </summary>
    /// <returns>.</returns>
    protected override Window CreateShell()
    {
        // 複数インスタンスが動かないようにするための処理
        _ = new Semaphore(1, 1, Assembly.GetExecutingAssembly().GetName().Name, out bool createdNew);

        // まだアプリが起動してなければ
        if (createdNew)
        {
            return Container.Resolve<ShellWindow>();
        }

        // 既にアプリが起動していればそのアプリを前面に出す
        foreach (Process p in Process.GetProcessesByName(Process.GetCurrentProcess().ProcessName))
        {
            // 自分自身のプロセスIDは無視する
            if (p.Id != Environment.ProcessId)
            {
                // プロセスのフルパス名を比較して同じアプリケーションか検証
                if (p.MainModule.FileName == Process.GetCurrentProcess().MainModule.FileName)
                {
                    // メイン・ウィンドウが最小化されていれば元に戻す
                    if (IsIconic(p.MainWindowHandle))
                    {
                        _ = ShowWindowAsync(p.MainWindowHandle, SW_RESTORE);
                    }

                    // メイン・ウィンドウを最前面に表示する
                    _ = SetForegroundWindow(p.MainWindowHandle);
                }
            }
        }

        Current.Shutdown();
        return null;
    }

    /// <summary>
    ///     コンテナを登録.
    /// </summary>
    /// <param name="containerRegistry">インジェクションするコンテナのレジストリ.</param>
    protected override void RegisterTypes(IContainerRegistry containerRegistry)
    {
        // _= containerRegistry.RegisterInstance<Window>(Container.Resolve<ShellWindow>());
        // アプリケーションコマンド
        // _ = containerRegistry.RegisterSingleton<IApplicationCommands, ApplicationCommandsProxy>();
        // 多言語化
        _ = containerRegistry.RegisterInstance<ILocalizeService>(Container.Resolve<LocalizeService>());

        // エンコーダー
        _ = containerRegistry.RegisterSingleton<IWebPService, WebPService>();

        // ログ
        _ = containerRegistry.RegisterSingleton<ILoggerService, LoggerService>();

        // ダイアログのデザインを揃える
        containerRegistry.RegisterDialogWindow<MetroDialogService>();

        // 情報ダイアログ
        containerRegistry.RegisterDialog<AboutDialog, AboutDialogViewModel>();
    }

    /// <summary>
    ///     The ConfigureRegionAdapterMappings.
    /// </summary>
    /// <param name="regionAdapterMappings">The regionAdapterMappings<see cref="RegionAdapterMappings" />.</param>
    protected override void ConfigureRegionAdapterMappings(RegionAdapterMappings regionAdapterMappings)
    {
        // 設定フライアウト
        regionAdapterMappings.RegisterMapping(typeof(FlyoutsControl), Container.Resolve<FlyoutsControlRegionAdapter>());

        base.ConfigureRegionAdapterMappings(regionAdapterMappings);
    }

    /// <summary>
    /// モジュール登録
    /// </summary>
    /// <returns></returns>
    /*
    protected override IModuleCatalog CreateModuleCatalog()
    {
        // return new DirectoryModuleCatalog() { ModulePath = @".\Modules" };
    }
    */
}
