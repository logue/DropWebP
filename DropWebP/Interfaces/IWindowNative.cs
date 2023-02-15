using System;
using System.Runtime.InteropServices;

namespace DropWebP.Interfaces;

/// <summary>
///     Defines the <see cref="IWindowNative" />.
/// </summary>
[ComImport]
[Guid("EECDBF0E-BAE9-4CB6-A68E-9598E1CB57BB")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IWindowNative
{
    /// <summary>
    ///     Gets the WindowHandle.
    /// </summary>
    IntPtr WindowHandle { get; }
}
