using DropWebP.Utility;
using Prism.Mvvm;
using Reactive.Bindings;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Windows;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ステータスバーのViewModel
    /// </summary>
    public class StatusBarViewModel : BindableBase, INotifyPropertyChanged
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
        public string StatusBarText { get; private set; }

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
            StatusBarText = obj.Text;
        }
        /*
        [NotifyPropertyChangedInvocator]
        protected virtual void OnPropertyChanged([CallerMemberName] string propertyName = null)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
        */
    }
}
