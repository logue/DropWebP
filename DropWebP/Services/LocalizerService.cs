// -----------------------------------------------------------------------
// <copyright file="LocalizerService.cs" company="Logue">
// Copyright (c) 2021 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

namespace DropWebP.Services
{
    using DropWebP.Interfaces;
    using System.Collections.Generic;
    using System.Globalization;
    using System.Linq;
    using WPFLocalizeExtension.Engine;
    using WPFLocalizeExtension.Extensions;

    /// <summary>
    /// 多言語化サービス.
    /// </summary>
    public class LocalizerService : ILocalizerService
    {
        /// <summary>
        /// Gets the SupportedLanguages
        /// List with supported languages..
        /// </summary>
        public IList<CultureInfo> SupportedLanguages { get; private set; }

        /// <summary>
        /// Gets or sets the SelectedLanguage
        /// The current selected language..
        /// </summary>
        public CultureInfo SelectedLanguage { get => LocalizeDictionary.Instance.Culture; set => SetLocale(value); }

        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="locale">The locale<see cref="string"/>.</param>
        public LocalizerService(string locale = null)
        {
            if (locale == null)
            {
                locale = CultureInfo.CurrentCulture.ToString();
            }

            SupportedLanguages = CultureInfo.GetCultures(CultureTypes.AllCultures).Where(
                c =>
                    c.IetfLanguageTag.Equals("en", System.StringComparison.Ordinal) ||
                    c.IetfLanguageTag.Equals("ja", System.StringComparison.Ordinal)
            )
                .ToList();
            SetLocale(locale);
        }

        /// <summary>
        /// Set localization.
        /// </summary>
        /// <param name="locale">.</param>
        public void SetLocale(string locale)
        {
            LocalizeDictionary.Instance.Culture = CultureInfo.GetCultureInfo(locale);
        }

        /// <summary>
        /// Set localization.
        /// </summary>
        /// <param name="culture">.</param>
        public void SetLocale(CultureInfo culture)
        {
            LocalizeDictionary.Instance.Culture = culture;
        }

        /// <summary>
        /// Get localized string from resource dictionary.
        /// </summary>
        /// <param name="key">.</param>
        /// <returns>.</returns>
        public string GetLocalizedString(string key)
        {
            LocExtension locExtension = new(key);
            _ = locExtension.ResolveLocalizedValue(out string uiString);
            return uiString;
        }
    }
}
