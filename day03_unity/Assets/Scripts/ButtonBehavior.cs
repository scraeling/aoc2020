using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class ButtonBehavior : MonoBehaviour
{
    public Button startButton;
    public InputField input;

    // Start is called before the first frame update
    void Start()
    {
        Button btn = startButton.GetComponent<Button>();
        btn.onClick.AddListener(DoOnClick);
    }

    void DoOnClick() {
        // Hide UI
        startButton.transform.Translate(0f, 100f, 0f);
        input.transform.Translate(0f, 100f, 0f);
        Debug.Log(input.text);
        GameObject.Find("TobogganBoy").GetComponent<TobogganBoy>().Go();
    }
}
