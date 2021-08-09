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
    /// ホーム画面のビューモデル
    /// </summary>
    public class HomeContentViewModel : BindableBase
    {
        /// <summary>
        /// WebPサービス.
        /// </summary>
        private readonly IWebPService WebPService;

        /// <summary>
        /// Gets or sets the Shell
        /// MetroWindow.
        /// </summary>
        private MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// ファイルブラウザボタンのコマンド.
        /// </summary>
        public DelegateCommand BrowseCommand { get; }

        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="webPService">The webPService<see cref="IWebPService"/>.</param>
        public HomeContentViewModel(IWebPService webPService)
        {
            // フォルダ選択ボタンイベントハンドラ
            BrowseCommand = new DelegateCommand(ExecuteBrowseButtonCommand);
            WebPService = webPService;
        }

        /// <summary>
        /// ファイルブラウザボタンが押された.
        /// </summary>
        private void ExecuteBrowseButtonCommand()
        {
            Debug.WriteLine("ブラウズボタン");
            // ダイアログを定義
            FolderPickerEx picker = new();
            // ファイルダイアログを表示
            var folder = picker.PickSingleFolder();

            if (folder == null)
            {
                return;
            }

            // 変換処理
            WebPService.Convert(Directory.GetFiles(folder.Path), Shell);
        }
    }
}
