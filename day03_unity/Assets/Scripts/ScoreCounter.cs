using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class ScoreCounter : MonoBehaviour
{
    public Text scoreboard;
    int score;
    void Start()
    {
        scoreboard.text = "0";
        score = 0;
    }

    public void Increment() {
        score += 1;
        scoreboard.text = score.ToString();
    }
}
