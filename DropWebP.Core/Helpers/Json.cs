// -----------------------------------------------------------------------
// <copyright file="Json.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Newtonsoft.Json;

namespace DropWebP.Core.Helpers;

/// <summary>
/// JSONクラス
/// </summary>
public static class Json
{
    /// <summary>
    /// JSONを読み込む
    /// </summary>
    /// <typeparam name="T">データ型</typeparam>
    /// <param name="value">値</param>
    /// <returns>JSONオブジェクトの内容</returns>
    public static async Task<T> ToObjectAsync<T>(string value)
    {
        return await Task.Run<T>(() =>
        {
            return JsonConvert.DeserializeObject<T>(value);
        });
    }

    /// <summary>
    /// JSONを書き出し
    /// </summary>
    /// <param name="value">入力データのオブジェクト</param>
    /// <returns>シリアライズ化されたデータ</returns>
    public static async Task<string> StringifyAsync(object value)
    {
        return await Task.Run<string>(() =>
        {
            return JsonConvert.SerializeObject(value);
        });
    }
}
