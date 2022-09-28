// -----------------------------------------------------------------------
// <copyright file="EnumToBooleanConverter.cs" company="Logue">
// Copyright (c) 2021-2022 Masashi Yoshikawa All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.
// </copyright>
// -----------------------------------------------------------------------

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Data;

namespace DropWebP.Helpers;

/// <summary>
/// 列挙型をbool型に変換するクラス
/// </summary>
public class EnumToBooleanConverter : IValueConverter
{
    /// <summary>
    /// Initializes a new instance of the <see cref="EnumToBooleanConverter"/> class.
    /// </summary>
    public EnumToBooleanConverter()
    {
    }

    /// <inheritdoc/>
    public object Convert(object value, Type targetType, object parameter, string language)
    {
        if (parameter is string enumString)
        {
            if (!Enum.IsDefined(typeof(ElementTheme), value))
            {
                throw new ArgumentException("ExceptionEnumToBooleanConverterValueMustBeAnEnum");
            }

            var enumValue = Enum.Parse(typeof(ElementTheme), enumString);

            return enumValue.Equals(value);
        }

        throw new ArgumentException("ExceptionEnumToBooleanConverterParameterMustBeAnEnumName");
    }

    /// <inheritdoc/>
    public object ConvertBack(object value, Type targetType, object parameter, string language)
    {
        if (parameter is string enumString)
        {
            return Enum.Parse(typeof(ElementTheme), enumString);
        }

        throw new ArgumentException("ExceptionEnumToBooleanConverterParameterMustBeAnEnumName");
    }
}
