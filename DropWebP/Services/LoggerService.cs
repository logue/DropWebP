// -----------------------------------------------------------------------
// <copyright file="LoggerService.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Services
{
    using DropWebP.Interfaces;
    using Microsoft.Extensions.Logging;
    using System;

    /// <summary>
    /// ロガーサービス.
    /// </summary>
    internal class LoggerService : ILoggerService
    {
        /// <summary>
        /// Defines the Logger.
        /// </summary>
        private readonly ILogger logger;

        /// <summary>
        /// Initializes a new instance of the <see cref="LoggerService"/> class.
        /// </summary>
        /// <param name="logger">The logger<see cref="ILogger"/>.</param>
        public LoggerService(ILogger logger)
        {
            this.logger = logger;
        }

        /// <summary>
        /// The Log.
        /// </summary>
        /// <param name="category">The category<see cref="LogLevel"/>.</param>
        /// <param name="message">The message<see cref="string"/>.</param>
        public void Log(LogLevel category, string message)
        {
            logger.Log(category, message);
        }

        /// <summary>
        /// The Log.
        /// </summary>
        /// <param name="category">The category<see cref="LogLevel"/>.</param>
        /// <param name="message">The message<see cref="string"/>.</param>
        /// <param name="ex">The ex<see cref="Exception"/>.</param>
        public void Log(LogLevel category, string message, Exception ex)
        {
            logger.Log(category, ex, message);
        }
    }
}
