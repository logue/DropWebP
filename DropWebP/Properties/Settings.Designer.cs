﻿//------------------------------------------------------------------------------
// <auto-generated>
//     このコードはツールによって生成されました。
//     ランタイム バージョン:4.0.30319.42000
//
//     このファイルへの変更は、以下の状況下で不正な動作の原因になったり、
//     コードが再生成されるときに損失したりします。
// </auto-generated>
//------------------------------------------------------------------------------

namespace DropWebP.Properties
{
    /// <summary>
    /// Defines the <see cref="Settings" />.
    /// </summary>
    [global::System.Runtime.CompilerServices.CompilerGeneratedAttribute()]
    [global::System.CodeDom.Compiler.GeneratedCodeAttribute("Microsoft.VisualStudio.Editors.SettingsDesigner.SettingsSingleFileGenerator", "16.10.0.0")]
    internal sealed partial class Settings : global::System.Configuration.ApplicationSettingsBase
    {
        /// <summary>
        /// Defines the defaultInstance.
        /// </summary>
        private static Settings defaultInstance = ((Settings)(global::System.Configuration.ApplicationSettingsBase.Synchronized(new Settings())));

        /// <summary>
        /// Gets the Default.
        /// </summary>
        public static Settings Default
        {
            get
            {
                return defaultInstance;
            }
        }

        /// <summary>
        /// Gets or sets a value indicating whether Lossless.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("True")]
        public bool Lossless
        {
            get
            {
                return ((bool)(this["Lossless"]));
            }
            set
            {
                this["Lossless"] = value;
            }
        }

        /// <summary>
        /// Gets or sets the Quality.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("95")]
        public long Quality
        {
            get
            {
                return ((long)(this["Quality"]));
            }
            set
            {
                this["Quality"] = value;
            }
        }

        /// <summary>
        /// Gets or sets a value indicating whether KeepOriginal.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("True")]
        public bool KeepOriginal
        {
            get
            {
                return ((bool)(this["KeepOriginal"]));
            }
            set
            {
                this["KeepOriginal"] = value;
            }
        }

        /// <summary>
        /// Gets or sets a value indicating whether IgnoreJpeg.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("False")]
        public bool IgnoreJpeg
        {
            get
            {
                return ((bool)(this["IgnoreJpeg"]));
            }
            set
            {
                this["IgnoreJpeg"] = value;
            }
        }

        /// <summary>
        /// Gets or sets the Language.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("")]
        public string Language
        {
            get
            {
                return ((string)(this["Language"]));
            }
            set
            {
                this["Language"] = value;
            }
        }

        /// <summary>
        /// Gets or sets a value indicating whether NotifyComplete.
        /// </summary>
        [global::System.Configuration.UserScopedSettingAttribute()]
        [global::System.Diagnostics.DebuggerNonUserCodeAttribute()]
        [global::System.Configuration.DefaultSettingValueAttribute("True")]
        public bool NotifyComplete
        {
            get
            {
                return ((bool)(this["NotifyComplete"]));
            }
            set
            {
                this["NotifyComplete"] = value;
            }
        }
    }
}
