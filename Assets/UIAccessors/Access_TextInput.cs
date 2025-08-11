using TMPro;
using UnityEngine;
using UnityEngine.UI;

[RequireComponent(typeof(TMP_InputField))]
public abstract class Access_TextInput : UIElementWrapper<TMP_InputField>
{
    public TMP_InputField textField;
    public string text {
        get => textField.text;
        set => textField.text = value;
    }

    protected override void Initialize()
    {   
        textField = GetComponent<TMP_InputField>();
        textField.onValueChanged.AddListener(OnValueChanged);
    }

    public void OnValueChanged(string inValue)
    {
        text = inValue;
    }
}
