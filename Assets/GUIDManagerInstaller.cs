using Reflex.Core;
using UnityEngine;
using GUIDSystem;

[RequireComponent(typeof(GUIDManager))]
public class GUIDManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<GUIDManager>());
    }
}
