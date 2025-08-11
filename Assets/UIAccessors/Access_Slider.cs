using UnityEngine;
using UnityEngine.UI;

[RequireComponent(typeof(Slider))]
public abstract class Access_Slider : UIElementWrapper<Slider>
{
    Slider slider;
    public float val {
        get => slider.value;
        set => slider.value = value;
    }
    public bool interactable
    {
        get => slider && slider.interactable;
        set
        {
            if (!slider) slider = GetComponent<Slider>();
            slider.interactable = value;
        }
    }

    protected override void Initialize()
    {   
        slider = GetComponent<Slider>();
        slider.onValueChanged.AddListener(OnValueChanged);
    }

    public void OnValueChanged(float inValue)
    {
        val = inValue;
    }
}
