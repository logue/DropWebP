// -----------------------------------------------------------------------
// <copyright file="MessageService.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Services
{
    using Prism.Events;

    /// <summary>
    /// Defines the <see cref="MessageService" />.
    /// </summary>
    public class MessageService : PubSubEvent<string>
    {
    }
}
