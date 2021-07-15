using DropWebP.Interfaces;
using DropWebP.Utility;
using Prism.Mvvm;
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
        public StatusBarViewModel()
        {
            Messenger.Default.Register<UpdateStatusBar>(this, OnUpdateStatusBar);
        }

        /// <summary>
        /// ステータスバーが更新された
        /// </summary>
        /// <param name="obj"></param>
        private void OnUpdateStatusBar(UpdateStatusBar obj)
        {
            StatusBarText.Value = obj.Text;
        }
    }
}
