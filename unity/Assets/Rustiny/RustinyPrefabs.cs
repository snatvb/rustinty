using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace rustiny
{
    [System.Serializable]
    public class PrefabBindingDictionary : SerializableDictionary<string, PrefabBinding> { }

    [System.Serializable]
    public struct PrefabBinding
    {
        public GameObject Prefab;
    }

    [CreateAssetMenu(fileName = "RustinyPrefabs", menuName = "rustiny/RustinyPrefabs")]
    public class RustinyPrefabs : ScriptableObject
    {
        public PrefabBindingDictionary PrefabBindings;
    }
}
