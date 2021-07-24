using DropWebP.Interfaces;
using MahApps.Metro.Controls;
using MahApps.Metro.Controls.Dialogs;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using System;
using System.IO;
using System.Windows;
using System.Windows.Interop;
using Windows.Storage;
using Windows.Storage.Pickers;
using WinRT;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ドラッグアンドロップする領域のViewModel
    /// https://github.com/nabehiro22/Image_DragAndDrop のコードを流用
    /// </summary>
    public class HomeContentViewModel : BindableBase
    {
        private IEventAggregator eventAggregator;
        private IRegionManager regionManager;

        /// <summary>
        /// MetroWindow
        /// </summary>
        public MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// Webのエンコーダー
        /// </summary>
        private IWebPService webPService;

        /// <summary>
        /// ファイルブラウザボタンのコマンド
        /// </summary>
        public DelegateCommand BrowseButtonCommand { get; }

        /// <summary>
        /// タブ名
        /// </summary>
        public string Name { get; set; } = "Home";

        /// <summary>
        /// ブラウズボタン
        /// </summary>
        public string BrowseText { get; set; } = "Browse...";

        /// <summary>
        /// ドラッグ・アンド・ドロップで画像ファイルをWebPに変換
        /// </summary>
        public string Message { get; set; } = "Drag and drop image file(s) to convert WebP.";

        /// <summary>
        /// ダイアログのインスタンス
        /// </summary>
        public IDialogCoordinator MahAppsDialogCoordinator { get; set; }


        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public HomeContentViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {
            // フォルダ選択ボタンイベントハンドラ
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;
            this.webPService = webPService;
        }

        /// <summary>
        /// ファイルブラウザボタンが押された
        /// </summary>
        private async void ExecuteBrowseButtonCommand()
        {
            // ダイアログを定義
            FolderPicker picker = new FolderPicker()
            {
                ViewMode = PickerViewMode.Thumbnail,
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
                FileTypeFilter = { ".jpg", ".jpeg", ".png", ".bmp", ".gif", ".jxr" }
            };
            // ウィンドウバンドルを取得
            IInitializeWithWindow withWindow = picker.As<IInitializeWithWindow>();
            withWindow.Initialize(new WindowInteropHelper(Application.Current.MainWindow).Handle);

            // ファイルダイアログを表示
            StorageFolder folder = await picker.PickSingleFolderAsync();

            if (folder == null)
            {
                return;
            }

            // Application now has read/write access to all contents in the picked folder
            // (including other sub-folder contents)
            // Windows.Storage.AccessCache.StorageApplicationPermissions.FutureAccessList.AddOrReplace("PickedFolderToken", folder);

            // ファイル一覧
            string[] files = Directory.GetFiles(folder.Path, "*.jpg, *.jpeg, *.png, *.bmp, *.gif, *.jxr", SearchOption.TopDirectoryOnly);

            // 変換処理
            webPService.Convert(files, Shell);
        }
    }
}
