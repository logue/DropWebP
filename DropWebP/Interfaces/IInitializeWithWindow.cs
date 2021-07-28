using System;
using System.Runtime.InteropServices;

namespace DropWebP.Interfaces
{
    /// <summary>
    /// WPFからMessageDialogを呼ぶ場合のおまじない
    /// https://qiita.com/okazuki/items/227f8d19e38a67099006.
    /// </summary>
    [ComImport]
    [Guid("3E68D4BD-7135-4D10-8018-9FB6D9F33FA1")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IInitializeWithWindow
    {
        /// <summary>
        /// The Initialize.
        /// </summary>
        /// <param name="hwnd">The hwnd<see cref="IntPtr"/>.</param>
        void Initialize(IntPtr hwnd);
    }
}
