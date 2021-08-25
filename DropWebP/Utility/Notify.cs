// -----------------------------------------------------------------------
// <copyright file="Notify.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Utility
{
    using Windows.Data.Xml.Dom;
    using Windows.UI.Notifications;

    /// <summary>
    /// Defines the <see cref="Notify" />.
    /// </summary>
    public class Notify
    {
        /// <summary>
        /// The Invoke.
        /// </summary>
        /// <param name="title">The Title<see cref="string"/>.</param>
        /// <param name="content">The Content<see cref="string"/>.</param>
        public static void Invoke(string title, string content)
        {
            string xmlString =
    $@"<toast><visual>
       <binding template='ToastGeneric'>
       <text>{title}</text>
       <text>{content}</text>
       </binding>
      </visual></toast>";

            XmlDocument toastXml = new();
            toastXml.LoadXml(xmlString);

            ToastNotification toast = new(toastXml);

            ToastNotificationManager.CreateToastNotifier().Show(toast);
        }
    }
}
