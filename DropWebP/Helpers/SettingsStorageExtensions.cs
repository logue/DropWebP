// -----------------------------------------------------------------------
// <copyright file="SettingsStorageExtensions.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Core.Helpers;

using Windows.Storage;
using Windows.Storage.Streams;

namespace DropWebP.Helpers;

// Use these extension methods to store and retrieve local and roaming app data
// More details regarding storing and retrieving app data at https://docs.microsoft.com/windows/apps/design/app-settings/store-and-retrieve-app-data

/// <summary>
/// 設定ストレージ機能拡張
/// </summary>
public static class SettingsStorageExtensions
{
    /// <summary>
    /// 設定ファイルの拡張子
    /// </summary>
    private const string FileExtension = ".json";

    /// <summary>
    /// 設定ファイルを書き込める状態か
    /// </summary>
    /// <param name="appData">データ</param>
    /// <returns>可否</returns>
    public static bool IsRoamingStorageAvailable(this ApplicationData appData)
    {
        return appData.RoamingStorageQuota == 0;
    }

    /// <summary>
    /// 設定を保存
    /// </summary>
    /// <typeparam name="T">設定の型</typeparam>
    /// <param name="folder">保存先フォルダ</param>
    /// <param name="name">ファイル名</param>
    /// <param name="content">内容</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    public static async Task SaveAsync<T>(this StorageFolder folder, string name, T content)
    {
        StorageFile file = await folder.CreateFileAsync(GetFileName(name), CreationCollisionOption.ReplaceExisting);
        string fileContent = await Json.StringifyAsync(content);

        await FileIO.WriteTextAsync(file, fileContent);
    }

    /// <summary>
    /// 設定を取得
    /// </summary>
    /// <typeparam name="T">設定の型</typeparam>
    /// <param name="folder">保存先フォルダ</param>
    /// <param name="name">ファイル名</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    public static async Task<T?> ReadAsync<T>(this StorageFolder folder, string name)
    {
        if (!File.Exists(Path.Combine(folder.Path, GetFileName(name))))
        {
            return default;
        }

        StorageFile file = await folder.GetFileAsync($"{name}.json");
        string fileContent = await FileIO.ReadTextAsync(file);

        return await Json.ToObjectAsync<T>(fileContent);
    }

    /// <summary>
    /// 設定を保存
    /// </summary>
    /// <typeparam name="T">型</typeparam>
    /// <param name="settings">設定名</param>
    /// <param name="key">キー</param>
    /// <param name="value">値</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    public static async Task SaveAsync<T>(this ApplicationDataContainer settings, string key, T value)
    {
        settings.SaveString(key, await Json.StringifyAsync(value));
    }

    /// <summary>
    /// テキストで保存
    /// </summary>
    /// <param name="settings">設定</param>
    /// <param name="key">設定キー</param>
    /// <param name="value">値</param>
    public static void SaveString(this ApplicationDataContainer settings, string key, string value)
    {
        settings.Values[key] = value;
    }

    /// <summary>
    /// 設定を読み込む
    /// </summary>
    /// <typeparam name="T">データ型</typeparam>
    /// <param name="settings">設定</param>
    /// <param name="key">キー</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    public static async Task<T?> ReadAsync<T>(this ApplicationDataContainer settings, string key)
    {
        return settings.Values.TryGetValue(key, out object? obj) ? await Json.ToObjectAsync<T>((string)obj) : default;
    }

    /// <summary>
    /// 設定を保存
    /// </summary>
    /// <param name="folder">保存先</param>
    /// <param name="content">内容</param>
    /// <param name="fileName">ファイル名</param>
    /// <param name="options">オプション</param>
    /// <returns>A <see cref="Task{TResult}"/> representing the result of the asynchronous operation.</returns>
    /// <exception cref="ArgumentNullException">引数がNull</exception>
    /// <exception cref="ArgumentException">無効な引数</exception>
    public static async Task<StorageFile> SaveFileAsync(this StorageFolder folder, byte[] content, string fileName, CreationCollisionOption options = CreationCollisionOption.ReplaceExisting)
    {
        if (content == null)
        {
            throw new ArgumentNullException(nameof(content));
        }

        if (string.IsNullOrEmpty(fileName))
        {
            throw new ArgumentException("File name is null or empty. Specify a valid file name", nameof(fileName));
        }

        StorageFile storageFile = await folder.CreateFileAsync(fileName, options);
        await FileIO.WriteBytesAsync(storageFile, content);
        return storageFile;
    }

    /// <summary>
    /// 設定ファイルを取得
    /// </summary>
    /// <param name="folder">フォルダ</param>
    /// <param name="fileName">ファイル名</param>
    /// <returns>A <see cref="Task{TResult}"/> representing the result of the asynchronous operation.</returns>
    public static async Task<byte[]?> ReadFileAsync(this StorageFolder folder, string fileName)
    {
        IStorageItem item = await folder.TryGetItemAsync(fileName).AsTask().ConfigureAwait(false);

        if ((item != null) && item.IsOfType(StorageItemTypes.File))
        {
            StorageFile storageFile = await folder.GetFileAsync(fileName);
            byte[]? content = await storageFile.ReadBytesAsync();
            return content;
        }

        return null;
    }

    /// <summary>
    /// バイトでファイルから読み込む
    /// </summary>
    /// <param name="file">ファイル</param>
    /// <returns>A <see cref="Task{TResult}"/> representing the result of the asynchronous operation.</returns>
    public static async Task<byte[]?> ReadBytesAsync(this StorageFile file)
    {
        if (file != null)
        {
            using IRandomAccessStream stream = await file.OpenReadAsync();
            using DataReader reader = new (stream.GetInputStreamAt(0));
            _ = await reader.LoadAsync((uint)stream.Size);
            byte[] bytes = new byte[stream.Size];
            reader.ReadBytes(bytes);
            return bytes;
        }

        return null;
    }

    /// <summary>
    /// ファイル名を取得
    /// </summary>
    /// <param name="name">設定名</param>
    /// <returns>ファイル名</returns>
    private static string GetFileName(string name)
    {
        return string.Concat(name, FileExtension);
    }
}
