using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace rustiny
{
    class Logger
    {
        private RustinyApi.DLoggerBind m_cbLog;
        public Logger()
        {
        }

        public void Initialize()
        {
            m_cbLog += CallbackLog;
            RustinyApi.LoggerBind(m_cbLog);
        }

        public void CallbackLog(LogMessage a_message)
        {
            switch (a_message.level)
            {
                case Level.Error:
                    UnityEngine.Debug.LogError("[RUST] ERROR: " + a_message.message);
                    break;
                case Level.Warn:
                    UnityEngine.Debug.LogWarning("[RUST] WARN: " + a_message.message);
                    break;
                case Level.Info:
                    UnityEngine.Debug.Log("[RUST] INFO: " + a_message.message);
                    break;
                case Level.Debug:
                    UnityEngine.Debug.Log("[RUST] DEBUG: " + a_message.message);
                    break;
                case Level.Trace:
                    UnityEngine.Debug.Log("[RUST] TRACE: " + a_message.message);
                    break;
            }
        }
    }
}
