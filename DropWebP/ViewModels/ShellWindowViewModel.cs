using DropWebP.Interfaces;
using DropWebP.Services;
using DropWebP.Views;
using MahApps.Metro.Controls;
using MahApps.Metro.Controls.Dialogs;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using Reactive.Bindings.Extensions;
using System;
using System.Diagnostics;
using System.Reactive.Disposables;
using System.Threading.Tasks;
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
        public MetroWindow Shell { get; set; } = System.Windows.Application.Current.MainWindow as MetroWindow;

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
        /// プログレスリングの表示制御
        /// </summary>
        public bool ProgressRing { get; set; } = false;

        /// <summary>
        /// ステータスバーのビューモデル
        /// </summary>
        // public IStatusBarViewModel statusBarViewModel;

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

        public IDialogCoordinator MahAppsDialogCoordinator { get; set; }

        /// <summary>
        /// プログレスダイアログ設定
        /// </summary>
        private MetroDialogSettings metroDialogSettings;


        /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public ShellWindowViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {
            _ = regionManager.RegisterViewWithRegion("ContentRegion", typeof(HomeContent));
            _ = regionManager.RegisterViewWithRegion("FlyoutRegion", typeof(ConfigFlyout));
            _ = regionManager.RegisterViewWithRegion("AboutRegion", typeof(AboutDialog));
            _ = regionManager.RegisterViewWithRegion("StatusBarRegion", typeof(StatusBar));

            _ = PreviewDragOverCommand.Subscribe(ImagePreviewDragOver).AddTo(Disposable);
            _ = DropCommand.Subscribe(ImageDrop).AddTo(Disposable);
            _ = ClosedCommand.Subscribe(Close).AddTo(Disposable);

            metroDialogSettings = new MetroDialogSettings()
            {
                AffirmativeButtonText = "OK",
                NegativeButtonText = "Cancel"
            };

            AboutButtonCommand = new DelegateCommand(ShowAboutDialog, CanClick);
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
        private async void ImageDrop(DragEventArgs e)
        {
            // ドロップされたものがFileDrop形式の場合は、各ファイルのパス文字列を文字列配列に格納する。
            string[] files = (string[])e.Data.GetData(DataFormats.FileDrop);
            // ファイルの個数
            int count = files.Length;

            StatusBarEvent s = new StatusBarEvent();

            ProgressDialogController controller = await Shell.ShowProgressAsync("Now converting", "Converting an image file.", false, metroDialogSettings);
            controller.Canceled += async (object sender, EventArgs e) =>
            {
                s.message = "Cancelled.";
                controller.Minimum = 0;
                controller.Maximum = 0;
                await controller.CloseAsync();
            };
            controller.Minimum = 0;
            controller.Maximum = count;

            Debug.WriteLine(count);

            await Task.Run(() =>
            {
                s.message = "Initializing...";
                s.progressMaximum = count;
                s.progress = 0;

                // キャンセル可能
                controller.SetCancelable(true);

                // ステータスバーのビューモデルの更新
                eventAggregator.GetEvent<StatusBarService>().Publish(s);
                // 進捗リングを表示
                ProgressRing = true;

                for (int i = 0; i <= count; i++)
                {
                    if (i == count)
                    {
                        controller.SetCancelable(true);
                        break;
                    }
                    Debug.WriteLine(files[i]);
                    controller.SetProgress(i);
                    controller.SetMessage(files[i]);

                    s.message = files[i];
                    s.progress = i;
                    eventAggregator.GetEvent<StatusBarService>().Publish(s);
                    webPService.ConvertWebPAsync(files[i], Properties.Settings.Default.Lossless ? -1 : Properties.Settings.Default.Quality);
                }
                // 進捗リングを非表示
                ProgressRing = false;

                s.progress = 0;
                s.progressMaximum = 0;
                s.message = "Finish.";
                eventAggregator.GetEvent<StatusBarService>().Publish(s);
            });
            await controller.CloseAsync();

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
            // eventAggregator.GetEvent<StatusBarService>().Publish("アバウトクリック");
        }

        /// <summary>
        /// 設定画面を開く
        /// </summary>
        private void ShowConfigFloyout()
        {
            // eventAggregator.GetEvent<StatusBarService>().Publish("設定ボタンクリック");
            Debug.WriteLine("設定ボタンクリック");
            regionManager.RequestNavigate("FlyoutRegion", "ConfigFlyout");
        }
    }
}
