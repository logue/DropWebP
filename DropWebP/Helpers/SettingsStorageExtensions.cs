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
    /// <returns>実行結果</returns>
    public static async Task SaveAsync<T>(this StorageFolder folder, string name, T content)
    {
        var file = await folder.CreateFileAsync(GetFileName(name), CreationCollisionOption.ReplaceExisting);
        var fileContent = await Json.StringifyAsync(content);

        await FileIO.WriteTextAsync(file, fileContent);
    }

    /// <summary>
    /// 設定を取得
    /// </summary>
    /// <typeparam name="T">設定の型</typeparam>
    /// <param name="folder">保存先フォルダ</param>
    /// <param name="name">ファイル名</param>
    /// <returns>実行結果</returns>
    public static async Task<T?> ReadAsync<T>(this StorageFolder folder, string name)
    {
        if (!File.Exists(Path.Combine(folder.Path, GetFileName(name))))
        {
            return default;
        }

        var file = await folder.GetFileAsync($"{name}.json");
        var fileContent = await FileIO.ReadTextAsync(file);

        return await Json.ToObjectAsync<T>(fileContent);
    }

    /// <summary>
    /// 
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <param name="settings"></param>
    /// <param name="key"></param>
    /// <param name="value"></param>
    /// <returns></returns>
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
    /// <returns>値</returns>
    public static async Task<T?> ReadAsync<T>(this ApplicationDataContainer settings, string key)
    {
        object? obj;

        if (settings.Values.TryGetValue(key, out obj))
        {
            return await Json.ToObjectAsync<T>((string)obj);
        }

        return default;
    }

    /// <summary>
    /// 
    /// </summary>
    /// <param name="folder"></param>
    /// <param name="content"></param>
    /// <param name="fileName"></param>
    /// <param name="options"></param>
    /// <returns></returns>
    /// <exception cref="ArgumentNullException"></exception>
    /// <exception cref="ArgumentException"></exception>
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

        var storageFile = await folder.CreateFileAsync(fileName, options);
        await FileIO.WriteBytesAsync(storageFile, content);
        return storageFile;
    }

    /// <summary>
    /// 
    /// </summary>
    /// <param name="folder"></param>
    /// <param name="fileName"></param>
    /// <returns></returns>
    public static async Task<byte[]?> ReadFileAsync(this StorageFolder folder, string fileName)
    {
        var item = await folder.TryGetItemAsync(fileName).AsTask().ConfigureAwait(false);

        if ((item != null) && item.IsOfType(StorageItemTypes.File))
        {
            var storageFile = await folder.GetFileAsync(fileName);
            var content = await storageFile.ReadBytesAsync();
            return content;
        }

        return null;
    }

    /// <summary>
    /// バイトでファイルから読み込む
    /// </summary>
    /// <param name="file">ファイル</param>
    /// <returns>バイト</returns>
    public static async Task<byte[]?> ReadBytesAsync(this StorageFile file)
    {
        if (file != null)
        {
            using IRandomAccessStream stream = await file.OpenReadAsync();
            using var reader = new DataReader(stream.GetInputStreamAt(0));
            await reader.LoadAsync((uint)stream.Size);
            var bytes = new byte[stream.Size];
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
