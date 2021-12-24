using UnityEngine;

namespace rustiny
{
    static class Converters
    {
        public static Vector3 CVecToVec(CVec3 vec)
        {
            return new Vector3(vec.x, vec.y, vec.z);
        }

        public static CVec3 VecToCVec(Vector3 vec)
        {
            return new CVec3
            {
                x = vec.x,
                y = vec.y,
                z = vec.z
            };
        }

        public static CTransform TransformToCTransform(Transform transform)
        {
            return new CTransform
            {
                position = VecToCVec(transform.position),
                rotation = new CVec3(),
                scale = VecToCVec(transform.localScale),
            };
        }

        public static void ApplyTransformFromC(Transform transform, CTransform c_transform)
        {
            // TODO: Work with rotation
            transform.position = CVecToVec(c_transform.position);
            transform.localScale = CVecToVec(c_transform.scale);
        }
    }
}
