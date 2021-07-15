using DropWebP.ViewModels;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for ConfigTabItem
    /// </summary>
    public partial class ConfigTabItem : UserControl
    {
        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="vm">ビューモデル</param>
        public ConfigTabItem(ConfigTabItemViewModel vm)
        {
            InitializeComponent();
            DataContext = vm;
        }

        private void QualitySlider_ValueChanged(object sender, System.Windows.RoutedPropertyChangedEventArgs<double> e)
        {
            if (QualityGroup.IsEnabled)
            {
                // QualityValueText.Text = QualitySlider.Value.ToString();
            }

        }

        private void LosslessCheckBox_Checked(object sender, System.Windows.RoutedEventArgs e)
        {
            // QualityGroup.IsEnabled = false;

        }

        private void LosslessCheckBox_Unchecked(object sender, System.Windows.RoutedEventArgs e)
        {
            // QualityGroup.IsEnabled = true;
        }
    }
}
