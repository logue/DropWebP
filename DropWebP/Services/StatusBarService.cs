using Prism.Events;
namespace DropWebP.Services
{
    public struct StatusBarEvent
    {
        /// <summary>
        /// ステータスバーに表示するメッセージ
        /// </summary>
        public string message;
        /// <summary>
        /// ステータスバーのプログレスバーの値
        /// </summary>
        public int progress;
        /// <summary>
        /// ステータスバーのプログレスバーの最大値
        /// </summary>
        public int progressMaximum;
    }
    class StatusBarService : PubSubEvent<StatusBarEvent>
    {
    }
}
