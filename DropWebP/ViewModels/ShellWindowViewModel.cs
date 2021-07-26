using DropWebP.Interfaces;
using DropWebP.Services;
using DropWebP.Views;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.Reactive.Disposables;
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
    /// シェル画面
    /// </summary>
    public class ShellWindowViewModel : BindableBase
    {
        private IEventAggregator eventAggregator;
        private IRegionManager regionManager;

        /// <summary>
        /// WebPサービス
        /// </summary>
        private IWebPService webPService;

        /// <summary>
        /// MetroWindow
        /// </summary>
        public MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// クリップボード監視
        /// </summary>
        public DelegateCommand ClipboardUpdateCommand { get; private set; }

        /// <summary>
        /// MainWindowのCloseイベント
        /// </summary>
        public ReactiveCommand ClosedCommand { get; } = new ReactiveCommand();

        /// <summary>
        /// Disposeが必要な処理をまとめてやる
        /// </summary>
        private CompositeDisposable Disposable { get; } = new CompositeDisposable();

        /// <summary>
        /// タイトル
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        /// <summary>
        /// アバウトボタンテキスト
        /// </summary>
        public string AboutText { get; set; } = "About";

        /// <summary>
        /// アバウトボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand AboutButtonCommand { get; set; }

        /// <summary>
        /// 設定ボタンテキスト
        /// </summary>
        public string ConfigText { get; set; } = "Config";

        public bool IsImage { get; set; }

        /// <summary>
        /// 設定ボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand ConfigButtonCommand { get; set; }

        /// <summary>
        /// ペーストコマンド
        /// </summary>
        public DelegateCommand PasteCommand { get; set; }

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

        /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public ShellWindowViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {
            // リージョン登録
            _ = regionManager.RegisterViewWithRegion("ContentRegion", typeof(HomeContent));
            _ = regionManager.RegisterViewWithRegion("FlyoutRegion", typeof(ConfigFlyout));

            // ドラッグアンドドロップハンドラ
            _ = ClosedCommand.Subscribe(Close).AddTo(Disposable);
            _ = PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            _ = DropCommand.Subscribe(ImageDrop).AddTo(Disposable);

            ClipboardUpdateCommand = new DelegateCommand(OnClipboardUpdate);

            // CTRL+V押下ハンドラ
            PasteCommand = new DelegateCommand(ExecutePasteCommand);

            // アバウトボタンイベントハンドラ
            AboutButtonCommand = new DelegateCommand(ShowAboutDialog, CanClick);
            // 設定ボタンイベントハンドラ
            ConfigButtonCommand = new DelegateCommand(ShowConfigFloyout, CanClick);

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

            webPService.Convert(files, Shell);
        }

        private bool CanClick()
        {
            return true;
        }

        /// <summary>
        /// 設定画面を開く
        /// </summary>
        private void ShowAboutDialog()
        {
            Debug.WriteLine("アバウトクリック");
            // TODO:
        }

        /// <summary>
        /// 設定画面を開く
        /// </summary>
        private void ShowConfigFloyout()
        {
            Debug.WriteLine("設定ボタンクリック");
            eventAggregator.GetEvent<MessageService>().Publish("Top");
            regionManager.RequestNavigate("FlyoutRegion", "ConfigFlyOut");
        }

        /// <summary>
        /// クリップボード監視
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="e"></param>
        private void OnClipboardUpdate()
        {
            IsImage = Clipboard.ContainsImage();
            Debug.WriteLine(IsImage);
        }

        /// <summary>
        /// クリップボードから画像を受け取ったときのメソッド
        /// </summary>
        private async void ExecutePasteCommand()
        {
            // 画像でない場合
            // if (!IsImage) return;

            // クリップボードからビットマップ画像を取得
            var data = Clipboard.GetDataObject();
            if (data == null) return;

            var ms = data.GetData("DeviceIndependentBitmap") as System.IO.MemoryStream;
            if (ms == null) return;

            //DeviceIndependentBitmapのbyte配列の15番目がbpp、
            //これが32未満ならBgr32へ変換、これでアルファの値が0でも255扱いになって表示される
            byte[] dib = ms.ToArray();

            // BitmapSourceを取得
            BitmapSource bitmapSource = (dib[14] < 32) ? new FormatConvertedBitmap(Clipboard.GetImage(), PixelFormats.Bgr32, null, 0) : Clipboard.GetImage();

            // Bitmap型に変換
            Bitmap bitmap = new Bitmap(
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
            FileSavePicker picker = new FileSavePicker()
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
            if (file != null)
            {
                // エンコード
                byte[] bytes = webPService.EncodeWebP(bitmap, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
                // 書き出し
                await FileIO.WriteBytesAsync(file, bytes);
            }
        }

        /// <summary>
        /// エクセルからのコピーなのかを判定、フォーマット形式にEnhancedMetafileがあればエクセル判定
        /// </summary>
        /// <returns></returns>
        private bool IsExcel()
        {
            string[] formats = Clipboard.GetDataObject().GetFormats();
            foreach (var item in formats)
            {
                if (item == "EnhancedMetafile")
                {
                    return true;
                }
            }
            return false;
        }
    }
}
