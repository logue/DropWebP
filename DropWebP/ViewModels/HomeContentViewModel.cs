// -----------------------------------------------------------------------
// <copyright file="HomeContentViewModel.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Interfaces;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Mvvm;
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows;
using Windows.Storage;
using Windows.Storage.Pickers;
using WinRT;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ホーム画面のビューモデル.
    /// </summary>
    public class HomeContentViewModel : BindableBase
    {
        /// <summary>
        /// WebPサービス..
        /// </summary>
        private readonly IWebPService webPService;

        /// <summary>
        /// Gets the BrowseCommand
        /// ファイルブラウザボタンのコマンド.
        /// </summary>
        public DelegateCommand BrowseCommand { get; }

        /// <summary>
        /// Gets or sets the Shell
        /// 現在のウィンドウ.
        /// </summary>
        private MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// Initializes a new instance of the <see cref="HomeContentViewModel"/> class.
        /// </summary>
        /// <param name="webPService">The webPService<see cref="IWebPService"/>.</param>
        public HomeContentViewModel(IWebPService webPService)
        {
            // フォルダ選択ボタンイベントハンドラ
            BrowseCommand = new DelegateCommand(ExecuteBrowseButtonCommand);
            this.webPService = webPService;
        }

        /// <summary>
        /// ファイルブラウザボタンが押された.
        /// </summary>
        public async void ExecuteBrowseButtonCommand()
        {
            Debug.WriteLine("ブラウズボタン");

            // フォルダ選択ダイアログ
            // ダイアログを定義
            FolderPicker picker = new()
            {
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
                ViewMode = PickerViewMode.List,
            };

            // ウィンドウバンドルを取得
            IntPtr hwnd = GetActiveWindow();
            IInitializeWithWindow withWindow = picker.As<IInitializeWithWindow>();
            withWindow.Initialize(hwnd);

            // ファイルダイアログを表示
            StorageFolder folder = await picker.PickSingleFolderAsync();
            if (folder == null)
            {
                return;
            }

            // 変換処理
            webPService.Convert(Directory.GetFiles(folder.Path), Shell);
        }

        /// <summary>
        /// アクティブなウィンドウのハンドルを取得.
        /// </summary>
        /// <returns>.</returns>
        [DllImport("user32.dll", ExactSpelling = true, CharSet = CharSet.Auto, PreserveSig = true, SetLastError = false)]
        private static extern IntPtr GetActiveWindow();
    }
}