// -----------------------------------------------------------------------
// <copyright file="SettingsPage.xaml.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.ViewModels;

using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Views;

// TODO: Set the URL for your privacy policy by updating SettingsPage_PrivacyTermsLink.NavigateUri in Resources.resw.
/// <summary>
/// 設定ページ
/// </summary>
public sealed partial class SettingsPage : Page
{
    /// <summary>
    /// 設定ページのビューモデル
    /// </summary>
    public SettingsViewModel ViewModel
    {
        get;
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="SettingsPage"/> class.
    /// </summary>
    public SettingsPage()
    {
        ViewModel = App.GetService<SettingsViewModel>();
        InitializeComponent();
    }
}
