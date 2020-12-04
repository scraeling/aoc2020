using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Tilemaps;

public class TobogganBoy : MonoBehaviour
{   
    bool go = false;
    private Vector3 newPos;

    public float horizontalVector;
    public float verticalVector;
    public float delay;

    public TileBase starredTree;
    public int frameDelay;
    public Tilemap groundMap;
    public Tilemap treeMap;
    

    public void Go() {
        go = true;
    }

    void Start() {
        newPos = transform.position;
    }
    void Update()
    {
        if (go && Time.frameCount % frameDelay == 0) {
            newPos = new Vector3(
                transform.position.x + horizontalVector,
                transform.position.y - verticalVector,
                transform.position.z
            ); 
            Vector3Int tilePos = treeMap.WorldToCell(transform.position);
            if (treeMap.HasTile(tilePos)) {
                treeMap.SetTile(tilePos, starredTree);
                GameObject.Find("Score").GetComponent<ScoreCounter>().Increment();
            }
        }
        transform.position = Helpers.Vec3LerpPreserveZ(transform.position, newPos, delay);
    }
}
