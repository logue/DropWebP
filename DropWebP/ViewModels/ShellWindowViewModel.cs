using DropWebP.Interfaces;
using DropWebP.Views;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System.Diagnostics;
using System.Reactive.Disposables;
using System.Windows;

namespace DropWebP.ViewModels
{
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
        /// プログレスリングの表示制御
        /// </summary>
        public bool ProgressRing { get; set; } = false;

        /// <summary>
        /// WebPサービス
        /// </summary>
        public IWebPService webPService;

        /// <summary>
        /// タイトル
        /// </summary>
        public string Title { get; set; } = "DropWebP";

        /// <summary>
        /// 設定ボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand AboutButtonCommand { get; set; }

        /// <summary>
        /// 設定ボタンクリック時のコマンド
        /// </summary>
        public DelegateCommand ConfigButtonCommand { get; set; }


        /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public ShellWindowViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {
            // リージョン登録
            _ = regionManager.RegisterViewWithRegion("ContentRegion", typeof(HomeContent));
            _ = regionManager.RegisterViewWithRegion("FlyoutRegion", typeof(ConfigFlyout));
            _ = regionManager.RegisterViewWithRegion("AboutRegion", typeof(AboutDialog));

            // ドラッグアンドドロップハンドラ
            _ = ClosedCommand.Subscribe(Close).AddTo(Disposable);

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
        }

        /// <summary>
        /// 設定画面を開く
        /// </summary>
        private void ShowConfigFloyout()
        {
            Debug.WriteLine("設定ボタンクリック");
            regionManager.RequestNavigate("FlyoutRegion", "ConfigFlyout");
        }
    }
}
