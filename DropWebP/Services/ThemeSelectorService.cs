// -----------------------------------------------------------------------
// <copyright file="ThemeSelectorService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Contracts.Services;
using DropWebP.Helpers;

using Microsoft.UI.Xaml;

namespace DropWebP.Services;

/// <summary>
/// テーマ選択サービス
/// </summary>
public class ThemeSelectorService : IThemeSelectorService
{
    /// <summary>
    /// 設定名
    /// </summary>
    private const string SettingsKey = "AppBackgroundRequestedTheme";

    /// <inheritdoc/>
    public ElementTheme Theme { get; set; } = ElementTheme.Default;

    private readonly ILocalSettingsService localSettingsService;

    /// <summary>
    /// Initializes a new instance of the <see cref="ThemeSelectorService"/> class.
    /// </summary>
    /// <param name="localSettingsService">ローカル設定サービス</param>
    public ThemeSelectorService(ILocalSettingsService localSettingsService)
    {
        this.localSettingsService = localSettingsService;
    }

    /// <inheritdoc/>
    public async Task InitializeAsync()
    {
        Theme = await LoadThemeFromSettingsAsync();
        await Task.CompletedTask;
    }

    /// <inheritdoc/>
    public async Task SetThemeAsync(ElementTheme theme)
    {
        Theme = theme;

        await SetRequestedThemeAsync();
        await SaveThemeInSettingsAsync(Theme);
    }

    /// <inheritdoc/>
    public async Task SetRequestedThemeAsync()
    {
        if (App.MainWindow.Content is FrameworkElement rootElement)
        {
            rootElement.RequestedTheme = Theme;

            TitleBarHelper.UpdateTitleBar(Theme);
        }

        await Task.CompletedTask;
    }

    /// <summary>
    /// テーマ設定を読み込む
    /// </summary>
    /// <returns>テーマ設定</returns>
    private async Task<ElementTheme> LoadThemeFromSettingsAsync()
    {
        var themeName = await localSettingsService.ReadSettingAsync<string>(SettingsKey);

        if (Enum.TryParse(themeName, out ElementTheme cacheTheme))
        {
            return cacheTheme;
        }

        return ElementTheme.Default;
    }

    /// <summary>
    /// テーマ設定を保存
    /// </summary>
    /// <param name="theme">保存するテーマ設定</param>
    /// <returns>実行結果</returns>
    private async Task SaveThemeInSettingsAsync(ElementTheme theme)
    {
        await localSettingsService.SaveSettingAsync(SettingsKey, theme.ToString());
    }
}
