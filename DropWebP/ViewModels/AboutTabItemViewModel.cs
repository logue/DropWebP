using Prism.Commands;
using Prism.Mvvm;
using System.Reflection;
using WebP.Net;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// アバウトタブのビューモデル
    /// </summary>
    public class AboutTabItemViewModel : BindableBase
    {
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
        /// コンストラクタ
        /// </summary>
        public AboutTabItemViewModel()
        {
            VisitButtonCommand = new DelegateCommand(ExecuteVisitButtonCommand);
        }

        /// <summary>
        /// プロジェクトサイト閲覧ボタンを実行
        /// </summary>
        private void ExecuteVisitButtonCommand()
        {
            System.Diagnostics.Process.Start("https://github.com/logue/DropWebP");
        }
    }
}
