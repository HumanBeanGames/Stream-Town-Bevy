using Reflex.Attributes;
using Reflex.Core;
using UnityEngine;

public abstract class UIGameObjectAccessor : MonoBehaviour, IInstaller
{
    [SerializeField] bool startEnabled = false;

    public void InitializeUI()
    {
        gameObject.SetActive(startEnabled);
    }

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(this, GetType());
    }

    public void SetActive(bool value) => gameObject.SetActive(value);

    public bool Enabled {
        get
        {
            return gameObject.activeInHierarchy;
        }

        set
        {
            gameObject.SetActive(value);
        }
    }

}