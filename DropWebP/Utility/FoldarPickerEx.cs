using System;
using System.Diagnostics;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using Windows.Storage;
using Windows.Storage.Pickers;

/// <summary>
/// FoldarPickerのラッパー
/// </summary>
/// <see cref="https://github.com/microsoft/WinUI-3-Demos/blob/FolderPickerEx/src/Build2020Demo/DemoBuildCs/DemoBuildCs/DemoBuildCs/FolderPickerEx.cs"/>
namespace DropWebP.Utility
{
    class FolderPickerEx
    {
        FolderPicker mfolderPicker;

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

        public StorageFolder PickSingleFolder()
        {
            return mfolderPicker.PickSingleFolderAsync().GetAwaiter().GetResult();
        }

        [DllImport("user32.dll")]
        private static extern IntPtr GetActiveWindow();
    }
}
