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
using System.Diagnostics;
using System.Reactive.Disposables;
using System.Windows;

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
        /// WebPサービス
        /// </summary>
        public IWebPService webPService;

        /// <summary>
        /// タイトル
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        public string AboutText { get; set; } = "About";

        /// <summary>
        /// アバウトボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand AboutButtonCommand { get; set; }

        public string ConfigText { get; set; } = "Config";

        /// <summary>
        /// 設定ボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand ConfigButtonCommand { get; set; }


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
    }
}
