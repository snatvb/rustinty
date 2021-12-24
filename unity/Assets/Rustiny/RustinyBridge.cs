using UnityEngine;
using System;
using System.Collections.Generic;

namespace rustiny
{
    public class RustinyBridge : MonoBehaviour
    {
        private DllLoader m_loader;
        private RustinyApi.DSpawnPrefabBind m_dSpawnPrefabBind;
        private Logger m_logger;
        private Dictionary<ulong, RustinyMono> m_gameObjects = new Dictionary<ulong, RustinyMono>();
        private RustinyApi.DSyncTransform dSyncTransform;

        [SerializeField]
        private Level m_logLevel;
        [SerializeField]
        private RustinyPrefabs m_prefabs;

        private void Start()
        {
            m_logger.Initialize();
            RustinyApi.Initialize();
            Debug.LogFormat("Loaded {0} v{1}", RustinyApi.GetName(), RustinyApi.GetVersion());
            dSyncTransform += SyncTransform;
            RustinyApi.SyncTransform(dSyncTransform);
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
            foreach (KeyCode keyCode in Enum.GetValues(typeof(KeyCode)))
            {
                if (Input.GetKey(keyCode))
                {
                    RustinyApi.PushInput(keyCode);
                }
            }

            RustinyApi.Update();
        }

        private void OnDestroy()
        {
            m_loader.Destroy();
            m_loader = null;
        }

        private void SyncTransform(ulong id, CTransform c_transform)
        {
            Debug.Log("Updated transform " + id);
            if (m_gameObjects.TryGetValue(id, out RustinyMono rustinyMono))
            {
                rustinyMono.SyncTransform(c_transform);
            }
            else
            {
                Debug.LogErrorFormat("Unknown id({0}) for update transform", id);
            }
        }

        private void SpawnPrefab(ulong id, string name, CTransform c_transform)
        {
            Debug.LogFormat("Spawn prefab id({0}), with name({1})", id, name);
            if (m_prefabs.PrefabBindings.TryGetValue(name, out PrefabBinding prefabBinding))
            {
                var spawned = Instantiate(prefabBinding.Prefab);
                var rustinyMono = spawned.AddComponent<RustinyMono>();
                rustinyMono.Id = id;
                spawned.transform.hasChanged = false;
                Converters.ApplyTransformFromC(spawned.transform, c_transform);
                m_gameObjects.Add(id, rustinyMono);
            }
            else
            {
                Debug.Log("Unknown prefab to spawn");
            }
        }
    }
}
