using MahApps.Metro.Controls;
using Prism.Services.Dialogs;

namespace DropWebP.Utility
{
    /// <summary>
    /// インラインダイアログのラッパー.
    /// </summary>
    internal class MetroDialogWindow : MetroWindow, IDialogWindow
    {
        /// <summary>
        /// Gets or sets the Result.
        /// </summary>
        public IDialogResult Result { get; set; }
    }
}
