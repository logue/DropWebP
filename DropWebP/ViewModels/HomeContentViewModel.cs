using DropWebP.Interfaces;
using MahApps.Metro.Controls;
using MahApps.Metro.Controls.Dialogs;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.Collections.Generic;
using System.IO;
using System.Reactive.Disposables;
using System.Threading.Tasks;
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
        /// MainWindowのCloseイベント
        /// </summary>
        public ReactiveCommand ClosedCommand { get; } = new ReactiveCommand();

        /// <summary>
        /// Disposeが必要な処理をまとめてやる
        /// </summary>
        private CompositeDisposable Disposable { get; } = new CompositeDisposable();

        /// <summary>
        /// Webのエンコーダー
        /// </summary>
        private IWebPService webPService;

        /// <summary>
        /// 表示するイメージのファイル名
        /// </summary>
        public ReactivePropertySlim<string> ViewImage { get; } = new ReactivePropertySlim<string>();

        /// <summary>
        /// ImageのPreviewDragOverイベントのコマンド
        /// </summary>
        public ReactiveCommand<DragEventArgs> PreviewDragOverCommand { get; } = new ReactiveCommand<DragEventArgs>();

        /// <summary>
        /// Imageのイベントのコマンド
        /// </summary>
        public ReactiveCommand<DragEventArgs> DropCommand { get; } = new ReactiveCommand<DragEventArgs>();

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
        /// ダイアログ設定
        /// </summary>
        private MetroDialogSettings metroDialogSettings;

        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public HomeContentViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {

            // ドラッグアンドドロップハンドラ
            _ = ClosedCommand.Subscribe(Close).AddTo(Disposable);
            _ = PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            _ = DropCommand.Subscribe(ImageDrop).AddTo(Disposable);

            // Metroダイアログのデフォルト設定
            metroDialogSettings = new MetroDialogSettings()
            {
                AffirmativeButtonText = "OK",
                NegativeButtonText = "Cancel"
            };

            // フォルダ選択ボタンイベントハンドラ
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;
            this.webPService = webPService;
        }


        /// <summary>
        /// ウィンドウが閉じられるイベント
        /// </summary>
        private void Close()
        {
            Disposable.Dispose();
        }

        /// <summary>
        /// ImageのPreviewDragOverイベントに対する処理
        /// </summary>
        /// <param name="e"></param>
        private void ImagePreviewDragOver(DragEventArgs e)
        {
            // マウスカーソルをコピーにする。
            e.Effects = DragDropEffects.Copy;
            // ドラッグされてきたものがFileDrop形式の場合だけ、このイベントを処理済みにする。
            e.Handled = e.Data.GetDataPresent(DataFormats.FileDrop);
        }

        /// <summary>
        /// ImageのDropイベントに対する処理
        /// </summary>
        /// <param name="e"></param>
        private void ImageDrop(DragEventArgs e)
        {
            // ドロップされたものがFileDrop形式の場合は、各ファイルのパス文字列を文字列配列に格納する。
            string[] files = (string[])e.Data.GetData(DataFormats.FileDrop);

            Convert(files);
        }

        /// <summary>
        /// ファイルブラウザボタンが押された
        /// </summary>
        private async void ExecuteBrowseButtonCommand()
        {

            List<string> fileTypes = new List<string>();

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
            // IReadOnlyList<StorageFile> files = await picker.PickMultipleFilesAsync();

            if (folder == null)
            {
                return;
            }

            // Application now has read/write access to all contents in the picked folder
            // (including other sub-folder contents)
            Windows.Storage.AccessCache.StorageApplicationPermissions.
            FutureAccessList.AddOrReplace("PickedFolderToken", folder);

            // ファイル一蘭
            string[] files = Directory.GetFiles(folder.ToString(), "*.jpg, *.jpeg, *.png, *.bmp, *.gif, *.jxr", SearchOption.TopDirectoryOnly);

            // 変換処理
            Convert(files);
        }

        /// <summary>
        /// 変換処理
        /// </summary>
        /// <param name="files">変換対象のファイル</param>
        private async void Convert(string[] files)
        {
            // 全ファイル数
            int count = files.Length;

            // プログレスコントローラー
            ProgressDialogController controller = await Shell.ShowProgressAsync("Now converting", "Initializing....", false, metroDialogSettings);
            controller.SetIndeterminate();
            // キャンセルボタンが押されたときの設定
            controller.Canceled += async (object sender, EventArgs e) =>
            {
                controller.Minimum = 0;
                controller.Maximum = 0;
                await controller.CloseAsync();
            };
            // プログレスダイアログの進捗情報の登録
            controller.Minimum = 0;
            controller.Maximum = count;

            await Task.Run(() =>
            {
                // キャンセル可能
                controller.SetCancelable(true);
                for (int i = 0; i <= count; i++)
                {
                    if (i == count)
                    {
                        controller.SetCancelable(false);
                        break;
                    }
                    controller.SetProgress(i);
                    controller.SetMessage(files[i]);

                    // 変換処理
                    webPService.ConvertWebPAsync(files[i], Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
                }
            });
            await controller.CloseAsync();

            await Shell.ShowMessageAsync("Finish.", count.ToString() + " files are converted.");
        }
    }
}
