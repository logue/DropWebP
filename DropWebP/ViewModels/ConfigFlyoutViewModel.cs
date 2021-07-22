using DropWebP.Services;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using System.Diagnostics;

namespace DropWebP.ViewModels
{
    public class ConfigFlyoutViewModel : BindableBase, INavigationAware
    {
        private IEventAggregator eventAggregator;
        private IRegionManager regionManager;

        /// <summary>
        /// Flyoutの題名
        /// </summary>
        public string Header { get; set; } = "Config";

        /// <summary>
        /// Flyout開閉フラグ
        /// </summary>
        public bool IsOpen { get; set; } = false;

        /// <summary>
        /// Flyoutの高さ
        /// </summary>
        public int Height { get; set; } = 200;

        /// <summary>
        /// Flyoutの位置
        /// </summary>
        public Position Position { get; set; } = Position.Left;

        /// <summary>
        /// 可逆圧縮
        /// </summary>
        public string LosslessText { get; set; } = "Lossless";

        /// <summary>
        /// 可逆圧縮スイッチ
        /// </summary>
        public bool ToggleLossless
        {
            get => Properties.Settings.Default.Lossless;
            set => Properties.Settings.Default.Lossless = value;
        }

        /// <summary>
        /// 圧縮レベル
        /// </summary>
        public string QualityText { get; set; } = "Quality";

        /// <summary>
        /// 圧縮レベルスライダー
        /// </summary>
        public long QualityValue
        {
            get => Properties.Settings.Default.Quality;
            set => Properties.Settings.Default.Quality = value;
        }

        /// <summary>
        /// 変換前のファイルを残す
        /// </summary>
        public string KeepOriginalText { get; set; } = "Keep Original";

        /// <summary>
        /// 変換前のファイルを残すチェックボックス
        /// </summary>
        public bool ToggleKeepOriginal
        {
            get => Properties.Settings.Default.KeepOriginal;
            set => Properties.Settings.Default.KeepOriginal = value;
        }

        /// <summary>
        /// 設定保存ボタンのコマンド
        /// </summary>
        public DelegateCommand SaveButtonCommand { get; }

        /// <summary>
        /// 保存ボタン
        /// </summary>
        public string SaveText { get; set; } = "Save Configure";

        private string data;

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public ConfigFlyoutViewModel(IEventAggregator eventAggregator, IRegionManager regionManager)
        {
            SaveButtonCommand = new DelegateCommand(ExecuteSaveButtonCommand);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;

            _ = eventAggregator.GetEvent<MessageService>().Subscribe(OnMesageReceieved);
        }

        /// <summary>
        /// 設定保存
        /// </summary>
        private void ExecuteSaveButtonCommand()
        {
            // 設定を保存
            Properties.Settings.Default.Save();
        }


        private void OnMesageReceieved(string obj)
        {
            Debug.WriteLine("イベント受け取り", obj);
            IsOpen = true;
        }

        public void OnNavigatedTo(NavigationContext navigationContext)
        {
            IsOpen = true;
            Debug.WriteLine("OnNavigatedTo");
        }

        public bool IsNavigationTarget(NavigationContext navigationContext)
        {
            Debug.WriteLine("IsNavigationTarget");
            //return true;
            return data != null;
        }

        public void OnNavigatedFrom(NavigationContext navigationContext)
        {
            Debug.WriteLine("OnNavigatedFrom");
            IsOpen = true;
        }
    }
}
