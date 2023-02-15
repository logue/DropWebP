// -----------------------------------------------------------------------
// <copyright file="AboutDialogViewModel.cs" company="Logue">
// Copyright (c) 2021-2023 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Helpers;
using DropWebP.Interfaces;
using DropWebP.Models;
using DropWebP.Properties;
using Prism.Commands;
using Prism.Mvvm;
using Prism.Services.Dialogs;
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Windows.Media;
using WebP.Net;

namespace DropWebP.ViewModels;

/// <summary>
///     バージョン情報ダイアログのビューモデル.
/// </summary>
public class AboutDialogViewModel : BindableBase, IDialogAware
{
    /// <summary>
    ///     多言語化サービス.
    /// </summary>
    private readonly ILocalizeService localizeService;

    /// <summary>
    ///     Initializes a new instance of the <see cref="AboutDialogViewModel" /> class.
    /// </summary>
    /// <param name="localizeService">多言語化サービス.</param>
    public AboutDialogViewModel(ILocalizeService localizeService)
    {
        // 多言語化サービスのインジェクション
        this.localizeService = localizeService;
        VisitCommand = new DelegateCommand(ExecuteVisitCommand);
        CloseCommand = new DelegateCommand(ExecuteCloseCommand);
        Assembly = new AppAssemblyModel();
        Logo = BitmapToImageSource.Convert(Resources.AppIcon);
    }

    /// <summary>
    ///     閉じるコマンド.
    /// </summary>
    public DelegateCommand CloseCommand { get; }

    /// <summary>
    ///     Gets the VisitCommand
    ///     プロジェクトサイト閲覧ボタンのコマンド.
    /// </summary>
    public DelegateCommand VisitCommand { get; }

    /// <summary>
    ///     ロゴ画像.
    /// </summary>
    public ImageSource Logo { get; }

    /// <summary>
    ///     アセンブリ情報モデル.
    /// </summary>
    public AppAssemblyModel Assembly { get; }

    /// <summary>
    ///     使用しているLibWebPのバージョン.
    /// </summary>
    /// <returns>LibWepPのバージョン</returns>
    public string WebPVersion { get; } = "libwebp Version: " + WebPObject.GetVersion();

    /// <summary>
    ///     タイトル.
    /// </summary>
    public string Title { get; } = "About";

    /// <summary>
    ///     ダイアログのCloseを要求するAction.
    /// </summary>
    public event Action<IDialogResult> RequestClose;

    /// <summary>
    ///     ダイアログを閉じることができるか.
    /// </summary>
    /// <returns>.</returns>
    public bool CanCloseDialog()
    {
        return true;
    }

    /// <summary>
    ///     The OnDialogClosed.
    /// </summary>
    public void OnDialogClosed()
    {
    }

    /// <summary>
    ///     The OnDialogOpened.
    /// </summary>
    /// <param name="parameters">IDialogServiceに設定されたパラメータを表すIDialogParameters.</param>
    public void OnDialogOpened(IDialogParameters parameters)
    {
    }

    /// <summary>
    ///     プロジェクトサイト閲覧ボタンを実行.
    /// </summary>
    private void ExecuteVisitCommand()
    {
        string url = "https://github.com/logue/DropWebP";
        try
        {
            _ = Process.Start(url);
        }
        catch
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                // Windowsのとき
                url = url.Replace("&", "^&");
                _ = Process.Start(new ProcessStartInfo("cmd", $"/c start {url}") { CreateNoWindow = true });
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            {
                // Linuxのとき
                _ = Process.Start("xdg-open", url);
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                // Macのとき
                _ = Process.Start("open", url);
            }
            else
            {
                throw;
            }
        }
    }

    /// <summary>
    ///     閉じるボタン.
    /// </summary>
    private void ExecuteCloseCommand()
    {
        RequestClose?.Invoke(new DialogResult(ButtonResult.Cancel));
    }
}
