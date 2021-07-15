using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for HomeTabItem
    /// </summary>
    public partial class HomeTabItem : UserControl
    {
        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="vm">ビューモデル</param>
        public HomeTabItem(HomeTabItemlViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }
    }
}
