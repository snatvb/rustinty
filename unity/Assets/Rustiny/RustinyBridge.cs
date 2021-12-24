using UnityEngine;
using System;
using System.Collections.Generic;

namespace rustiny
{
    public class RustinyBridge : MonoBehaviour
    {
        public RustinyApi.DSyncTransform dSubscribeUpdateTransform;

        private DllLoader m_loader;
        private RustinyApi.DSpawnPrefabBind m_dSpawnPrefabBind;
        private Logger m_logger;
        private Dictionary<ulong, GameObject> m_gameObjects = new Dictionary<ulong, GameObject>();

        [SerializeField]
        private Level m_logLevel;
        [SerializeField]
        private RustinyPrefabs m_prefabs;

        private void Start()
        {
            m_logger.Initialize();
            RustinyApi.Initialize();
            Debug.LogFormat("Loaded {0} v{1}", RustinyApi.GetName(), RustinyApi.GetVersion());
            dSubscribeUpdateTransform += LogUpdateTransform;
            RustinyApi.SyncTransform(dSubscribeUpdateTransform);
            m_dSpawnPrefabBind += SpawnPrefab;
            RustinyApi.SpawnPrefabBind(m_dSpawnPrefabBind);
        }

        private void Awake()
        {
            m_loader = new DllLoader(DllLoader.DEFAULT_PATH);
            m_loader.Load();
            m_logger = new rustiny.Logger();
        }

        // Update is called once per frame
        private void Update()
        {
            RustinyApi.Update();
        }

        private void OnDestroy()
        {
            m_loader.Destroy();
            m_loader = null;
        }

        private void LogUpdateTransform(UInt64 id, CTransform _)
        {
            Debug.Log("Updated transform " + id);
        }

        private void SpawnPrefab(ulong id, string name, CTransform c_transform)
        {
            Debug.LogFormat("Spawn prefab id({0}), with name({1})", id, name);
            if (m_prefabs.PrefabBindings.TryGetValue(name, out PrefabBinding prefabBinding))
            {
                var spawned = Instantiate(prefabBinding.Prefab);
                var mono = spawned.AddComponent<RustinyMono>();
                mono.Id = id;
                spawned.transform.hasChanged = false;
                Converters.ApplyTransformFromC(spawned.transform, c_transform);
                m_gameObjects.Add(id, spawned);
            }
            else
            {
                Debug.Log("Unknown prefab to spawn");
            }
        }
    }
}
