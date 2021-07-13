using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for ConfigTabItem
    /// </summary>
    public partial class ConfigTabItem : UserControl
    {
        public ConfigTabItem(ConfigTabItemViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }
    }
}
