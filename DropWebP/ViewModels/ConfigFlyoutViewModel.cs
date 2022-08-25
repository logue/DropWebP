// -----------------------------------------------------------------------
// <copyright file="ConfigFlyoutViewModel.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Interfaces;
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
    /// 設定フライアウトのViewModel.
    /// </summary>
    public class ConfigFlyoutViewModel : BindableBase, INavigationAware
    {
        /// <summary>
        /// 多言語化サービス.
        /// </summary>
        private readonly ILocalizerService localizerService;

        /// <summary>
        /// 可逆圧縮スイッチ.
        /// </summary>
        public static bool ToggleLossless { get => Properties.Settings.Default.Lossless; set => Properties.Settings.Default.Lossless = value; }

        /// <summary>
        /// 圧縮レベルスライダー.
        /// </summary>
        public static long QualityValue { get => Properties.Settings.Default.Quality; set => Properties.Settings.Default.Quality = value; }

        /// <summary>
        /// 変換前のファイルを残すチェックボックス.
        /// </summary>
        public static bool ToggleKeepOriginal { get => Properties.Settings.Default.KeepOriginal; set => Properties.Settings.Default.KeepOriginal = value; }

        /// <summary>
        /// Jpegを無視のチェックボックス.
        /// </summary>
        public static bool ToggleIgnoreJpeg { get => Properties.Settings.Default.IgnoreJpeg; set => Properties.Settings.Default.IgnoreJpeg = value; }

        /// <summary>
        /// 完了時に通知を出す.
        /// </summary>
        public static bool ToggleNotifyComplete { get => Properties.Settings.Default.NotifyComplete; set => Properties.Settings.Default.NotifyComplete = value; }

        /// <summary>
        /// イベントを受け取った.
        /// </summary>
        /// <param name="obj">.</param>
        private static void OnMesageReceieved(string obj)
        {
            Debug.WriteLine("OnMesageReceieved", obj);
        }

        /// <summary>
        /// Flyoutのヘッダ.
        /// </summary>
        public string Header { get; set; } = "Config";

        /// <summary>
        /// Flyout開閉フラグ.
        /// </summary>
        private bool isOpen;

        /// <summary>
        /// 開閉フラグ.
        /// </summary>
        public bool IsOpen { get => isOpen; set => SetProperty(ref isOpen, value); }

        /// <summary>
        /// Flyoutの位置.
        /// </summary>
        public Position Position { get; set; } = Position.Right;

        /// <summary>
        /// Flyoutの幅.
        /// </summary>
        public int Width { get; set; } = 200;

        /// <summary>
        /// Flyoutの幅.
        /// </summary>
        public FlyoutTheme Theme { get; set; } = FlyoutTheme.Dark;

        /// <summary>
        /// Flyoutの閉じるボタン.
        /// </summary>
        public DelegateCommand CloseFlyoutCommand { get; private set; }

        /// <summary>
        /// 対応言語.
        /// </summary>
        public IList<CultureInfo> SupportedLanguages => localizerService.SupportedLanguages;

        /// <summary>
        /// 選択されている言語..
        /// </summary>
        public CultureInfo SelectedLanguage
        {
            get => localizerService != null ? localizerService.SelectedLanguage : null;
            set
            {
                if (localizerService != null && value != null && value != localizerService.SelectedLanguage)
                {
                    localizerService.SelectedLanguage = value;
                    Properties.Settings.Default.Language = value.ToString();
                }
            }
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="ConfigFlyoutViewModel"/> class.
        /// </summary>
        /// <param name="localizerService">多言語化サービス.</param>
        public ConfigFlyoutViewModel(ILocalizerService localizerService)
        {
            // 多言語化サービスのインジェクション
            this.localizerService = localizerService;

            Debug.WriteLine(SupportedLanguages);

            // フライアウトを閉じる
            CloseFlyoutCommand = new DelegateCommand(ExecuteSaveButtonCommand);

            // フライアウトのヘッダー
            Header = localizerService.GetLocalizedString("ConfigText");
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
    }
}