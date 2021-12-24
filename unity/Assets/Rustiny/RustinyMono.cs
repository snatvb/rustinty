using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace rustiny
{
    public class RustinyMono : MonoBehaviour
    {
        public ulong Id;

        public void SyncTransform(CTransform c_transform)
        {
            Converters.ApplyTransformFromC(transform, c_transform);
            transform.hasChanged = false;
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
