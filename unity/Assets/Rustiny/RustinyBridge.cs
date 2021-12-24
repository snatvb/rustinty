using UnityEngine;
using System;

namespace rustiny
{
    public class RustinyBridge : MonoBehaviour
    {
        public RustinyApi.DSyncTransform dSubscribeUpdateTransform;

        DllLoader m_loader;
        RustinyApi.DTestCallback m_dTestCallback;
        Logger m_logger;
        [SerializeField]
        Level m_logLevel;

        void Start()
        {
            m_logger.Initialize();
            RustinyApi.Initialize();
            Debug.Log("Loaded " + RustinyApi.GetName() + " v" + RustinyApi.GetVersion());
            Debug.Log(RustinyApi.HelloWorld());
            m_dTestCallback += TestCallback;
            RustinyApi.TestCallback(m_dTestCallback);
            dSubscribeUpdateTransform += LogUpdateTransform;
            RustinyApi.SyncTransform(dSubscribeUpdateTransform);
        }

        private void Awake()
        {
            m_loader = new DllLoader(DllLoader.DEFAULT_PATH);
            m_loader.Load();
            m_logger = new rustiny.Logger();
        }

        // Update is called once per frame
        void Update()
        {
            RustinyApi.Update();
        }

        void OnDestroy()
        {
            m_loader.Destroy();
            m_loader = null;
        }

        private void LogUpdateTransform(UInt64 id, CTransform _)
        {
            Debug.Log("Updated transform " + id);
        }

        public void TestCallback(UInt64 a_identifierBits)
        {
            Debug.Log(a_identifierBits);
        }
    }
}
