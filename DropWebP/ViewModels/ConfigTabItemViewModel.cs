using Prism.Commands;
using Prism.Mvvm;

namespace DropWebP.ViewModels
{
    public class ConfigTabItemViewModel : BindableBase
    {
        /// <summary>
        /// タブ名
        /// </summary>
        public string Name { get; set; } = "Config";

        /// <summary>
        /// 可逆圧縮
        /// </summary>
        public string LosslessText { get; set; } = "Lossless";

        /// <summary>
        /// 可逆圧縮スイッチ
        /// </summary>
        public bool ToggleLossless
        {
            get => Properties.Settings.Default.Lossless;
            set
            {
                Properties.Settings.Default.Lossless = value;
            }
        }

        /// <summary>
        /// 圧縮レベル
        /// </summary>
        public string QualityText { get; set; } = "Quality";

        /// <summary>
        /// 圧縮レベルスライダー
        /// </summary>
        public long QualityValue
        {
            get => Properties.Settings.Default.Quality;
            set
            {
                Properties.Settings.Default.Quality = value;
            }
        }

        /// <summary>
        /// 変換前のファイルを残す
        /// </summary>
        public string KeepOriginalText { get; set; } = "Keep Original";

        /// <summary>
        /// 変換前のファイルを残すチェックボックス
        /// </summary>
        public bool ToggleKeepOriginal
        {
            get => Properties.Settings.Default.KeepOriginal;
            set
            {
                Properties.Settings.Default.KeepOriginal = value;
            }
        }

        /// <summary>
        /// 設定保存ボタンのコマンド
        /// </summary>
        public DelegateCommand SaveButtonCommand { get; }

        /// <summary>
        /// 保存ボタン
        /// </summary>
        public string SaveText { get; set; } = "Save Configure";

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public ConfigTabItemViewModel()
        {
            SaveButtonCommand = new DelegateCommand(ExecuteSaveButtonCommand);
        }

        /// <summary>
        /// 設定保存
        /// </summary>
        private void ExecuteSaveButtonCommand()
        {
            Properties.Settings.Default.Save();
            // TODO: 保存完了通知
        }
    }
}
