// -----------------------------------------------------------------------
// <copyright file="IActivationService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Contracts.Services;

/// <summary>
/// アクティベーションサービスのインターフェース
/// </summary>
public interface IActivationService
{
    /// <summary>
    /// アクティベート時の処理
    /// </summary>
    /// <param name="activationArgs">アクティベート時の引数</param>
    /// <returns>A <see cref="Task"/> representing the result of the asynchronous operation.</returns>
    Task ActivateAsync(object activationArgs);
}
