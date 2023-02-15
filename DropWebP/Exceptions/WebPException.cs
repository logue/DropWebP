// -----------------------------------------------------------------------
// <copyright file="WebPException.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System;
using System.Runtime.Serialization;

namespace DropWebP.Exceptions;

/// <summary>
///     WebPService内で発生した例外
/// </summary>
internal class WebPException : Exception
{
    /// <summary>
    ///     Initializes a new instance of the <see cref="WebPException" /> class.
    /// </summary>
    /// <param name="e">Exception</param>
    public WebPException(Exception e)
    {
    }

    /// <summary>
    ///     Initializes a new instance of the <see cref="WebPException" /> class.
    /// </summary>
    /// <param name="message">
    ///     <inheritdoc />
    /// </param>
    public WebPException(string message)
        : base(message)
    {
    }

    /// <summary>
    ///     Initializes a new instance of the <see cref="WebPException" /> class.
    /// </summary>
    /// <param name="message">
    ///     <inheritdoc />
    /// </param>
    /// <param name="innerException">
    ///     <inheritdoc />
    /// </param>
    public WebPException(string message, Exception innerException)
        : base(message, innerException)
    {
    }

    /// <summary>
    ///     Initializes a new instance of the <see cref="WebPException" /> class.
    /// </summary>
    /// <param name="info">
    ///     <inheritdoc />
    /// </param>
    /// <param name="context">
    ///     <inheritdoc />
    /// </param>
    protected WebPException(SerializationInfo info, StreamingContext context)
        : base(info, context)
    {
    }
}
