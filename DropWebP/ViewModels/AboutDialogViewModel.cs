using DropWebP.Models;
using DropWebP.Utility;
using Prism.Commands;
using Prism.Mvvm;
using Prism.Services.Dialogs;
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Windows.Media;
using WebP.Net;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// アバウト画面のビューモデル.
    /// </summary>
    public class AboutDialogViewModel : BindableBase, IDialogAware
    {
        /// <summary>
        /// 閉じるコマンド
        /// </summary>
        public DelegateCommand CloseCommand { get; }

        /// <summary>
        /// プロジェクトサイト閲覧ボタンのコマンド.
        /// </summary>
        public DelegateCommand VisitCommand { get; }

        /// <summary>
        /// Gets or sets the Name
        /// タブ名.
        /// </summary>
        public string Title => "About";

        /// <summary>
        /// プロジェクトサイト閲覧ボタン.
        /// </summary>
        public string VisitText { get; set; } = "Visit project site";

        /// <summary>
        /// 閉じるボタン
        /// </summary>
        public string CloseText { get; set; } = "Close";

        /// <summary>
        /// ロゴ画像
        /// </summary>
        public ImageSource Logo { get; set; }

        /// <summary>
        /// アセンブリ情報モデル
        /// </summary>
        public AppAssembly Assembly { get; }

        /// <summary>
        /// 使用しているLibWebPのバージョン
        /// </summary>
        public string WebPVersion { get => "libwebp Version: " + WebPLibrary.GetVersion().ToString(); }

        /// <summary>
        /// ダイアログのCloseを要求するAction。.
        /// </summary>
        public event Action<IDialogResult> RequestClose;

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public AboutDialogViewModel()
        {
            VisitCommand = new DelegateCommand(ExecuteVisitCommand);
            CloseCommand = new DelegateCommand(ExecuteCloseCommand);
            Assembly = new AppAssembly();

            Logo = BitmapToImageSource.Convert(Properties.Resources.AppIcon);
        }

        /// <summary>
        /// The CanCloseDialog.
        /// </summary>
        /// <returns>.</returns>
        public bool CanCloseDialog()
        {
            return true;
        }

        /// <summary>
        /// The OnDialogClosed.
        /// </summary>
        public void OnDialogClosed()
        {
        }

        /// <summary>
        /// The OnDialogOpened.
        /// </summary>
        /// <param name="parameters">IDialogServiceに設定されたパラメータを表すIDialogParameters。.</param>
        public void OnDialogOpened(IDialogParameters parameters)
        {
        }

        /// <summary>
        /// プロジェクトサイト閲覧ボタンを実行.
        /// </summary>
        private void ExecuteVisitCommand()
        {
            string url = "https://github.com/logue/DropWebP";
            try
            {
                Process.Start(url);
            }
            catch
            {
                if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                {
                    //Windowsのとき  
                    url = url.Replace("&", "^&");
                    _ = Process.Start(new ProcessStartInfo("cmd", $"/c start {url}") { CreateNoWindow = true });
                }
                else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
                {
                    //Linuxのとき  
                    _ = Process.Start("xdg-open", url);
                }
                else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
                {
                    //Macのとき  
                    _ = Process.Start("open", url);
                }
                else
                {
                    throw;
                }
            }
        }
        /// <summary>
        /// 閉じるボタン
        /// </summary>
        private void ExecuteCloseCommand()
        {
            RequestClose?.Invoke(new DialogResult(ButtonResult.Cancel));
        }
    }
}
