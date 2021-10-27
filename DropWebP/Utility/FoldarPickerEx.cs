// -----------------------------------------------------------------------
// <copyright file="FoldarPickerEx.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using System;
using System.Diagnostics;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using Windows.Storage;
using Windows.Storage.Pickers;

namespace DropWebP.Utility
{
    /// <summary>
    /// Defines the <see cref="FolderPickerEx" />.
    /// </summary>
    public class FolderPickerEx
    {
        /// <summary>
        /// Defines the mfolderPicker.
        /// </summary>
        private readonly FolderPicker mfolderPicker;

        /// <summary>
        /// Initializes a new instance of the <see cref="FolderPickerEx"/> class.
        /// </summary>
        public FolderPickerEx()
        {
            mfolderPicker = new FolderPicker()
            {
                ViewMode = PickerViewMode.Thumbnail,
                SuggestedStartLocation = PickerLocationId.PicturesLibrary,
            };
            mfolderPicker.FileTypeFilter.Add("*");
            ImageCodecInfo[] codecs = ImageCodecInfo.GetImageEncoders();
            foreach (ImageCodecInfo c in codecs)
            {
                // string codecName = c.CodecName.Substring(8).Replace("Codec", "Files").Trim();
                // mfolderPicker.FileTypeFilter.Add(codecName, new List<string>() { c.FilenameExtension });
                Debug.WriteLine(c.FilenameExtension);

                //mfolderPicker.FileTypeFilter.Add(c.FilenameExtension);
            }
            IntPtr hwnd = GetActiveWindow();
            WinRT.Interop.InitializeWithWindow.Initialize(mfolderPicker, hwnd);
        }

        /// <summary>
        /// The PickSingleFolder.
        /// </summary>
        /// <returns>The <see cref="StorageFolder"/>.</returns>
        public StorageFolder PickSingleFolder()
        {
            return mfolderPicker.PickSingleFolderAsync().GetAwaiter().GetResult();
        }

        /// <summary>
        /// The GetActiveWindow.
        /// </summary>
        /// <returns>The <see cref="IntPtr"/>.</returns>
        [DllImport("user32.dll")]
        private static extern IntPtr GetActiveWindow();
    }
}