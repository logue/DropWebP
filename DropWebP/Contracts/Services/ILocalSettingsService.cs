// -----------------------------------------------------------------------
// <copyright file="ILocalSettingsService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Contracts.Services;

/// <summary>
/// 設定サービスのインターフェース
/// </summary>
public interface ILocalSettingsService
{
    /// <summary>
    /// 設定を読み込む（非同期）
    /// </summary>
    /// <typeparam name="T">設定の型</typeparam>
    /// <param name="key">設定のキー名</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task<T?> ReadSettingAsync<T>(string key);

    /// <summary>
    /// 設定を保存（非同期）
    /// </summary>
    /// <typeparam name="T">設定の型</typeparam>
    /// <param name="key">保存する設定のキー</param>
    /// <param name="value">保存する設定の値</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task SaveSettingAsync<T>(string key, T value);
}
