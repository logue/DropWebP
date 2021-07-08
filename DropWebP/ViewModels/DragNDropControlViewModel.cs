using DropWebP.Interfaces;
using Prism.Commands;
using Prism.Mvvm;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.ComponentModel;
using System.Diagnostics;
using System.Reactive.Disposables;
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
    public class DragNDropControlViewModel : BindableBase, INotifyPropertyChanged
    {
        /// <summary>
		/// MainWindowのCloseイベント
		/// </summary>
		public ReactiveCommand ClosedCommand { get; } = new ReactiveCommand();

        /// <summary>
        /// Disposeが必要な処理をまとめてやる
        /// </summary>
        private CompositeDisposable Disposable { get; } = new CompositeDisposable();

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

        public IWebPEncorderService webPEncorderService;

        private bool ToggleLossless
        {
            get => _toggleLossless;
            set
            {
                _toggleLossless = value;
            }
        }


        private bool _toggleLossless = false;
        /// <summary>
        /// ブラウズボタン
        /// </summary>
        private string _browseText = "Browse...";
        public string BrowseText
        {
            get { return _browseText; }
            set { SetProperty(ref _browseText, value); }
        }

        private StatusBarViewModel statusBarViewModel;
        /// <summary>
        /// コンストラクタ
        /// </summary>
        public DragNDropControlViewModel(IWebPEncorderService webPEncorderService)
        {
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);
            PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            DropCommand.Subscribe(ImageDrop).AddTo(Disposable);
            ClosedCommand.Subscribe(Close).AddTo(Disposable);
            this.webPEncorderService = webPEncorderService;
        }
        /// <summary>
		/// ウィンドウが閉じられるイベント
		/// </summary>
		private void Close()
        {
            Disposable.Dispose();
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
                webPEncorderService.EncordeWebP(file.Path, _toggleLossless ? -1 : 95);
            }
            else
            {
                Debug.WriteLine("Operation cancelled.");
            }
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
            // 複数ドロップの可能性もあるので、今回は最初のファイルを選択して表示
            webPEncorderService.EncordeWebP(files[0], _toggleLossless ? -1 : 95);
        }
    }
}
