using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(PlayerManager))]
public class PlayerManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<PlayerManager>());
    }
}
