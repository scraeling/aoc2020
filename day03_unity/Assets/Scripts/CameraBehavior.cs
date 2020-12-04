using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraBehavior : MonoBehaviour
{
    public Transform tobi;
    public float delay;

    void Update()
    {
        var p = Vector2.Lerp(transform.position, tobi.position, delay);
        transform.position = new Vector3(p.x, p.y, transform.position.z);
        
    }
}
