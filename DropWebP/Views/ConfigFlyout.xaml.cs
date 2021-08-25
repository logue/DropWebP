// -----------------------------------------------------------------------
// <copyright file="ConfigFlyout.xaml.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Views
{
    using MahApps.Metro.Controls;
    using System.Windows.Controls;

    /// <summary>
    /// Interaction logic for ConfigFlyout.
    /// </summary>
    public partial class ConfigFlyout : UserControl
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ConfigFlyout"/> class.
        /// </summary>
        public ConfigFlyout()
        {
            InitializeComponent();

            if (LosslessToggleSwitch != null && QualityGroup != null)
            {
                // 可逆圧縮が無効化されているときは品質オプションを操作可能にする
                QualityGroup.IsEnabled = !LosslessToggleSwitch.IsOn;
            }
        }

        /// <summary>
        /// 可逆圧縮トグル.
        /// </summary>
        /// <param name="sender">.</param>
        /// <param name="e">.</param>
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
