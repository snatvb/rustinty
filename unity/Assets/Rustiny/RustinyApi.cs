using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;
using rustiny;
using System;

namespace rustiny
{
    [DllBind("rustiny_game")]
    public static class RustinyApi
    {
        [DllMethodBind("rustiny_name")]
        public static _GetName GetName = null;
        public delegate string _GetName();

        [DllMethodBind("rustiny_version")]
        public static _GetVersion GetVersion = null;
        public delegate string _GetVersion();

        [DllMethodBind("rustiny_initialize")]
        public static _Initialize Initialize = null;
        public delegate void _Initialize();

        [DllMethodBind("rustiny_update")]
        public static _Update Update = null;
        public delegate void _Update();

        [DllMethodBind("rustiny_logger_bind")]
        public static _LoggerBind LoggerBind = null;
        public delegate string _LoggerBind([MarshalAs(UnmanagedType.FunctionPtr)] DLoggerBind c_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DLoggerBind(LogMessage c_message);

        [DllMethodBind("rustiny_world_sync_transform_bind")]
        public static _SyncTransform SyncTransform = null;
        public delegate void _SyncTransform([MarshalAs(UnmanagedType.FunctionPtr)] DSyncTransform c_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DSyncTransform(ulong id, CTransform c_transform);

        [DllMethodBind("rustiny_world_sync_transform_from_unity")]
        public static _SyncPushTransform SyncPushTransform = null;
        public delegate void _SyncPushTransform(ulong id, CTransform c_transform);

        [DllMethodBind("rustiny_world_spawn_prefab_bind")]
        public static _SpawnPrefabBind SpawnPrefabBind = null;
        public delegate void _SpawnPrefabBind([MarshalAs(UnmanagedType.FunctionPtr)] DSpawnPrefabBind c_callback);
        [UnmanagedFunctionPointer(CallingConvention.StdCall)] public delegate void DSpawnPrefabBind(ulong id, string name, CTransform c_transform);
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


    [StructLayout(LayoutKind.Sequential)]
    public struct CVec3
    {
        public float x;
        public float y;
        public float z;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CTransform
    {
        public CVec3 position;
        public CVec3 rotation;
        public CVec3 scale;
    }
}
