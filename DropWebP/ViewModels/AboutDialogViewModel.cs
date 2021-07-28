using Prism.Commands;
using Prism.Mvvm;
using Prism.Services.Dialogs;
using System;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Windows.Input;
using WebP.Net;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// アバウト画面のビューモデル.
    /// </summary>
    public class AboutDialogViewModel : BindableBase, IDialogAware
    {
        /// <summary>
        /// Gets or sets the Name
        /// タブ名.
        /// </summary>
        public string Name { get; set; } = "About";

        /// <summary>
        /// Gets or sets the VisitText
        /// プロジェクトサイト閲覧ボタン.
        /// </summary>
        public string VisitText { get; set; } = "Visit project site";

        /// <summary>
        /// 閉じるコマンド
        /// </summary>
        private readonly ICommand CloseCommand;

        /// <summary>
        /// Gets the VisitButtonCommand
        /// プロジェクトサイト閲覧ボタンのコマンド.
        /// </summary>
        public DelegateCommand VisitButtonCommand { get; }

        /// <summary>
        /// Gets the Title
        /// アプリケーション名.
        /// </summary>
        public string Title
        {
            get
            {
                object[] attributes = Assembly.GetExecutingAssembly().GetCustomAttributes(typeof(AssemblyTitleAttribute), false);
                if (attributes.Length > 0)
                {
                    AssemblyTitleAttribute titleAttribute = (AssemblyTitleAttribute)attributes[0];
                    if (titleAttribute.Title != "")
                    {
                        return titleAttribute.Title;
                    }
                }

                // return System.IO.Path.GetFileNameWithoutExtension(Assembly.GetExecutingAssembly().CodeBase);
                return ((AssemblyTitleAttribute)attributes[0]).Title;
            }
        }

        /// <summary>
        /// Gets the Version
        /// バージョン.
        /// </summary>
        public string Version => Assembly.GetExecutingAssembly().GetName().Version.ToString();

        /// <summary>
        /// Gets the Description
        /// 説明.
        /// </summary>
        public string Description
        {
            get
            {
                object[] attributes = Assembly.GetExecutingAssembly().GetCustomAttributes(typeof(AssemblyDescriptionAttribute), false);
                if (attributes.Length == 0)
                {
                    return "";
                }
                return ((AssemblyDescriptionAttribute)attributes[0]).Description;
            }
        }

        /// <summary>
        /// Gets the Product
        /// 製品名.
        /// </summary>
        public string Product
        {
            get
            {
                object[] attributes = Assembly.GetExecutingAssembly().GetCustomAttributes(typeof(AssemblyProductAttribute), false);
                if (attributes.Length == 0)
                {
                    return "";
                }
                return ((AssemblyProductAttribute)attributes[0]).Product;
            }
        }

        /// <summary>
        /// Gets the Copyright
        /// 著作権表記.
        /// </summary>
        public string Copyright
        {
            get
            {
                object[] attributes = Assembly.GetExecutingAssembly().GetCustomAttributes(typeof(AssemblyCopyrightAttribute), false);
                if (attributes.Length == 0)
                {
                    return "";
                }
                return ((AssemblyCopyrightAttribute)attributes[0]).Copyright;
            }
        }

        /// <summary>
        /// Gets the Company
        /// 会社名.
        /// </summary>
        public string Company
        {
            get
            {
                object[] attributes = Assembly.GetExecutingAssembly().GetCustomAttributes(typeof(AssemblyCompanyAttribute), false);
                if (attributes.Length == 0)
                {
                    return "";
                }
                return ((AssemblyCompanyAttribute)attributes[0]).Company;
            }
        }

        /// <summary>
        /// Gets the WebPVersion.
        /// </summary>
        public string WebPVersion { get => "libwebp Version: " + WebPLibrary.GetVersion().ToString(); }

        /// <summary>
        /// ダイアログのCloseを要求するAction。.
        /// </summary>
        public event Action<IDialogResult> RequestClose;

        /// <summary>
        /// Initializes a new instance of the <see cref="AboutDialogViewModel"/> class.
        /// </summary>
        public AboutDialogViewModel()
        {
            VisitButtonCommand = new DelegateCommand(ExecuteVisitButtonCommand);

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
        private void ExecuteVisitButtonCommand()
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
                    Process.Start(new ProcessStartInfo("cmd", $"/c start {url}") { CreateNoWindow = true });
                }
                else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
                {
                    //Linuxのとき  
                    Process.Start("xdg-open", url);
                }
                else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
                {
                    //Macのとき  
                    Process.Start("open", url);
                }
                else
                {
                    throw;
                }
            }
        }
    }
}
