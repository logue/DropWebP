// -----------------------------------------------------------------------
// <copyright file="ConfigFlyout.xaml.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------
using System.Windows.Controls;
using MahApps.Metro.Controls;

namespace DropWebP.Views
{
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
            if (sender is ToggleSwitch toggleSwitch && QualityGroup != null)
            {
                // 可逆圧縮が無効化されているときは品質オプションを操作可能にする
                QualityGroup.IsEnabled = !toggleSwitch.IsOn;
            }
        }
    }
}