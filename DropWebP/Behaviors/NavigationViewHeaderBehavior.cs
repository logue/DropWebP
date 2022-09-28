// -----------------------------------------------------------------------
// <copyright file="NavigationViewHeaderBehavior.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using DropWebP.Contracts.Services;

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;
using Microsoft.Xaml.Interactivity;

namespace DropWebP.Behaviors;

/// <summary>
/// ナビゲーションビューのヘッダビヘイビア
/// </summary>
public class NavigationViewHeaderBehavior : Behavior<NavigationView>
{
    /// <summary>
    /// 現在のビヘイビア
    /// </summary>
    private static NavigationViewHeaderBehavior? current;

    /// <summary>
    /// 現在のページ
    /// </summary>
    private Page? currentPage;

    /// <summary>
    /// ヘッダーのテンプレート
    /// </summary>
    public DataTemplate? DefaultHeaderTemplate
    {
        get; set;
    }

    /// <summary>
    /// デフォルトのヘッダ
    /// </summary>
    public object DefaultHeader
    {
        get => GetValue(DefaultHeaderProperty);
        set => SetValue(DefaultHeaderProperty, value);
    }

    /// <summary>
    /// デフォルトのヘッダーのプロパティ
    /// </summary>
    public static readonly DependencyProperty DefaultHeaderProperty =
        DependencyProperty.Register("DefaultHeader", typeof(object), typeof(NavigationViewHeaderBehavior), new PropertyMetadata(null, (d, e) => current!.UpdateHeader()));

    /// <summary>
    /// ナビゲーション
    /// </summary>
    /// <param name="item">項目</param>
    /// <returns>項目の値</returns>
    public static NavigationViewHeaderMode GetHeaderMode(Page item) => (NavigationViewHeaderMode)item.GetValue(HeaderModeProperty);

    /// <summary>
    /// ヘッダーのモードを設定
    /// </summary>
    /// <param name="item">ページ</param>
    /// <param name="value">値</param>
    public static void SetHeaderMode(Page item, NavigationViewHeaderMode value) => item.SetValue(HeaderModeProperty, value);

    /// <summary>
    /// ヘッダーのモードのプロパティ
    /// </summary>
    public static readonly DependencyProperty HeaderModeProperty =
        DependencyProperty.RegisterAttached("HeaderMode", typeof(bool), typeof(NavigationViewHeaderBehavior), new PropertyMetadata(NavigationViewHeaderMode.Always, (d, e) => current!.UpdateHeader()));

    /// <summary>
    /// ヘッダーの項目のコンテキスト
    /// </summary>
    /// <param name="item">ページ</param>
    /// <returns>項目の値</returns>
    public static object GetHeaderContext(Page item) => item.GetValue(HeaderContextProperty);

    /// <summary>
    /// ヘッダーのコンテキストを設定
    /// </summary>
    /// <param name="item">ページ</param>
    /// <param name="value">値</param>
    public static void SetHeaderContext(Page item, object value) => item.SetValue(HeaderContextProperty, value);

    /// <summary>
    /// ヘッダーのコンテキストのプロパティ
    /// </summary>
    public static readonly DependencyProperty HeaderContextProperty =
        DependencyProperty.RegisterAttached("HeaderContext", typeof(object), typeof(NavigationViewHeaderBehavior), new PropertyMetadata(null, (d, e) => current!.UpdateHeader()));

    /// <summary>
    /// ヘッダーのテンプレートを取得
    /// </summary>
    /// <param name="item">ページ</param>
    /// <returns>テンプレート</returns>
    public static DataTemplate GetHeaderTemplate(Page item) => (DataTemplate)item.GetValue(HeaderTemplateProperty);

    /// <summary>
    /// ヘッダー
    /// </summary>
    /// <param name="item">ページ</param>
    /// <param name="value">値</param>
    public static void SetHeaderTemplate(Page item, DataTemplate value) => item.SetValue(HeaderTemplateProperty, value);

    /// <summary>
    /// ヘッダーのテンプレートのプロパティ
    /// </summary>
    public static readonly DependencyProperty HeaderTemplateProperty =
        DependencyProperty.RegisterAttached("HeaderTemplate", typeof(DataTemplate), typeof(NavigationViewHeaderBehavior), new PropertyMetadata(null, (d, e) => current!.UpdateHeaderTemplate()));

    /// <inheritdoc/>
    protected override void OnAttached()
    {
        base.OnAttached();

        var navigationService = App.GetService<INavigationService>();
        navigationService.Navigated += OnNavigated;

        current = this;
    }

    /// <inheritdoc/>
    protected override void OnDetaching()
    {
        base.OnDetaching();

        var navigationService = App.GetService<INavigationService>();
        navigationService.Navigated -= OnNavigated;
    }

    /// <summary>
    /// ナビゲーション発生時のイベントハンドラ
    /// </summary>
    /// <param name="sender">送信元</param>
    /// <param name="e">イベント</param>
    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        if (sender is Frame frame && frame.Content is Page page)
        {
            currentPage = page;

            UpdateHeader();
            UpdateHeaderTemplate();
        }
    }

    /// <summary>
    /// ヘッダーを更新
    /// </summary>
    private void UpdateHeader()
    {
        if (currentPage != null)
        {
            var headerMode = GetHeaderMode(currentPage);
            if (headerMode == NavigationViewHeaderMode.Never)
            {
                AssociatedObject.Header = null;
                AssociatedObject.AlwaysShowHeader = false;
            }
            else
            {
                var headerFromPage = GetHeaderContext(currentPage);
                if (headerFromPage != null)
                {
                    AssociatedObject.Header = headerFromPage;
                }
                else
                {
                    AssociatedObject.Header = DefaultHeader;
                }

                if (headerMode == NavigationViewHeaderMode.Always)
                {
                    AssociatedObject.AlwaysShowHeader = true;
                }
                else
                {
                    AssociatedObject.AlwaysShowHeader = false;
                }
            }
        }
    }

    /// <summary>
    /// ヘッダーのテンプレートの更新
    /// </summary>
    private void UpdateHeaderTemplate()
    {
        if (currentPage != null)
        {
            var headerTemplate = GetHeaderTemplate(currentPage);
            AssociatedObject.HeaderTemplate = headerTemplate ?? DefaultHeaderTemplate;
        }
    }
}
