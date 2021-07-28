using DropWebP.Interfaces;
using DropWebP.Utility;
using MahApps.Metro.Controls;
using MahApps.Metro.Controls.Dialogs;
using Prism.Commands;
using Prism.Events;
using Prism.Mvvm;
using Prism.Regions;
using System.Diagnostics;
using System.Drawing.Imaging;
using System.IO;
using System.Linq;
using System.Windows;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// ドラッグアンドロップする領域のViewModel
    /// https://github.com/nabehiro22/Image_DragAndDrop のコードを流用.
    /// </summary>
    public class HomeContentViewModel : BindableBase
    {
        /// <summary>
        /// Defines the eventAggregator.
        /// </summary>
        private IEventAggregator eventAggregator;

        /// <summary>
        /// Defines the regionManager.
        /// </summary>
        private IRegionManager regionManager;

        /// <summary>
        /// Gets or sets the Shell
        /// MetroWindow.
        /// </summary>
        public MetroWindow Shell { get; set; } = Application.Current.MainWindow as MetroWindow;

        /// <summary>
        /// Webのエンコーダー.
        /// </summary>
        private readonly IWebPService webPService;

        /// <summary>
        /// ファイルブラウザボタンのコマンド.
        /// </summary>
        public DelegateCommand BrowseButtonCommand { get; }

        /// <summary>
        /// タブ名.
        /// </summary>
        public string Name { get; set; } = "Home";

        /// <summary>
        /// ブラウズボタン.
        /// </summary>
        public string BrowseText { get; set; } = "Browse Foldar...";

        /// <summary>
        /// ドラッグ・アンド・ドロップで画像ファイルをWebPに変換.
        /// </summary>
        public string Message { get; set; } = "Drag and drop image file(s) to convert WebP.";

        /// <summary>
        /// Gets or sets the MahAppsDialogCoordinator
        /// ダイアログのインスタンス.
        /// </summary>
        public IDialogCoordinator MahAppsDialogCoordinator { get; set; }

        /// <summary>
        /// Initializes a new instance of the <see cref="HomeContentViewModel"/> class.
        /// </summary>
        /// <param name="eventAggregator">The eventAggregator<see cref="IEventAggregator"/>.</param>
        /// <param name="regionManager">インジェクションするIRegionManager。.</param>
        /// <param name="webPService">The webPService<see cref="IWebPService"/>.</param>
        public HomeContentViewModel(IEventAggregator eventAggregator, IRegionManager regionManager, IWebPService webPService)
        {
            // フォルダ選択ボタンイベントハンドラ
            BrowseButtonCommand = new DelegateCommand(ExecuteBrowseButtonCommand);

            this.eventAggregator = eventAggregator;
            this.regionManager = regionManager;
            this.webPService = webPService;
        }

        /// <summary>
        /// ファイルブラウザボタンが押された.
        /// </summary>
        private void ExecuteBrowseButtonCommand()
        {
            // ダイアログを定義
            FolderPickerEx picker = new();
            // ファイルダイアログを表示
            var folder = picker.PickSingleFolder();

            if (folder == null)
            {
                return;
            }

            /*
            // TODO: ファイルの絞り込み処理
            string[] imageFileExtensions = ImageCodecInfo.GetImageEncoders()
                                      .Select(c => c.FilenameExtension.Replace("*", ""))
                                      .SelectMany(e => e.Split(';')).ToArray();

            // ファイル一覧
            string[] files = Directory.GetFiles(folder.Path, string.Join(",", imageFileExtensions) + ".exr", SearchOption.TopDirectoryOnly);
            Debug.WriteLine(string.Join(",", imageFileExtensions));
            */
            // 変換処理
            webPService.Convert(Directory.GetFiles(folder.Path), Shell);
        }
    }
}
