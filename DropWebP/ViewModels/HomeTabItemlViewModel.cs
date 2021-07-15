using DropWebP.Interfaces;
using Prism.Commands;
using Prism.Mvvm;
using Reactive.Bindings;
using System;
using System.Windows;
using System.Windows.Interop;
using Windows.Storage;
using Windows.Storage.Pickers;
using Windows.UI.Popups;
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
        private IWebPService webPService;

        // private IStatusBarViewModel statusBarViewModel;

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
        /// コンストラクタ
        /// </summary>
        public HomeTabItemlViewModel(IWebPService webPService)
        {
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);

            this.webPService = webPService;
            // this.statusBarViewModel = statusBarViewModel;
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
            // IReadOnlyList<StorageFile> files = await picker.PickMultipleFilesAsync();

            if (file == null)
            {
                return;
            }

            webPService.ConvertWebP(file.Path, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
            MessageDialog dialog = new MessageDialog("Finish");
            withWindow = dialog.As<IInitializeWithWindow>();
            withWindow.Initialize(new WindowInteropHelper(Application.Current.MainWindow).Handle);
            _ = await dialog.ShowAsync();

            /*
            // ファイルの個数
            int count = files.Count;

            // ダイアログ
            MessageDialog dialog;
            if (count == 0)
            {
                // 選択されているファイルがない場合
                dialog = new MessageDialog("Operation cancelled.");
                return;
            }
            else
            {
                for (int i = 0; i <= count - 1; i++)
                {
                    webPService.ConvertWebP(files[i].Path, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
                }
                dialog = new MessageDialog("Finish");
            }
            withWindow = dialog.As<IInitializeWithWindow>();
            withWindow.Initialize(new WindowInteropHelper(Application.Current.MainWindow).Handle);

            _ = dialog.ShowAsync();
            */
        }
    }
}
