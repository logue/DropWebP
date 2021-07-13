using DropWebP.Interfaces;
using DropWebP.Views;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.Reactive.Disposables;
using System.Windows;

namespace DropWebP.ViewModels
{
    public class MainWindowViewModel : BindableBase
    {
        private IRegionManager regionManager;
        private IEventAggregator eventAggregator;

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

        public IWebPEncorderService webPEncorderService;

        /// <summary>
        /// タイトル
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public MainWindowViewModel(IRegionManager regionManager, IEventAggregator eventAggregator, IWebPEncorderService webPEncorderService)
        {
            regionManager.RegisterViewWithRegion("ContentRegion", typeof(HomeTabItem));
            regionManager.RegisterViewWithRegion("ContentRegion", typeof(ConfigTabItem));
            regionManager.RegisterViewWithRegion("ContentRegion", typeof(AboutTabItem));
            regionManager.RegisterViewWithRegion("StatusBarRegion", typeof(StatusBar));

            PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            DropCommand.Subscribe(ImageDrop).AddTo(Disposable);
            ClosedCommand.Subscribe(Close).AddTo(Disposable);

            this.regionManager = regionManager;
            this.eventAggregator = eventAggregator;

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
            webPEncorderService.EncordeWebP(files[0], -1);
        }
    }
}
