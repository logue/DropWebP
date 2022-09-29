// -----------------------------------------------------------------------
// <copyright file="SettingsViewModel.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System.Reflection;
using System.Windows.Input;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using DropWebP.Contracts.Services;
using DropWebP.Helpers;
using Microsoft.UI.Xaml;
using Windows.ApplicationModel;

namespace DropWebP.ViewModels;

/// <summary>
/// 設定ビューモデル
/// </summary>
public class SettingsViewModel : ObservableRecipient
{
    /// <summary>
    /// テーマ選択サービス
    /// </summary>
    private readonly IThemeSelectorService themeSelectorService;

    /// <summary>
    /// テーマ
    /// </summary>
    private ElementTheme elementTheme;

    /// <summary>
    /// バージョン情報
    /// </summary>
    private string versionDescription;

    /// <summary>
    /// テーマ
    /// </summary>
    public ElementTheme ElementTheme
    {
        get => elementTheme;
        set => SetProperty(ref elementTheme, value);
    }

    /// <summary>
    /// バージョン情報
    /// </summary>
    public string VersionDescription
    {
        get => versionDescription;
        set => SetProperty(ref versionDescription, value);
    }

    /// <summary>
    /// テーマスイッチコマンド
    /// </summary>
    public ICommand SwitchThemeCommand
    {
        get;
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="SettingsViewModel"/> class.
    /// </summary>
    /// <param name="themeSelectorService">テーマ選択サービス</param>
    public SettingsViewModel(IThemeSelectorService themeSelectorService)
    {
        this.themeSelectorService = themeSelectorService;
        elementTheme = this.themeSelectorService.Theme;
        versionDescription = GetVersionDescription();

        SwitchThemeCommand = new RelayCommand<ElementTheme>(
            async (param) =>
            {
                if (ElementTheme != param)
                {
                    ElementTheme = param;
                    await this.themeSelectorService.SetThemeAsync(param);
                }
            });
    }

    /// <summary>
    /// バージョン情報
    /// </summary>
    /// <returns>アプリケーション名とバージョン</returns>
    private static string GetVersionDescription()
    {
        Version version;

        if (RuntimeHelper.IsMSIX)
        {
            PackageVersion packageVersion = Package.Current.Id.Version;

            version = new (packageVersion.Major, packageVersion.Minor, packageVersion.Build, packageVersion.Revision);
        }
        else
        {
            version = Assembly.GetExecutingAssembly().GetName().Version!;
        }

        return $"{"AppDisplayName".GetLocalized()} - {version.Major}.{version.Minor}.{version.Build}.{version.Revision}";
    }
}
