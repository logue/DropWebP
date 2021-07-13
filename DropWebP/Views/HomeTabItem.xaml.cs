using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for HomeTabItem
    /// </summary>
    public partial class HomeTabItem : UserControl
    {
        public HomeTabItem(HomeTabItemlViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }
    }
}
