using DropWebP.Interfaces;
using Prism.Commands;
using Prism.Mvvm;
using Reactive.Bindings;
using System;
using System.Diagnostics;
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
    public class HomeTabItemlViewModel : BindableBase
    {
        /// <summary>
        /// Webのエンコーダー
        /// </summary>
        private IWebPEncorderService webPEncorderService;

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
        public string Message { get; set; } = "Convert image files to WebP by drag and drop";

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public HomeTabItemlViewModel(IWebPEncorderService webPEncorderService)
        {
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);

            this.webPEncorderService = webPEncorderService;
        }

        /// <summary>
        /// ファイルブラウザボタンが押された
        /// </summary>
        private async void ExecuteBrowseButtonCommand()
        {
            // ダイアログを定義
            FileOpenPicker picker = new FileOpenPicker()
            {
                ViewMode = PickerViewMode.Thumbnail,
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
                FileTypeFilter = { ".jpg", ".jpeg", ".png", ".bmp" }
            };
            // ウィンドウバンドルを取得
            IInitializeWithWindow withWindow = picker.As<IInitializeWithWindow>();
            withWindow.Initialize(new WindowInteropHelper(Application.Current.MainWindow).Handle);
            // ファイルダイアログを表示
            StorageFile file = await picker.PickSingleFileAsync();
            if (file != null)
            {
                // Application now has read/write access to the picked file
                Debug.WriteLine(file.Path);
                webPEncorderService.EncordeWebP(file.Path, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
            }
            else
            {
                Debug.WriteLine("Operation cancelled.");
            }
        }
    }
}
