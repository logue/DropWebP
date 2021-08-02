using DropWebP.Interfaces;
using DropWebP.Service;
using DropWebP.Utility;
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

namespace DropWebP
{
    /// <summary>
    /// Interaction logic for App.xaml.
    /// </summary>
    public partial class App : PrismApplication
    {
        // 外部プロセスのメイン・ウィンドウを起動するためのWin32 API
        [DllImport("user32.dll")]
        private static extern bool SetForegroundWindow(IntPtr hWnd);
        [DllImport("user32.dll")]
        private static extern bool ShowWindowAsync(IntPtr hWnd, int nCmdShow);
        [DllImport("user32.dll")]
        private static extern bool IsIconic(IntPtr hWnd);
        // ShowWindowAsync関数のパラメータに渡す定義値(画面を元の大きさに戻す)
        private const int SW_RESTORE = 9;
        Semaphore semaphore = null;

        /// <summary>
        /// The CreateShell.
        /// </summary>
        /// <returns>.</returns>
        protected override Window CreateShell()
        {
            // 複数インスタンスが動かないようにするための処理
            semaphore = new Semaphore(1, 1, Assembly.GetExecutingAssembly().GetName().Name, out bool createdNew);
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
            Shutdown();
            return null;
        }

        /// <summary>
        /// コンテナを登録.
        /// </summary>
        /// <param name="containerRegistry">.</param>
        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
            // ダイアログのデザインを揃える
            containerRegistry.RegisterDialogWindow<MetroDialogService>();
            // エンコーダー
            _ = containerRegistry.RegisterSingleton<IWebPService, WebPService>();
            // アバウドダイアログ
            containerRegistry.RegisterDialog<AboutDialog, AboutDialogViewModel>();
        }

        /// <summary>
        /// The ConfigureRegionAdapterMappings.
        /// </summary>
        /// <param name="regionAdapterMappings">The regionAdapterMappings<see cref="RegionAdapterMappings"/>.</param>
        protected override void ConfigureRegionAdapterMappings(RegionAdapterMappings regionAdapterMappings)
        {
            // 設定フライアウト
            regionAdapterMappings.RegisterMapping(typeof(FlyoutsControl), Container.Resolve<FlyoutsControlRegionAdapter>());
            base.ConfigureRegionAdapterMappings(regionAdapterMappings);
        }
    }
}
