using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;
using rustiny;
using System;

namespace rustiny
{
    [DllBindAttr("rustiny_game")]
    static class RustinyApi
    {
        [DllMethodBindAttr("rustiny_name")]
        public static _GetName GetName = null;
        public delegate string _GetName();

        [DllMethodBindAttr("rustiny_version")]
        public static _GetVersion GetVersion = null;
        public delegate string _GetVersion();

        [DllMethodBindAttr("hello_world")]
        public static _HelloWorld HelloWorld = null;
        public delegate string _HelloWorld();

        [DllMethodBindAttr("rustiny_initialize")]
        public static _Initialize Initialize = null;
        public delegate void _Initialize();

        [DllMethodBindAttr("rustiny_update")]
        public static _Update Update = null;
        public delegate void _Update();

        [DllMethodBindAttr("rustiny_callback")]
        public static _TestCallback TestCallback = null;
        public delegate string _TestCallback([MarshalAs(UnmanagedType.FunctionPtr)] DTestCallback a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DTestCallback(UInt64 a_identifierBits);

        [DllMethodBindAttr("rustiny_logger_bind")]
        public static _LoggerBind LoggerBind = null;
        public delegate string _LoggerBind([MarshalAs(UnmanagedType.FunctionPtr)] DLoggerBind a_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DLoggerBind(LogMessage a_message);

    }


    [StructLayout(LayoutKind.Sequential)]
    public struct LogMessage
    {
        public Level level;
        public string message;
    }

    [System.Serializable]
    public enum Level : int
    {
        Off = 0,
        Error = 1,
        Warn,
        Info,
        Debug,
        Trace,
    }
}
