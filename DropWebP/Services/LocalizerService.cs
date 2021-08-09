using DropWebP.Interfaces;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using WPFLocalizeExtension.Engine;
using WPFLocalizeExtension.Extensions;

namespace DropWebP.Services
{
    public class LocalizerService : ILocalizerService
    {
        /// <summary>
        /// List with supported languages
        /// </summary>
        public IList<CultureInfo> SupportedLanguages { get; private set; }

        /// <summary>
        /// The current selected language
        /// </summary>
        public CultureInfo SelectedLanguage
        {
            get => LocalizeDictionary.Instance.Culture;
            set => SetLocale(value);
        }

        /// <summary>
        /// Ctor
        /// </summary>
        /// <param name="culture"></param>
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
        /// Set localization
        /// </summary>
        /// <param name="locale"></param>
        public void SetLocale(string locale)
        {
            LocalizeDictionary.Instance.Culture = CultureInfo.GetCultureInfo(locale);
        }

        /// <summary>
        /// Set localization
        /// </summary>
        /// <param name="culture"></param>
        public void SetLocale(CultureInfo culture)
        {
            LocalizeDictionary.Instance.Culture = culture;
        }

        /// <summary>
        /// Get localized string from resource dictionary
        /// </summary>
        /// <param name="key"></param>
        /// <returns></returns>
        public string GetLocalizedString(string key)
        {
            LocExtension locExtension = new(key);
            _ = locExtension.ResolveLocalizedValue(out string uiString);
            return uiString;
        }
    }
}