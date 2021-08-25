﻿// -----------------------------------------------------------------------
// <copyright file="IInitializeWithWindow.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Interfaces
{
    using System;
    using System.Runtime.InteropServices;

    /// <summary>
    /// WPFからMessageDialogを呼ぶ場合のおまじない
    /// https://qiita.com/okazuki/items/227f8d19e38a67099006.
    /// </summary>
    [ComImport]
    [Guid("3E68D4BD-7135-4D10-8018-9FB6D9F33FA1")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IInitializeWithWindow
    {
        /// <summary>
        /// The Initialize.
        /// </summary>
        /// <param name="hwnd">The hwnd<see cref="IntPtr"/>.</param>
        void Initialize([In] IntPtr hwnd);
    }

    /// <summary>
    /// Defines the <see cref="IWindowNative" />.
    /// </summary>
    [ComImport]
    [Guid("EECDBF0E-BAE9-4CB6-A68E-9598E1CB57BB")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IWindowNative
    {
        /// <summary>
        /// Gets the WindowHandle.
        /// </summary>
        IntPtr WindowHandle { get; }
    }
}
