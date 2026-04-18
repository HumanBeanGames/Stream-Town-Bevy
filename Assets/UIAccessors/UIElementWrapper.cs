using Reflex.Core;
using UnityEngine;

public abstract class UIElementWrapper<T> : MonoBehaviour, IInstaller
{
    public virtual void InstallBindings(ContainerBuilder containerBuilder)
    {
        Initialize();
        containerBuilder.AddSingleton(this, GetType());
    }

    protected virtual void Initialize() { }
}
