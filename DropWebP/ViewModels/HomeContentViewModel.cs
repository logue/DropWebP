// -----------------------------------------------------------------------
// <copyright file="HomeContentViewModel.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Interfaces;
using DropWebP.Utility;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Mvvm;
using System.Diagnostics;
using System.IO;
using System.Windows;

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
        /// Gets or sets the Shell
        /// 現在のウィンドウ.
        /// </summary>
        private MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// Gets the BrowseCommand
        /// ファイルブラウザボタンのコマンド..
        /// </summary>
        public DelegateCommand BrowseCommand { get; }

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
        public void ExecuteBrowseButtonCommand()
        {
            Debug.WriteLine("ブラウズボタン");

            // ダイアログを定義
            FolderPickerEx picker = new();

            // ファイルダイアログを表示
            Windows.Storage.StorageFolder folder = picker.PickSingleFolder();

            if (folder == null)
            {
                return;
            }

            // 変換処理
            webPService.Convert(Directory.GetFiles(folder.Path), Shell);
        }
    }
}