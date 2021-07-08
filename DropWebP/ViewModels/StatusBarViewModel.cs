using Prism.Mvvm;
using Reactive.Bindings;
using System.Windows;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ステータスバーのViewModel
    /// </summary>
    public class StatusBarViewModel : BindableBase
    {
        /// <summary>
        /// ステータスバーに表示するテキスト
        /// </summary>
        public ReactiveProperty<string> statusBarText { get; set; } = new ReactiveProperty<string>("Ready.");
        /// <summary>
        /// プログレスバーの表示／非表示
        /// </summary>
        public ReactiveProperty<Visibility> progressBarVisibility { get; set; } = new ReactiveProperty<Visibility>(Visibility.Collapsed);
        /// <summary>
        /// プログレスバーの値
        /// </summary>
        public ReactiveProperty<int> progress { get; set; } = new ReactiveProperty<int>(0);
        /// <summary>
        /// プログレスバーの最大値
        /// </summary>
        public ReactiveProperty<int> progressMaximum { get; set; } = new ReactiveProperty<int>(100);
        /// <summary>
        /// コンストラクタ
        /// </summary>
        public StatusBarViewModel()
        {
        }
    }
}
