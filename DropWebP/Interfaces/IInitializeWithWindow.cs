// -----------------------------------------------------------------------
// <copyright file="IInitializeWithWindow.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System;
using System.Runtime.InteropServices;

namespace DropWebP.Interfaces;

/// <summary>
///     WPFからMessageDialogを呼ぶ場合のおまじない
///     https://qiita.com/okazuki/items/227f8d19e38a67099006.
/// </summary>
[ComImport]
[Guid("3E68D4BD-7135-4D10-8018-9FB6D9F33FA1")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IInitializeWithWindow
{
    /// <summary>
    ///     The Initialize.
    /// </summary>
    /// <param name="hWnd">The hWnd<see cref="IntPtr" />.</param>
    void Initialize([In] IntPtr hWnd);
}
