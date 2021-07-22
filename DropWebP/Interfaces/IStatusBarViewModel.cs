using Reactive.Bindings;
using System.Windows;

namespace DropWebP.Interfaces
{
    public interface IStatusBarViewModel
    {
        /// <summary>
        /// ステータスバーに表示するテキスト
        /// </summary>
        public ReactiveProperty<string> StatusBarText { get; set; }

        /// <summary>
        /// プログレスバーの表示／非表示
        /// </summary>
        public ReactiveProperty<Visibility> ProgressBarVisibility { get; set; }

        /// <summary>
        /// プログレスバーの値
        /// </summary>
        public ReactiveProperty<int> Progress { get; set; }

        /// <summary>
        /// プログレスバーの最大値
        /// </summary>
        public ReactiveProperty<int> ProgressMaximum { get; set; }
    }
}
