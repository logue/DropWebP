using DropWebP.Interfaces;
using DropWebP.Services;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using Reactive.Bindings;
using System.ComponentModel;
using System.Windows;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ステータスバーのViewModel
    /// </summary>
    public class StatusBarViewModel : BindableBase, INotifyPropertyChanged, IStatusBarViewModel
    {
        private IEventAggregator eventAggregator;
        private IRegionManager regionManager;

        /// <summary>
        /// ステータスバーに表示するテキスト
        /// </summary>
        public ReactiveProperty<string> StatusBarText { get; set; } = new ReactiveProperty<string>("Ready.");

        /// <summary>
        /// プログレスバーの表示／非表示
        /// </summary>
        public ReactiveProperty<Visibility> ProgressBarVisibility { get; set; } = new ReactiveProperty<Visibility>(Visibility.Collapsed);

        /// <summary>
        /// プログレスバーの値
        /// </summary>
        public ReactiveProperty<int> Progress { get; set; } = new ReactiveProperty<int>(0);

        /// <summary>
        /// プログレスバーの最大値
        /// </summary>
        public ReactiveProperty<int> ProgressMaximum { get; set; } = new ReactiveProperty<int>(100);

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public StatusBarViewModel(IEventAggregator eventAggregator, IRegionManager regionManager)
        {
            _ = eventAggregator.GetEvent<StatusBarService>().Subscribe(OnUpdateStatusBar);
        }

        /// <summary>
        /// ステータスバーが更新された
        /// </summary>
        /// <param name="obj"></param>
        private void OnUpdateStatusBar(StatusBarEvent e)
        {
            StatusBarText.Value = e.message;
            if (e.progressMaximum != 0)
            {
                ProgressBarVisibility.Value = Visibility.Visible;
                ProgressMaximum.Value = e.progressMaximum;
                Progress.Value = e.progress;
            }
            else
            {
                ProgressBarVisibility.Value = Visibility.Collapsed;
            }
        }
    }
}
