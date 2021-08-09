using Microsoft.Extensions.Logging;
using System;

namespace DropWebP.Interfaces
{
    public interface ILoggerService
    {
        void Log(LogLevel category, string message);
        void Log(LogLevel category, string message, Exception ex);
    }
}
