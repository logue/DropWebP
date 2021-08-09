﻿using DropWebP.Interfaces;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Mvvm;
using Prism.Regions;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;

namespace DropWebP.ViewModels
{
    /// <summary>
    /// 設定フライアウトのViewModel
    /// </summary>
    public class ConfigFlyoutViewModel : BindableBase, INavigationAware
    {
        /// <summary>
        /// 多言語化サービス
        /// </summary>
        private readonly ILocalizerService LocalizerService;

        /// <summary>
        /// Flyoutの題名.
        /// </summary>
        public string Header { get; set; } = "Config";

        /// <summary>
        /// Flyout開閉フラグ.
        /// </summary>
        private bool isOpen;

        /// <summary>
        /// 開閉フラグ
        /// </summary>
        public bool IsOpen
        {
            get => isOpen;
            set => SetProperty(ref isOpen, value);
        }

        /// <summary>
        /// Flyoutの位置.
        /// </summary>
        public Position Position { get; set; } = Position.Right;

        /// <summary>
        /// Flyoutの閉じるボタン.
        /// </summary>
        public DelegateCommand CloseFlyoutCommand { get; private set; }

        /// <summary>
        /// 可逆圧縮.
        /// </summary>
        public string LosslessText { get; set; } = "Lossless";

        /// <summary>
        /// 可逆圧縮スイッチ.
        /// </summary>
        public bool ToggleLossless { get => Properties.Settings.Default.Lossless; set => Properties.Settings.Default.Lossless = value; }

        /// <summary>
        /// 圧縮レベル.
        /// </summary>
        public string QualityText { get; set; } = "Quality";

        /// <summary>
        /// 圧縮レベルスライダー.
        /// </summary>
        public long QualityValue { get => Properties.Settings.Default.Quality; set => Properties.Settings.Default.Quality = value; }

        /// <summary>
        /// 変換前のファイルを残す.
        /// </summary>
        public string KeepOriginalText { get; set; } = "Keep Original";

        /// <summary>
        /// 変換前のファイルを残すチェックボックス.
        /// </summary>
        public bool ToggleKeepOriginal { get => Properties.Settings.Default.KeepOriginal; set => Properties.Settings.Default.KeepOriginal = value; }

        /// <summary>
        /// Jpegを無視.
        /// </summary>
        public string IgnoreJpegText { get; set; } = "Ignore JPEG image";

        /// <summary>
        /// Jpegを無視のチェックボックス.
        /// </summary>
        public bool ToggleIgnoreJpeg { get => Properties.Settings.Default.IgnoreJpeg; set => Properties.Settings.Default.IgnoreJpeg = value; }

        /// <summary>
        /// Supported languages
        /// </summary>
        public IList<CultureInfo> SupportedLanguages => LocalizerService?.SupportedLanguages;

        /// <summary>
        /// The selected language
        /// </summary>
        public CultureInfo SelectedLanguage
        {
            get => LocalizerService != null ? LocalizerService.SelectedLanguage : null;
            set
            {
                if (LocalizerService != null && value != null && value != LocalizerService.SelectedLanguage)
                {
                    LocalizerService.SelectedLanguage = value;
                    Properties.Settings.Default.Language = value.ToString();
                }
            }
        }


        /// <summary>
        /// コンストラクタ.
        /// </summary>
        public ConfigFlyoutViewModel(ILocalizerService localizerService)
        {
            CloseFlyoutCommand = new DelegateCommand(ExecuteSaveButtonCommand);
            // SelectedLanguage = SupportedLanguages.FirstOrDefault(c => c.Name.Equals(Properties.Settings.Default.Language, System.StringComparison.Ordinal));

            LocalizerService = localizerService;
        }

        /// <summary>
        /// 設定保存.
        /// </summary>
        private void ExecuteSaveButtonCommand()
        {
            Debug.WriteLine("保存しました");
            // 設定を保存
            Properties.Settings.Default.Save();
            IsOpen = false;
        }

        /// <summary>
        /// イベントを受け取った.
        /// </summary>
        /// <param name="obj">.</param>
        private void OnMesageReceieved(string obj)
        {
            Debug.WriteLine("OnMesageReceieved", obj);
        }

        /// <summary>
        /// The OnNavigatedTo.
        /// </summary>
        /// <param name="navigationContext">The navigationContext<see cref="NavigationContext"/>.</param>
        public void OnNavigatedTo(NavigationContext navigationContext)
        {
            Debug.WriteLine("OnNavigatedTo", navigationContext);
        }

        /// <summary>
        /// The IsNavigationTarget.
        /// </summary>
        /// <param name="navigationContext">The navigationContext<see cref="NavigationContext"/>.</param>
        /// <returns>The <see cref="bool"/>.</returns>
        public bool IsNavigationTarget(NavigationContext navigationContext)
        {
            Debug.WriteLine("IsNavigationTarget", navigationContext);
            return true;
        }

        /// <summary>
        /// ナビゲーション発生.
        /// </summary>
        /// <param name="navigationContext">.</param>
        public void OnNavigatedFrom(NavigationContext navigationContext)
        {
            // Debug.WriteLine("OnNavigatedFrom");
            // flyoutを開く
            Properties.Settings.Default.Reload();
            IsOpen = true;
        }
    }
}
