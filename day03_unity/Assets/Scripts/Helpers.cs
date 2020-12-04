using System;
using UnityEngine;
public class Helpers {
    public static Vector3 Vec3LerpPreserveZ(Vector3 left, Vector3 right, float interp) {
        var p = Vector2.Lerp(left, right, interp);
        return new Vector3(p.x, p.y, left.z);
    }
}