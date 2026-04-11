using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(TownResourceManager))]
public class TownResourceManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<TownResourceManager>());
    }
}
