using Prism.Events;

namespace DropWebP.Services
{
    /// <summary>
    /// Defines the <see cref="MessageService" />.
    /// </summary>
    public class MessageService : PubSubEvent<string>
    {
    }
}
