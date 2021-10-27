// -----------------------------------------------------------------------
// <copyright file="ILoggerService.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.Extensions.Logging;
using System;

namespace DropWebP.Interfaces
{
    /// <summary>
    /// Defines the <see cref="ILoggerService" />.
    /// </summary>
    public interface ILoggerService
    {
        /// <summary>
        /// The Log.
        /// </summary>
        /// <param name="category">The category<see cref="LogLevel"/>.</param>
        /// <param name="message">The message<see cref="string"/>.</param>
        void Log(LogLevel category, string message);

        /// <summary>
        /// The Log.
        /// </summary>
        /// <param name="category">The category<see cref="LogLevel"/>.</param>
        /// <param name="message">The message<see cref="string"/>.</param>
        /// <param name="ex">The ex<see cref="Exception"/>.</param>
        void Log(LogLevel category, string message, Exception ex);
    }
}