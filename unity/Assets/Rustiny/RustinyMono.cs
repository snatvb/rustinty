using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace rustiny
{
    public class RustinyMono : MonoBehaviour
    {
        public ulong Id;

        private RustinyBridge m_bridge;


        private void Awake()
        {
            var brigdes = FindObjectsOfType<RustinyBridge>();
            Debug.Assert(brigdes.Length == 1); // should be one of RustinyBridge
            m_bridge = brigdes[0];

            m_bridge.dSubscribeUpdateTransform += SyncTransform;
        }

        private void SyncTransform(ulong id, CTransform c_transform)
        {
            if (Id == id)
            {
                Converters.ApplyTransformFromC(transform, c_transform);
                transform.hasChanged = false;
            }
        }

        private void Update()
        {
            if (transform.hasChanged)
            {
                SyncPushTranform(Id, transform);
                transform.hasChanged = false;
            }
        }

        private void SyncPushTranform(UInt64 id, Transform transform)
        {
            var c_transform = Converters.TransformToCTransform(transform);
            RustinyApi.SyncPushTransform(id, c_transform);
        }
    }
}
