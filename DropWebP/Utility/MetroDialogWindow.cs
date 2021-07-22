using MahApps.Metro.Controls;
using Prism.Services.Dialogs;

namespace DropWebP.Utility
{
    /// <summary>
    /// インラインダイアログのラッパー
    /// </summary>
    class MetroDialogWindow : MetroWindow, IDialogWindow
    {
        public IDialogResult Result { get; set; }
    }
}
