using Prism.Mvvm;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// アバウトタブのビューモデル
    /// </summary>
    public class AboutTabItemViewModel : BindableBase
    {
        /// <summary>
        /// タブ名
        /// </summary>
        public string Name { get; set; } = "About";

        public AboutTabItemViewModel()
        {

        }
    }
}
