using UnityEngine;
using UnityEngine.UI;

[RequireComponent(typeof(Toggle))]
public abstract class Access_Toggle : UIElementWrapper<Toggle>
{
    public Toggle toggle;
    public bool isOn {
        get => toggle.isOn;
        set => toggle.isOn = value;
    }
    protected override void Initialize()
    {   
        toggle = GetComponent<Toggle>();
        toggle.onValueChanged.AddListener(OnValueChanged);
    }

    public void OnValueChanged(bool inValue)
    {
        isOn = inValue;
    }
}
