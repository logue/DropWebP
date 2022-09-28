// -----------------------------------------------------------------------
// <copyright file="MainPage.xaml.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.ViewModels;

using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Views;

/// <summary>
/// メインページ
/// </summary>
public sealed partial class MainPage : Page
{
    /// <summary>
    /// メインページのビューモデル
    /// </summary>
    public MainViewModel ViewModel
    {
        get;
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="MainPage"/> class.
    /// </summary>
    public MainPage()
    {
        ViewModel = App.GetService<MainViewModel>();
        InitializeComponent();
    }
}
