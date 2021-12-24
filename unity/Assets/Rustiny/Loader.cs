using System;
using System.Runtime.InteropServices;
using UnityEngine;

namespace rustiny
{
    static class SystemLibrary
    {
        [DllImport("kernel32", SetLastError = true, CharSet = CharSet.Unicode)]
        static public extern IntPtr LoadLibrary(string path);

        [DllImport("kernel32", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static public extern bool FreeLibrary(IntPtr dll);

        [DllImport("kernel32")]
        static public extern IntPtr GetProcAddress(IntPtr dll, string procedureName);

        //[DllImport("kernel32")]
        //static public extern IntPtr GetProcAddress(IntPtr dll, string procedureName);

        [DllImport("kernel32.dll")]
        static public extern uint GetLastError();
    }

    public class DllLoader
    {
        public static readonly string DEFAULT_PATH = Application.dataPath + "/Plugins/";
        readonly string m_path;
        readonly string m_ext;
        IntPtr m_dll_ptr;

        public DllLoader(string path, string ext = "dll")
        {
            m_path = path;
            m_ext = ext;
        }

        public void Load()
        {
            var assemblies = AppDomain.CurrentDomain.GetAssemblies();

            foreach (var assembly in assemblies)
            {
                foreach (var type in assembly.GetTypes())
                {
                    var dllTypeAttributes = type.GetCustomAttributes(typeof(DllBindAttribute), true);
                    if (dllTypeAttributes.Length == 0)
                    {
                        continue;
                    }
                    var dllBind = dllTypeAttributes[0] as DllBindAttribute;
                    var path = m_path + dllBind.DllName + "." + m_ext;

                    var dll = SystemLibrary.LoadLibrary(path);
                    if (dll == IntPtr.Zero)
                    {
                        throw new Exception("Failed to load dll: " + path + "");
                    }
                    m_dll_ptr = dll;

                    var fields = type.GetFields(System.Reflection.BindingFlags.Static | System.Reflection.BindingFlags.Public);
                    foreach (var field in fields)
                    {

                        var fieldAttributes = field.GetCustomAttributes(typeof(DllMethodBindAttribute), true);
                        if (fieldAttributes.Length == 0)
                        {
                            continue;
                        }

                        var methodPtr = SystemLibrary.GetProcAddress(dll, (fieldAttributes[0] as DllMethodBindAttribute).MethodName);
                        var methodDelegate = Marshal.GetDelegateForFunctionPointer(methodPtr, field.FieldType);
                        field.SetValue(null, methodDelegate);
                    }
                }
            }
        }

        public void Destroy()
        {
            SystemLibrary.FreeLibrary(m_dll_ptr);
        }
    }

    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false, Inherited = true)]
    public class DllBindAttribute : Attribute
    {
        public string DllName { get; private set; }

        public DllBindAttribute(string name)
        {
            DllName = name;
        }
    }

    [AttributeUsage(AttributeTargets.Field, AllowMultiple = false, Inherited = true)]
    public class DllMethodBindAttribute : Attribute
    {
        public string MethodName { get; private set; }

        public DllMethodBindAttribute(string methodName)
        {
            MethodName = methodName;
        }
    }
}

