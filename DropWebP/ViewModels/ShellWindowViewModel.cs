// -----------------------------------------------------------------------
// <copyright file="ShellWindowViewModel.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.ViewModels
{
    using DropWebP.Interfaces;
    using DropWebP.Utility;
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
    using System.IO;
    using System.Linq;
    using System.Reactive.Disposables;
    using System.Reactive.Linq;
    using System.Runtime.InteropServices;
    using System.Windows;
    using System.Windows.Media;
    using System.Windows.Media.Imaging;
    using Windows.Storage;
    using Windows.Storage.Pickers;
    using WinRT;

    /// <summary>
    /// シェル画面.
    /// </summary>
    public class ShellWindowViewModel : BindableBase
    {
        /// <summary>
        /// イベントアグリエイター..
        /// </summary>
        private readonly IEventAggregator EventAggregator;

        /// <summary>
        /// リージョンマネージャー..
        /// </summary>
        private readonly IRegionManager RegionManager;

        /// <summary>
        /// ダイアログサービス..
        /// </summary>
        private readonly IDialogService DialogService;

        /// <summary>
        /// WebPサービス...
        /// </summary>
        private readonly IWebPService webPService;

        /// <summary>
        /// Gets or sets the Shell
        /// MetroWindow...
        /// </summary>
        public MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// Gets the ClipboardUpdateCommand
        /// クリップボード更新の監視...
        /// </summary>
        public DelegateCommand ClipboardUpdateCommand { get; private set; }

        /// <summary>
        /// Gets the ClosedCommand
        /// MainWindowのCloseイベント...
        /// </summary>
        public ReactiveCommand ClosedCommand { get; } = new ReactiveCommand();

        /// <summary>
        /// Gets the Disposable
        /// Disposeが必要な処理をまとめてやる...
        /// </summary>
        private CompositeDisposable Disposable { get; } = new CompositeDisposable();

        /// <summary>
        /// Gets or sets the Title
        /// タイトル...
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        /// <summary>
        /// Gets or sets the AboutButtonCommand
        /// アバウトボタンクリック時のコマンド...
        /// </summary>
        public DelegateCommand AboutCommand { get; set; }

        /// <summary>
        /// Gets or sets a value indicating whether IsImage.
        /// </summary>
        public bool IsImage { get; set; }

        /// <summary>
        /// Gets or sets the ConfigCommand
        /// 設定ボタンクリック時のコマンド...
        /// </summary>
        public DelegateCommand ConfigCommand { get; set; }

        /// <summary>
        /// Gets or sets the OpenCommand
        /// 画像を開くコマンド...
        /// </summary>
        public DelegateCommand OpenCommand { get; set; }

        /// <summary>
        /// Gets or sets the OpenFolderCommand
        /// フォルダを開くコマンド...
        /// </summary>
        public DelegateCommand OpenFolderCommand { get; set; }

        /// <summary>
        /// Gets or sets the PasteCommand
        /// ペーストコマンド...
        /// </summary>
        public DelegateCommand PasteCommand { get; set; }

        /// <summary>
        /// Gets or sets the ExitCommand
        /// 終了コマンド...
        /// </summary>
        public DelegateCommand ExitCommand { get; set; }

        /// <summary>
        /// Gets the ViewImage
        /// 表示するイメージのファイル名...
        /// </summary>
        public ReactivePropertySlim<string> ViewImage { get; } = new ReactivePropertySlim<string>();

        /// <summary>
        /// Gets the PreviewDragOverCommand
        /// ImageのPreviewDragOverイベントのコマンド...
        /// </summary>
        public ReactiveCommand<DragEventArgs> PreviewDragOverCommand { get; } = new ReactiveCommand<DragEventArgs>();

        /// <summary>
        /// Gets the DropCommand
        /// Imageのイベントのコマンド...
        /// </summary>
        public ReactiveCommand<DragEventArgs> DropCommand { get; } = new ReactiveCommand<DragEventArgs>();

        /// <summary>
        /// Initializes a new instance of the <see cref="ShellWindowViewModel"/> class.
        /// </summary>
        /// <param name="eventAggregator">The eventAggregator<see cref="IEventAggregator"/>.</param>
        /// <param name="regionManager">インジェクションするIRegionManager。.</param>
        /// <param name="dialogService">The dialogService<see cref="IDialogService"/>.</param>
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

            IDisposable watcher = Observable
                .Timer(TimeSpan.FromMilliseconds(200))  // 200ms毎に               
                .Where(_ => Clipboard.ContainsImage())   // クリップボードに画像データがあるかを確認
                .Repeat()                               // 上記の監視を何度も繰り返す     
                .DistinctUntilChanged()                 // 以前の結果と違う場合のみ
                .Subscribe(OnClipboardUpdate);

            // ペースト  
            PasteCommand = new DelegateCommand(ExecutePasteCommand);

            // 画像ファイルを開く（未実装）
            OpenCommand = new DelegateCommand(ExecuteOpenCommand);

            // 画像ファイルを開く（未実装）
            OpenFolderCommand = new DelegateCommand(ExecuteOpenFolderCommand);

            // 終了
            ExitCommand = new DelegateCommand(ExecuteExitCommand);

            // アバウトボタンイベントハンドラ
            AboutCommand = new DelegateCommand(ShowAboutDialog);

            // 設定ボタンイベントハンドラ
            ConfigCommand = new DelegateCommand(ShowConfigFloyout);

            Debug.WriteLine("イベント割当");

            EventAggregator = eventAggregator;
            RegionManager = regionManager;
            DialogService = dialogService;
            this.webPService = webPService;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="ShellWindowViewModel"/> class.
        /// </summary>
        public ShellWindowViewModel()
        {
        }

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
            DialogService.ShowDialog("AboutDialog");
        }

        /// <summary>
        /// 設定画面を開く.
        /// </summary>
        private void ShowConfigFloyout()
        {
            Debug.WriteLine("設定ボタンクリック");

            // eventAggregator.GetEvent<MessageService>().Publish("Top");
            RegionManager.RequestNavigate("FlyoutRegion", "ConfigFlyOut");
        }

        /// <summary>
        /// クリップボード監視.
        /// </summary>
        /// <param name="obj">The obj<see cref="long"/>.</param>
        private void OnClipboardUpdate(long obj)
        {
            Debug.WriteLine("clipboard changed");
            Windows.ApplicationModel.DataTransfer.DataPackageView content = Windows.ApplicationModel.DataTransfer.Clipboard.GetContent();
            IsImage = content.Contains(Windows.ApplicationModel.DataTransfer.StandardDataFormats.Bitmap);
        }

        /// <summary>
        /// ファイルを開くときのメソッド.
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
            IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(Application.Current.MainWindow);
            WinRT.Interop.InitializeWithWindow.Initialize(picker, hwnd);

            // var hwnd = GetActiveWindow();
            // var withWindow = picker.As<IInitializeWithWindow>();
            // withWindow.Initialize(hwnd);

            StorageFile selectedFile = await picker.PickSingleFileAsync();
            if (selectedFile == null)
            {
                // 選択個数が0のとき中止
                return;
            }

            // エンコード
            webPService.ConvertWebP(selectedFile.Path, Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
        }

        /// <summary>
        /// ファイルブラウザボタンが押された.
        /// </summary>
        private void ExecuteOpenFolderCommand()
        {
            IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(Application.Current.MainWindow);
            FolderPicker folderPicker = new Windows.Storage.Pickers.FolderPicker();
            WinRT.Interop.InitializeWithWindow.Initialize(folderPicker, hwnd);

            // ダイアログを定義
            FolderPickerEx picker = new();

            // ファイルダイアログを表示
            StorageFolder folder = picker.PickSingleFolder();

            if (folder == null)
            {
                return;
            }

            // 変換処理
            webPService.Convert(Directory.GetFiles(folder.Path), Shell);
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

            MemoryStream ms = data.GetData("DeviceIndependentBitmap") as System.IO.MemoryStream;
            if (ms == null)
            {
                return;
            }

            // DeviceIndependentBitmapのbyte配列の15番目がbpp、
            // これが32未満ならBgr32へ変換、これでアルファの値が0でも255扱いになって表示される
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
            IntPtr hwnd = GetActiveWindow();
            IInitializeWithWindow withWindow = picker.As<IInitializeWithWindow>();
            withWindow.Initialize(hwnd);

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
        /// 終了コマンド.
        /// </summary>
        public static void ExecuteExitCommand()
        {
            Application.Current.Shutdown();
        }

        /// <summary>
        /// アクティブなウィンドウのハンドルを取得.
        /// </summary>
        /// <returns>.</returns>
        [DllImport("user32.dll", ExactSpelling = true, CharSet = CharSet.Auto, PreserveSig = true, SetLastError = false)]
        private static extern IntPtr GetActiveWindow();
    }
}
