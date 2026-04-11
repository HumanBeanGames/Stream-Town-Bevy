using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(RoleManager))]
public class RoleManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<RoleManager>());
    }
}
