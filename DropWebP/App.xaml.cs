// -----------------------------------------------------------------------
// <copyright file="App.xaml.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Activation;
using DropWebP.Contracts.Services;
using DropWebP.Core.Contracts.Services;
using DropWebP.Core.Services;
using DropWebP.Models;
using DropWebP.Services;
using DropWebP.ViewModels;
using DropWebP.Views;

using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.UI.Xaml;

namespace DropWebP;

// To learn more about WinUI 3, see https://docs.microsoft.com/windows/apps/winui/winui3/.

/// <summary>
/// アプリケーションクラス
/// </summary>
public partial class App : Application
{
    // The .NET Generic Host provides dependency injection, configuration, logging, and other services.
    // https://docs.microsoft.com/dotnet/core/extensions/generic-host
    // https://docs.microsoft.com/dotnet/core/extensions/dependency-injection
    // https://docs.microsoft.com/dotnet/core/extensions/configuration
    // https://docs.microsoft.com/dotnet/core/extensions/logging
    public IHost Host
    {
        get;
    }

    /// <summary>
    /// サービス取得
    /// </summary>
    /// <typeparam name="T">サービスのデータ型</typeparam>
    /// <returns>サービス</returns>
    /// <exception cref="ArgumentException">引数例外</exception>
    public static T GetService<T>()
        where T : class
    {
        return (App.Current as App)!.Host.Services.GetService(typeof(T)) is not T service
            ? throw new ArgumentException($"{typeof(T)} needs to be registered in ConfigureServices within App.xaml.cs.")
            : service;
    }

    /// <summary>
    /// メイン画面
    /// </summary>
    public static WindowEx MainWindow { get; } = new MainWindow();

    /// <summary>
    /// Initializes a new instance of the <see cref="App"/> class.
    /// </summary>
    public App()
    {
        InitializeComponent();

        Host = Microsoft.Extensions.Hosting.Host.
        CreateDefaultBuilder().
        UseContentRoot(AppContext.BaseDirectory).
        ConfigureServices((context, services) =>
        {
            // Default Activation Handler
            _ = services.AddTransient<ActivationHandler<LaunchActivatedEventArgs>, DefaultActivationHandler>();

            // Other Activation Handlers

            // Services
            _ = services.AddSingleton<ILocalSettingsService, LocalSettingsService>();
            _ = services.AddSingleton<IThemeSelectorService, ThemeSelectorService>();
            _ = services.AddTransient<INavigationViewService, NavigationViewService>();

            _ = services.AddSingleton<IActivationService, ActivationService>();
            _ = services.AddSingleton<IPageService, PageService>();
            _ = services.AddSingleton<INavigationService, NavigationService>();

            // Core Services
            _ = services.AddSingleton<IFileService, FileService>();

            // Views and ViewModels
            _ = services.AddTransient<SettingsViewModel>();
            _ = services.AddTransient<SettingsPage>();
            _ = services.AddTransient<MainViewModel>();
            _ = services.AddTransient<MainPage>();
            _ = services.AddTransient<ShellPage>();
            _ = services.AddTransient<ShellViewModel>();

            // Configuration
            _ = services.Configure<LocalSettingsOptions>(context.Configuration.GetSection(nameof(LocalSettingsOptions)));
        }).
        Build();

        UnhandledException += App_UnhandledException;
    }

    /// <inheritdoc/>
    protected override async void OnLaunched(LaunchActivatedEventArgs args)
    {
        base.OnLaunched(args);

        await App.GetService<IActivationService>().ActivateAsync(args);
    }

    /// <summary>
    /// 例外発生時
    /// </summary>
    /// <param name="sender">呼び出し元</param>
    /// <param name="e">例外の引数</param>
    private void App_UnhandledException(object sender, Microsoft.UI.Xaml.UnhandledExceptionEventArgs e)
    {
        // TODO: Log and handle exceptions as appropriate.
        // https://docs.microsoft.com/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.application.unhandledexception.
    }
}
