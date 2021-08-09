using DropWebP.Interfaces;
using Microsoft.Extensions.Logging;
using System;

namespace DropWebP.Services
{
    class LoggerService : ILoggerService
    {
        private readonly ILogger _logger;
        public LoggerService(ILogger logger)
        {
            _logger = logger;
        }

        public void Log(LogLevel category, string message)
        {
            _logger.Log(category, message);
        }

        public void Log(LogLevel category, string message, Exception ex)
        {
            _logger.Log(category, ex, message);
        }
    }
}
