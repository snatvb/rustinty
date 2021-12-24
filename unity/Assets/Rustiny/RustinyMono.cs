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
                transform.position = CVecToVec(c_transform.position);
                transform.localScale = CVecToVec(c_transform.scale);
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
            var c_transform = new CTransform
            {
                position = VecToCVec(transform.position),
                rotation = new CVec3(),
                scale = VecToCVec(transform.localScale),
            };
        RustinyApi.SyncPushTransform(id, c_transform);
        }

        private Vector3 CVecToVec(CVec3 vec) {
            return new Vector3(vec.x, vec.y, vec.z);
        }

        private CVec3 VecToCVec(Vector3 vec)
        {
            return new CVec3
            {
                x = vec.x,
                y = vec.y,
                z = vec.z
            };
        }
    }
}
