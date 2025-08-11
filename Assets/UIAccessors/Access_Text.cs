using TMPro;
using UnityEngine;

[RequireComponent(typeof(TextMeshProUGUI))]
public abstract class Access_Text : UIElementWrapper<TextMeshProUGUI>
{
    TextMeshProUGUI text;
    public string val
    {
        get => text.text;
        set => text.text = value;
    }

    protected override void Initialize()
    {
        text = GetComponent<TextMeshProUGUI>();
    }
}
