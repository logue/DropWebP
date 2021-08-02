using DropWebP.Interfaces;
using DropWebP.Views;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Prism.Services.Dialogs;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.Linq;
using System.Reactive.Disposables;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Interop;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using Windows.Storage;
using Windows.Storage.Pickers;
using WinRT;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// シェル画面.
    /// </summary>
    public class ShellWindowViewModel : BindableBase
    {
        /// <summary>
        /// イベントアグリエイター
        /// </summary>
        private readonly IEventAggregator eventAggregator;

        /// <summary>
        /// リージョンマネージャー
        /// </summary>
        private readonly IRegionManager regionManager;

        /// <summary>
        /// ダイアログサービス
        /// </summary>
        private readonly IDialogService dialogService;

        /// <summary>
        /// WebPサービス.
        /// </summary>
        private readonly IWebPService webPService;

        /// <summary>
        /// Gets or sets the Shell
        /// MetroWindow.
        /// </summary>
        public MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// クリップボード更新の監視.
        /// </summary>
        public DelegateCommand ClipboardUpdateCommand { get; private set; }

        /// <summary>
        /// MainWindowのCloseイベント.
        /// </summary>
        public ReactiveCommand ClosedCommand { get; } = new ReactiveCommand();

        /// <summary>
        /// Disposeが必要な処理をまとめてやる.
        /// </summary>
        private CompositeDisposable Disposable { get; } = new CompositeDisposable();

        /// <summary>
        /// Gets or sets the Title
        /// タイトル.
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        /// <summary>
        /// Gets or sets the AboutText
        /// アバウトボタンテキスト.
        /// </summary>
        public string AboutText { get; set; } = "About";

        /// <summary>
        /// Gets or sets the AboutButtonCommand
        /// アバウトボタンクリック時のコマンド.
        /// </summary>
        public DelegateCommand AboutCommand { get; set; }

        /// <summary>
        /// Gets or sets the ConfigText
        /// 設定ボタンテキスト.
        /// </summary>
        public string ConfigText { get; set; } = "Config";

        /// <summary>
        /// Gets or sets a value indicating whether IsImage.
        /// </summary>
        public bool IsImage { get; set; }

        /// <summary>
        /// 設定ボタンクリック時のコマンド.
        /// </summary>
        public DelegateCommand ConfigCommand { get; set; }

        /// <summary>
        /// 画像を開くコマンド.
        /// </summary>
        public DelegateCommand OpenCommand { get; set; }

        /// <summary>
        /// ペーストコマンド.
        /// </summary>
        public DelegateCommand PasteCommand { get; set; }

        /// <summary>
        /// 終了コマンド.
        /// </summary>
        public DelegateCommand ExitCommand { get; set; }

        /// <summary>
        /// 表示するイメージのファイル名.
        /// </summary>
        public ReactivePropertySlim<string> ViewImage { get; } = new ReactivePropertySlim<string>();

        /// <summary>
        /// ImageのPreviewDragOverイベントのコマンド.
        /// </summary>
        public ReactiveCommand<DragEventArgs> PreviewDragOverCommand { get; } = new ReactiveCommand<DragEventArgs>();

        /// <summary>
        /// Imageのイベントのコマンド.
        /// </summary>
        public ReactiveCommand<DragEventArgs> DropCommand { get; } = new ReactiveCommand<DragEventArgs>();

        /// <summary>
        /// コンストラクタ.
        /// </summary>
        /// <param name="eventAggregator">The eventAggregator<see cref="IEventAggregator"/>.</param>
        /// <param name="regionManager">インジェクションするIRegionManager。.</param>
        /// <param name="webPService">The webPService<see cref="IWebPService"/>.</param>
        public ShellWindowViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IDialogService dialogService, IWebPService webPService)
        {
            // リージョン登録
            _ = regionManager.RegisterViewWithRegion("ContentRegion", typeof(HomeContent));
            _ = regionManager.RegisterViewWithRegion("FlyoutRegion", typeof(ConfigFlyout));

            // ドラッグアンドドロップハンドラ
            _ = ClosedCommand.Subscribe(Close).AddTo(Disposable);
            _ = PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            _ = DropCommand.Subscribe(ImageDrop).AddTo(Disposable);

            // クリップボードが更新されれた時のハンドラ
            ClipboardUpdateCommand = new DelegateCommand(OnClipboardUpdate);
            // ペースト  
            PasteCommand = new DelegateCommand(ExecutePasteCommand);
            // 画像ファイルを開く（未実装）
            OpenCommand = new DelegateCommand(ExecuteOpenCommand);
            // 終了
            ExitCommand = new DelegateCommand(ExecuteExitCommand);
            // アバウトボタンイベントハンドラ
            AboutCommand = new DelegateCommand(ShowAboutDialog);
            // 設定ボタンイベントハンドラ
            ConfigCommand = new DelegateCommand(ShowConfigFloyout);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;
            this.dialogService = dialogService;
            this.webPService = webPService;
        }

        public ShellWindowViewModel() { }

        /// <summary>
        /// ウィンドウが閉じられるイベント.
        /// </summary>
        private void Close()
        {
            Disposable.Dispose();
        }

        /// <summary>
        /// ImageのPreviewDragOverイベントに対する処理.
        /// </summary>
        /// <param name="e">.</param>
        private void ImagePreviewDragOver(DragEventArgs e)
        {
            // マウスカーソルをコピーにする。
            e.Effects = DragDropEffects.Copy;
            // ドラッグされてきたものがFileDrop形式の場合だけ、このイベントを処理済みにする。
            e.Handled = e.Data.GetDataPresent(DataFormats.FileDrop);
        }

        /// <summary>
        /// ImageのDropイベントに対する処理.
        /// </summary>
        /// <param name="e">.</param>
        private void ImageDrop(DragEventArgs e)
        {
            // ドロップされたものがFileDrop形式の場合は、各ファイルのパス文字列を文字列配列に格納する。
            string[] files = (string[])e.Data.GetData(DataFormats.FileDrop);
            webPService.Convert(files, Shell);
        }

        /// <summary>
        /// 設定画面を開く.
        /// </summary>
        private void ShowAboutDialog()
        {
            Debug.WriteLine("アバウトクリック");
            dialogService.ShowDialog("AboutDialog");
        }

        /// <summary>
        /// 設定画面を開く.
        /// </summary>
        private void ShowConfigFloyout()
        {
            Debug.WriteLine("設定ボタンクリック");
            // eventAggregator.GetEvent<MessageService>().Publish("Top");
            regionManager.RequestNavigate("FlyoutRegion", "ConfigFlyOut");
        }

        /// <summary>
        /// クリップボード監視.
        /// </summary>
        private void OnClipboardUpdate()
        {
            IsImage = Clipboard.ContainsImage();
            Debug.WriteLine(IsImage);
        }

        /// <summary>
        /// ファイルを開くときのメソッド
        /// </summary>
        private async void ExecuteOpenCommand()
        {
            // TODO: 動きません！
            // メモ：https://github.com/microsoft/WindowsAppSDK/issues/466

            Debug.WriteLine("開く");
            ImageCodecInfo[] codecs = ImageCodecInfo.GetImageEncoders();
            foreach (ImageCodecInfo c in codecs)
            {
                // string codecName = c.CodecName.Substring(8).Replace("Codec", "Files").Trim();
                // mfolderPicker.FileTypeFilter.Add(codecName, new List<string>() { c.FilenameExtension });
                Debug.WriteLine(c.FilenameExtension);
                //mfolderPicker.FileTypeFilter.Add(c.FilenameExtension);
            }

            FileOpenPicker picker = new()
            {
                ViewMode = PickerViewMode.Thumbnail,
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
            };

            // ウィンドウバンドルを取得
            IInitializeWithWindow initializeWithWindowWrapper = picker.As<IInitializeWithWindow>();
            IntPtr hwnd = GetActiveWindow();
            initializeWithWindowWrapper.Initialize(hwnd);

            IReadOnlyList<StorageFile> selectedFiles = await picker.PickMultipleFilesAsync();
            if (selectedFiles.Count() == 0)
            {
                // 選択個数が0のとき中止
                return;
            }
            string[] files = selectedFiles.Select(s => s.Path).ToArray();
            webPService.Convert(files, Shell);
        }

        /// <summary>
        /// クリップボードから画像を受け取ったときのメソッド.
        /// </summary>
        private async void ExecutePasteCommand()
        {
            // 画像でない場合
            // if (!IsImage) return;

            // クリップボードからビットマップ画像を取得
            IDataObject data = Clipboard.GetDataObject();
            if (data == null)
            {
                return;
            }

            System.IO.MemoryStream ms = data.GetData("DeviceIndependentBitmap") as System.IO.MemoryStream;
            if (ms == null)
            {
                return;
            }

            //DeviceIndependentBitmapのbyte配列の15番目がbpp、
            //これが32未満ならBgr32へ変換、これでアルファの値が0でも255扱いになって表示される
            byte[] dib = ms.ToArray();

            // BitmapSourceを取得
            BitmapSource bitmapSource = (dib[14] < 32) ? new FormatConvertedBitmap(Clipboard.GetImage(), PixelFormats.Bgr32, null, 0) : Clipboard.GetImage();

            // Bitmap型に変換
            Bitmap bitmap = new(
                bitmapSource.PixelWidth,
                bitmapSource.PixelHeight,
                System.Drawing.Imaging.PixelFormat.Format32bppPArgb
            );
            BitmapData bitmapData = bitmap.LockBits(
                new Rectangle(System.Drawing.Point.Empty, bitmap.Size),
                ImageLockMode.WriteOnly,
                System.Drawing.Imaging.PixelFormat.Format32bppPArgb
            );
            bitmapSource.CopyPixels(
                Int32Rect.Empty,
                bitmapData.Scan0,
                bitmapData.Height * bitmapData.Stride,
                bitmapData.Stride
            );
            bitmap.UnlockBits(bitmapData);

            // ダイアログを定義
            FileSavePicker picker = new()
            {
                SuggestedFileName = "image",
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
                FileTypeChoices = { { "WebP Image", new List<string>() { ".webp" } } }
            };
            // ウィンドウバンドルを取得
            IInitializeWithWindow withWindow = picker.As<IInitializeWithWindow>();
            withWindow.Initialize(new WindowInteropHelper(Application.Current.MainWindow).Handle);

            // ファイルダイアログを表示4
            StorageFile file = await picker.PickSaveFileAsync();
            if (file == null)
            {
                return;
            }
            // エンコード
            byte[] bytes = webPService.EncodeWebP(bitmap, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
            // 書き出し
            await FileIO.WriteBytesAsync(file, bytes);
        }
        /// <summary>
        /// 終了コマンド
        /// </summary>
        public void ExecuteExitCommand()
        {
            Application.Current.Shutdown();
        }

        /// <summary>
        /// アクティブなウィンドウのハンドルを取得
        /// </summary>
        /// <returns></returns>
        [DllImport("user32.dll", ExactSpelling = true, CharSet = CharSet.Auto, PreserveSig = true, SetLastError = false)]
        public static extern IntPtr GetActiveWindow();
    }
}
