// -----------------------------------------------------------------------
// <copyright file="MetroDialogService.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using MahApps.Metro.Controls;
using Prism.Services.Dialogs;

namespace DropWebP.Services
{
    /// <summary>
    /// インラインダイアログのラッパー.
    /// </summary>
    internal class MetroDialogService : MetroWindow, IDialogWindow
    {
        /// <summary>
        /// Gets or sets the Result.
        /// </summary>
        public IDialogResult Result { get; set; }
    }
}