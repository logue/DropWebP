// -----------------------------------------------------------------------
// <copyright file="IActivationHandler.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Activation;

/// <summary>
/// アクティベーションのハンドラのインターフェース
/// </summary>
public interface IActivationHandler
{
    /// <summary>
    /// ハンドル可能か
    /// </summary>
    /// <param name="args">引数</param>
    /// <returns>可否</returns>
    bool CanHandle(object args);

    /// <summary>
    /// ハンドル実行
    /// </summary>
    /// <param name="args">引数</param>
    /// <returns>実行結果</returns>
    Task HandleAsync(object args);
}
