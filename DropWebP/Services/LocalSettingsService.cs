// -----------------------------------------------------------------------
// <copyright file="LocalSettingsService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Contracts.Services;
using DropWebP.Core.Contracts.Services;
using DropWebP.Core.Helpers;
using DropWebP.Helpers;
using DropWebP.Models;

using Microsoft.Extensions.Options;
using Windows.Storage;

namespace DropWebP.Services;

/// <summary>
/// ローカル設定サービス
/// </summary>
public class LocalSettingsService : ILocalSettingsService
{
    private const string DefaultApplicationDataFolder = "DropWebP/ApplicationData";
    private const string DefaultLocalSettingsFile = "LocalSettings.json";

    private readonly IFileService fileService;
    private readonly LocalSettingsOptions options;

    private readonly string localApplicationData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
    private readonly string applicationDataFolder;
    private readonly string localsettingsFile;

    private IDictionary<string, object> settings;

    private bool isInitialized;

    /// <summary>
    /// Initializes a new instance of the <see cref="LocalSettingsService"/> class.
    /// </summary>
    /// <param name="fileService">ファイルサービス</param>
    /// <param name="options">オプション</param>
    public LocalSettingsService(IFileService fileService, IOptions<LocalSettingsOptions> options)
    {
        this.fileService = fileService;
        this.options = options.Value;

        applicationDataFolder = Path.Combine(localApplicationData, this.options.ApplicationDataFolder ?? DefaultApplicationDataFolder);
        localsettingsFile = this.options.LocalSettingsFile ?? DefaultLocalSettingsFile;

        settings = new Dictionary<string, object>();
    }

    /// <inheritdoc/>
    public async Task<T?> ReadSettingAsync<T>(string key)
    {
        if (RuntimeHelper.IsMSIX)
        {
            if (ApplicationData.Current.LocalSettings.Values.TryGetValue(key, out var obj))
            {
                return await Json.ToObjectAsync<T>((string)obj);
            }
        }
        else
        {
            await InitializeAsync();

            if (settings != null && settings.TryGetValue(key, out var obj))
            {
                return await Json.ToObjectAsync<T>((string)obj);
            }
        }

        return default;
    }

    /// <inheritdoc/>
    public async Task SaveSettingAsync<T>(string key, T value)
    {
        if (RuntimeHelper.IsMSIX)
        {
            ApplicationData.Current.LocalSettings.Values[key] = await Json.StringifyAsync(value);
        }
        else
        {
            await InitializeAsync();

            settings[key] = await Json.StringifyAsync(value);

            await Task.Run(() => fileService.Save(applicationDataFolder, localsettingsFile, settings));
        }
    }

    /// <summary>
    /// ハンドラーを初期化し、オプションを解決して検証します。
    /// </summary>
    /// <returns>タスク</returns>
    private async Task InitializeAsync()
    {
        if (!isInitialized)
        {
            settings = await Task.Run(() => fileService.Read<IDictionary<string, object>>(applicationDataFolder, localsettingsFile)) ?? new Dictionary<string, object>();

            isInitialized = true;
        }
    }
}
