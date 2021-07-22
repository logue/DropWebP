using MahApps.Metro.Controls;
using System.Windows.Controls;

namespace DropWebP.Views
{
    /// <summary>
    /// Interaction logic for ConfigTabItem
    /// </summary>
    public partial class ConfigFlyout : UserControl
    {
        /// <summary>
        /// コンストラクタ
        /// </summary>
        public ConfigFlyout()
        {
            InitializeComponent();
        }

        /// <summary>
        /// スライダーが変化したときに値を表示されている値を変更する
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="e"></param>
        private void QualitySlider_ValueChanged(object sender, System.Windows.RoutedPropertyChangedEventArgs<double> e)
        {
            if (QualityGroup != null && QualityGroup.IsEnabled)
            {
                QualityValueText.Text = QualitySlider.Value.ToString();
            }
        }

        /// <summary>
        /// 可逆圧縮トグル
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="e"></param>
        private void LosslessToggleSwitch_Toggled(object sender, System.Windows.RoutedEventArgs e)
        {
            ToggleSwitch toggleSwitch = sender as ToggleSwitch;

            if (toggleSwitch != null && QualityGroup != null)
            {
                // 可逆圧縮が無効化されているときは品質オプションを操作可能にする
                QualityGroup.IsEnabled = !toggleSwitch.IsOn;
            }
        }
    }
}
