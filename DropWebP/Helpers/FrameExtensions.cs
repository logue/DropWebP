// -----------------------------------------------------------------------
// <copyright file="FrameExtensions.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml.Controls;

namespace DropWebP.Helpers;

/// <summary>
/// フレーム拡張クラス
/// </summary>
public static class FrameExtensions
{
    /// <summary>
    /// ページのビューモデルを取得
    /// </summary>
    /// <param name="frame">フレーム</param>
    /// <returns>オブジェクト</returns>
    public static object? GetPageViewModel(this Frame frame) => frame?.Content?.GetType().GetProperty("ViewModel")?.GetValue(frame.Content, null);
}
