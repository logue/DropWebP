using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for AboutTabItem
    /// </summary>
    public partial class AboutDialog : UserControl
    {
        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="vm">ビューモデル</param>
        public AboutDialog(AboutDialogViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }
    }
}
