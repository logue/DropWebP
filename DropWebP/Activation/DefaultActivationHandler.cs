// -----------------------------------------------------------------------
// <copyright file="DefaultActivationHandler.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Contracts.Services;
using DropWebP.ViewModels;

using Microsoft.UI.Xaml;

namespace DropWebP.Activation;

/// <summary>
/// デフォルトの有効化ハンドラ
/// </summary>
public class DefaultActivationHandler : ActivationHandler<LaunchActivatedEventArgs>
{
    /// <summary>
    /// ナビゲーションサービス
    /// </summary>
    private readonly INavigationService navigationService;

    /// <summary>
    /// Initializes a new instance of the <see cref="DefaultActivationHandler"/> class.
    /// </summary>
    /// <param name="navigationService">ナビゲーションサービス</param>
    public DefaultActivationHandler(INavigationService navigationService)
    {
        this.navigationService = navigationService;
    }

    /// <inheritdoc/>
    protected override bool CanHandleInternal(LaunchActivatedEventArgs args)
    {
        // None of the ActivationHandlers has handled the activation.
        return navigationService.Frame?.Content == null;
    }

    /// <inheritdoc/>
    protected override async Task HandleInternalAsync(LaunchActivatedEventArgs args)
    {
        _ = navigationService.NavigateTo(typeof(MainViewModel).FullName!, args.Arguments);

        await Task.CompletedTask;
    }
}
