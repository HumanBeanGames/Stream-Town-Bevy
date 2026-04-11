using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(StationManager))]
public class StationManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<StationManager>());
    }
}
