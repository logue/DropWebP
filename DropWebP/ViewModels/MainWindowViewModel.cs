using DropWebP.Views;
using Prism.Mvvm;
using Prism.Regions;

namespace DropWebP.ViewModels
{
    public class MainWindowViewModel : BindableBase
    {
        /// <summary>
        /// タイトル
        /// </summary>
        private string _title = "Prism Application";
        public string Title
        {
            get { return _title; }
            set { SetProperty(ref _title, value); }
        }
        /// <summary>コンストラクタ</summary>
        /// <param name="regionManager">インジェクションするIRegionManager。</param>
        public MainWindowViewModel(IRegionManager regionManager)
        {
            regionManager.RegisterViewWithRegion("ContentRegion", typeof(DragNDropControl));
            regionManager.RegisterViewWithRegion("StatusBarRegion", typeof(StatusBar));
        }
    }
}
