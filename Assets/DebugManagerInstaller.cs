using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(DebugManager))]
public class DebugManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<DebugManager>());
    }
}
