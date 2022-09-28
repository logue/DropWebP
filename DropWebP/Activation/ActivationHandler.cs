// -----------------------------------------------------------------------
// <copyright file="ActivationHandler.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Activation;

/// <summary>
/// Extend this class to implement new ActivationHandlers. See DefaultActivationHandler for an example.
/// https://github.com/microsoft/TemplateStudio/blob/main/docs/WinUI/activation.md
/// </summary>
/// <typeparam name="T">データ型</typeparam>
public abstract class ActivationHandler<T> : IActivationHandler
    where T : class
{
    /// <inheritdoc/>
    public bool CanHandle(object args) => args is T && CanHandleInternal((args as T)!);

    /// <inheritdoc/>
    public async Task HandleAsync(object args) => await HandleInternalAsync((args as T)!);

    /// <summary>
    /// Override this method to add the logic for whether to handle the activation.
    /// </summary>
    /// <param name="args">引数</param>
    /// <returns>成否</returns>
    protected virtual bool CanHandleInternal(T args) => true;

    /// <summary>
    /// Override this method to add the logic for your activation handler.
    /// </summary>
    /// <param name="args"></param>
    /// <returns></returns>
    protected abstract Task HandleInternalAsync(T args);
}
