// -----------------------------------------------------------------------
// <copyright file="RuntimeHelper.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System.Runtime.InteropServices;
using System.Text;

namespace DropWebP.Helpers;

/// <summary>
/// ランタイムへルパ
/// </summary>
public class RuntimeHelper
{
    /// <summary>
    /// 現在のパッケージのフルネームを取得
    /// </summary>
    /// <param name="packageFullNameLength">パッケージのフルネームの長さ</param>
    /// <param name="packageFullName">パッケージのフルネーム</param>
    /// <returns></returns>
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern int GetCurrentPackageFullName(ref int packageFullNameLength, StringBuilder? packageFullName);

    /// <summary>
    /// MSIXか
    /// </summary>
    public static bool IsMSIX
    {
        get
        {
            int length = 0;

            return GetCurrentPackageFullName(ref length, null) != 15700L;
        }
    }
}
