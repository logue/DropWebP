using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for AboutTabItem
    /// </summary>
    public partial class AboutTabItem : UserControl
    {
        public AboutTabItem(AboutTabItemViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }
    }
}
