using DropWebP.Services;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using System.Diagnostics;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// Defines the <see cref="ConfigFlyoutViewModel" />.
    /// </summary>
    public class ConfigFlyoutViewModel : BindableBase, INavigationAware
    {
        /// <summary>
        /// Defines the eventAggregator.
        /// </summary>
        private IEventAggregator eventAggregator;

        /// <summary>
        /// Defines the regionManager.
        /// </summary>
        private IRegionManager regionManager;

        /// <summary>
        /// Gets or sets the Header
        /// Flyoutの題名.
        /// </summary>
        public string Header { get; set; } = "Config";

        /// <summary>
        /// Flyout開閉フラグ.
        /// </summary>
        private bool isOpen = false;

        /// <summary>
        /// Gets or sets a value indicating whether IsOpen.
        /// </summary>
        public bool IsOpen
        {
            get { return isOpen; }
            set { SetProperty(ref isOpen, value); }
        }

        /// <summary>
        /// Flyoutの位置.
        /// </summary>
        public Position Position { get; set; } = Position.Right;

        /// <summary>
        /// Flyoutの閉じるボタン.
        /// </summary>
        public DelegateCommand CloseFlyoutCommand { get; private set; }

        /// <summary>
        /// Gets or sets the LosslessText
        /// 可逆圧縮.
        /// </summary>
        public string LosslessText { get; set; } = "Lossless";

        /// <summary>
        /// 可逆圧縮スイッチ.
        /// </summary>
        public bool ToggleLossless { get => Properties.Settings.Default.Lossless; set => Properties.Settings.Default.Lossless = value; }

        /// <summary>
        /// 圧縮レベル.
        /// </summary>
        public string QualityText { get; set; } = "Quality";

        /// <summary>
        /// 圧縮レベルスライダー.
        /// </summary>
        public long QualityValue { get => Properties.Settings.Default.Quality; set => Properties.Settings.Default.Quality = value; }

        /// <summary>
        /// 変換前のファイルを残す.
        /// </summary>
        public string KeepOriginalText { get; set; } = "Keep Original";

        /// <summary>
        /// 変換前のファイルを残すチェックボックス.
        /// </summary>
        public bool ToggleKeepOriginal { get => Properties.Settings.Default.KeepOriginal; set => Properties.Settings.Default.KeepOriginal = value; }

        /// <summary>
        /// Jpegを無視
        /// </summary>
        public string IgnoreJpegText { get; set; } = "Ignore JPEG image";

        /// <summary>
        /// Jpegを無視
        /// </summary>
        public bool ToggleIgnoreJpeg { get => Properties.Settings.Default.IgnoreJpeg; set => Properties.Settings.Default.IgnoreJpeg = value; }

        /// <summary>
        /// Initializes a new instance of the <see cref="ConfigFlyoutViewModel"/> class.
        /// </summary>
        /// <param name="eventAggregator">The eventAggregator<see cref="IEventAggregator"/>.</param>
        /// <param name="regionManager">The regionManager<see cref="IRegionManager"/>.</param>
        public ConfigFlyoutViewModel(IEventAggregator eventAggregator, IRegionManager regionManager)
        {
            CloseFlyoutCommand = new DelegateCommand(ExecuteSaveButtonCommand);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;

            _ = eventAggregator.GetEvent<MessageService>().Subscribe(OnMesageReceieved);
        }

        /// <summary>
        /// 設定保存.
        /// </summary>
        private void ExecuteSaveButtonCommand()
        {
            Debug.WriteLine("保存しました");
            // 設定を保存
            Properties.Settings.Default.Save();
            IsOpen = false;
        }

        /// <summary>
        /// イベントを受け取った.
        /// </summary>
        /// <param name="obj">.</param>
        private void OnMesageReceieved(string obj)
        {
            Debug.WriteLine("OnMesageReceieved", obj);
        }

        /// <summary>
        /// The OnNavigatedTo.
        /// </summary>
        /// <param name="navigationContext">The navigationContext<see cref="NavigationContext"/>.</param>
        public void OnNavigatedTo(NavigationContext navigationContext)
        {
            Debug.WriteLine("OnNavigatedTo", navigationContext);
        }

        /// <summary>
        /// The IsNavigationTarget.
        /// </summary>
        /// <param name="navigationContext">The navigationContext<see cref="NavigationContext"/>.</param>
        /// <returns>The <see cref="bool"/>.</returns>
        public bool IsNavigationTarget(NavigationContext navigationContext)
        {
            Debug.WriteLine("IsNavigationTarget", navigationContext);
            return true;
        }

        /// <summary>
        /// ナビゲーション発生.
        /// </summary>
        /// <param name="navigationContext">.</param>
        public void OnNavigatedFrom(NavigationContext navigationContext)
        {
            // Debug.WriteLine("OnNavigatedFrom");
            // flyoutを開く
            Properties.Settings.Default.Reload();
            IsOpen = true;
        }
    }
}
