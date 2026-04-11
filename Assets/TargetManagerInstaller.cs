using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(TargetManager))]
public class TargetManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<TargetManager>());
    }
}
