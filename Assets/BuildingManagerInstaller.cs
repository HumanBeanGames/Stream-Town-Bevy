using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(BuildingManager))]
public class BuildingManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton<BuildingManager>(container => GetComponent<BuildingManager>());
    }
}
