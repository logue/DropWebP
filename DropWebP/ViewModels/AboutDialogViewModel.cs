using Prism.Commands;
using Prism.Mvvm;
using Prism.Services.Dialogs;
using System;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.InteropServices;
using WebP.Net;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// アバウト画面のビューモデル
    /// </summary>
    public class AboutDialogViewModel : BindableBase, IDialogAware
    {
        #region Properties
        /// <summary>
        /// タブ名
        /// </summary>
        public string Name { get; set; } = "About";

        /// <summary>
        /// プロジェクトサイト閲覧ボタン
        /// </summary>
        public string VisitText { get; set; } = "Visit project site";

        /// <summary>
        /// プロジェクトサイト閲覧ボタンのコマンド
        /// </summary>
        public DelegateCommand VisitButtonCommand { get; }

        /// <summary>
        /// アプリケーション名
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
        /// バージョン
        /// </summary>
        public string Version => Assembly.GetExecutingAssembly().GetName().Version.ToString();
        /// <summary>
        /// 説明
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
        /// 製品名
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
        /// 著作権表記
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
        /// 会社名
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

        public string WebPVersion
        {
            get => "libwebp Version: " + WebPLibrary.GetVersion().ToString();
        }

        /// <summary>
        /// ダイアログのCloseを要求するAction。
        /// </summary>
        public event Action<IDialogResult> RequestClose;
        #endregion

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public AboutDialogViewModel()
        {
            VisitButtonCommand = new DelegateCommand(ExecuteVisitButtonCommand);
        }

        /// <summary>ダイアログがClose可能かを取得します。</summary>
        /// <returns></returns>
        public bool CanCloseDialog() { return true; }

        /// <summary>ダイアログClose時のイベントハンドラ。</summary>
        public void OnDialogClosed() { }

        /// <summary>ダイアログOpen時のイベントハンドラ。</summary>
        /// <param name="parameters">IDialogServiceに設定されたパラメータを表すIDialogParameters。</param>
        public void OnDialogOpened(IDialogParameters parameters) { }

        /// <summary>
        /// プロジェクトサイト閲覧ボタンを実行
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
