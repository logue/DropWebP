// -----------------------------------------------------------------------
// <copyright file="ConfigFlyoutViewModel.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Interfaces;
using DropWebP.Properties;
using MahApps.Metro.Controls;
using Prism.Commands;
using Prism.Mvvm;
using Prism.Regions;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;

namespace DropWebP.ViewModels;

/// <summary>
///     設定フライアウトのViewModel.
/// </summary>
public class ConfigFlyoutViewModel : BindableBase, INavigationAware
{
    /// <summary>
    ///     多言語化サービス.
    /// </summary>
    private readonly ILocalizeService localizeService;

    /// <summary>
    ///     Flyout開閉フラグ.
    /// </summary>
    private bool isOpen;

    /// <summary>
    ///     Initializes a new instance of the <see cref="ConfigFlyoutViewModel" /> class.
    /// </summary>
    /// <param name="localizeService">多言語化サービス.</param>
    public ConfigFlyoutViewModel(ILocalizeService localizeService)
    {
        // 多言語化サービスのインジェクション
        this.localizeService = localizeService;

        Debug.WriteLine(SupportedLanguages);

        // フライアウトを閉じる
        CloseFlyoutCommand = new DelegateCommand(ExecuteSaveButtonCommand);

        // フライアウトのヘッダー
        Header = localizeService.GetLocalizedString("ConfigText");
    }

    /// <summary>
    ///     可逆圧縮スイッチ.
    /// </summary>
    public static bool ToggleLossless { get => Settings.Default.Lossless; set => Settings.Default.Lossless = value; }

    /// <summary>
    ///     圧縮レベルスライダー.
    /// </summary>
    public static long QualityValue { get => Settings.Default.Quality; set => Settings.Default.Quality = value; }

    /// <summary>
    ///     変換前のファイルを残すチェックボックス.
    /// </summary>
    public static bool ToggleKeepOriginal
    {
        get => Settings.Default.KeepOriginal;
        set => Settings.Default.KeepOriginal = value;
    }

    /// <summary>
    ///     Jpegを無視のチェックボックス.
    /// </summary>
    public static bool ToggleIgnoreJpeg
    {
        get => Settings.Default.IgnoreJpeg;
        set => Settings.Default.IgnoreJpeg = value;
    }

    /// <summary>
    ///     完了時に通知を出す.
    /// </summary>
    public static bool ToggleNotifyComplete
    {
        get => Settings.Default.NotifyComplete;
        set => Settings.Default.NotifyComplete = value;
    }

    /// <summary>
    ///     ダークモード.
    /// </summary>
    public static bool ToggleDarkMode { get => Settings.Default.DarkMode; set => Settings.Default.DarkMode = value; }

    /// <summary>
    ///     Flyoutのヘッダ.
    /// </summary>
    public string Header { get; set; }

    /// <summary>
    ///     開閉フラグ.
    /// </summary>
    public bool IsOpen { get => isOpen; set => SetProperty(ref isOpen, value); }

    /// <summary>
    ///     Flyoutの位置.
    /// </summary>
    public Position Position { get; set; } = Position.Right;

    /// <summary>
    ///     Flyoutの幅.
    /// </summary>
    public int Width { get; set; } = 240;

    /// <summary>
    ///     Flyoutの幅.
    /// </summary>
    public FlyoutTheme Theme { get; set; } = FlyoutTheme.Dark;

    /// <summary>
    ///     Flyoutの閉じるボタン.
    /// </summary>
    public DelegateCommand CloseFlyoutCommand { get; }

    /// <summary>
    ///     対応言語.
    /// </summary>
    public IList<CultureInfo> SupportedLanguages => localizeService.SupportedLanguages;

    /// <summary>
    ///     選択されている言語..
    /// </summary>
    public CultureInfo SelectedLanguage
    {
        get => localizeService?.SelectedLanguage;
        set
        {
            if (localizeService != null && value != null && value != localizeService.SelectedLanguage)
            {
                localizeService.SelectedLanguage = value;
                Settings.Default.Language = value.ToString();
            }
        }
    }

    /// <summary>
    ///     The OnNavigatedTo.
    /// </summary>
    /// <param name="navigationContext">The navigationContext<see cref="NavigationContext" />.</param>
    public void OnNavigatedTo(NavigationContext navigationContext)
    {
        Debug.WriteLine("OnNavigatedTo", navigationContext);
    }

    /// <summary>
    ///     The IsNavigationTarget.
    /// </summary>
    /// <param name="navigationContext">The navigationContext<see cref="NavigationContext" />.</param>
    /// <returns>The <see cref="bool" />.</returns>
    public bool IsNavigationTarget(NavigationContext navigationContext)
    {
        Debug.WriteLine("IsNavigationTarget", navigationContext);
        return true;
    }

    /// <summary>
    ///     ナビゲーション発生.
    /// </summary>
    /// <param name="navigationContext">.</param>
    public void OnNavigatedFrom(NavigationContext navigationContext)
    {
        // Debug.WriteLine("OnNavigatedFrom");
        // flyoutを開く
        Settings.Default.Reload();
        IsOpen = true;
    }

    /// <summary>
    ///     イベントを受け取った.
    /// </summary>
    /// <param name="obj">.</param>
    private static void OnMesageReceieved(string obj)
    {
        Debug.WriteLine("OnMesageReceieved", obj);
    }

    /// <summary>
    ///     設定保存.
    /// </summary>
    private void ExecuteSaveButtonCommand()
    {
        Debug.WriteLine("保存しました");

        // 設定を保存
        Settings.Default.Save();
        IsOpen = false;
    }
}
